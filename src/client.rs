use http::Method;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};
use url::Url;

use crate::{
    auth::{Auth, OAuth2Token},
    config::Config,
    error::{Error, Result},
    repo::Repository,
    user::User,
};

pub struct GriteaBuilder {
    scheme: &'static str,
    host: String,
    token: Auth,
    cli: Option<Client>,
}

impl GriteaBuilder {
    /// Create a new Gitea API client builder.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            scheme: "https",
            host: host.into(),
            token: Auth::None,
            cli: None,
        }
    }

    /// Switch to an insecure protocol (http instead of https).
    pub fn insecure(&mut self) -> &mut Self {
        self.scheme = "http";
        self
    }

    /// Switch to using an Application token
    pub fn token(&mut self, token: impl Into<String>) -> &mut Self {
        self.token = Auth::Token(token.into());
        self
    }

    /// Switch to using an OAuth2 token instead of a personal access token
    pub fn oauth2_token(&mut self, oauth_token: &OAuth2Token) -> &mut Self {
        self.token = Auth::OAuth2(oauth_token.clone());
        self
    }

    pub fn build(&self) -> Result<Gritea> {
        let base_url = Url::parse(&format!("{}://{}/api/v1/", self.scheme, self.host))?;

        let cli = match &self.cli {
            Some(inner) => inner.clone(),
            None => Client::new(),
        };

        Ok(Gritea {
            conf: Arc::new(RwLock::new(Config {
                base_url,
                token: self.token.clone(),
            })),
            cli,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Gritea {
    conf: Arc<RwLock<Config>>,
    cli: Client,
}

impl Gritea {
    pub fn builder(host: impl Into<String>) -> GriteaBuilder {
        GriteaBuilder::new(host)
    }

    pub fn r_conf(&self) -> Result<Config> {
        Ok(self.conf.read()?.clone())
    }

    pub fn abs_url(&self, rel_url: &str) -> Result<Url> {
        Ok(self.r_conf()?.base_url.join(rel_url)?)
    }

    pub fn headers(&self) -> Result<(String, String)> {
        // TODO: auto refresh
        self.r_conf()?.token.headers()
    }

    pub fn request(&self, method: Method, rel_url: &str) -> Result<RequestBuilder> {
        let url = self.abs_url(rel_url)?;
        let auth_header = self.headers()?;

        Ok(self
            .cli
            .request(method, url)
            .header(auth_header.0, auth_header.1))
    }
}

// API
impl Gritea {
    // ===============================================
    // User related apis
    // ===============================================
    pub async fn current_user(&self) -> Result<User> {
        let resp = self.request(Method::GET, "user")?.send().await?;

        resp_json::<User>(resp, "get user failed").await
    }

    // ===============================================
    // Repository related apis
    // ===============================================
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let resp = self
            .request(Method::GET, &format!("repos/{}/{}", owner, repo))?
            .send()
            .await?;

        resp_json::<Repository>(resp, "get repo failed").await
    }
}

pub async fn resp_json<T>(resp: reqwest::Response, err_mes: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    if !resp.status().is_success() {
        Err(Error::GiteaError(format!(
            "{}: [{}] {}",
            err_mes,
            resp.status(),
            resp.text().await?
        )))
    } else {
        Ok(resp.json::<T>().await?)
    }
}
