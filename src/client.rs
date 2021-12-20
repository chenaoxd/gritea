use http::Method;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};
use url::Url;

use crate::{
    builder::GriteaBuilder,
    config::Config,
    error::{Error, Result},
    hook::{CreateHookOption, Hook},
    pagination::Pagination,
    repo::{CommitStatus, CreateStatusOption, Repository},
    user::User,
};

#[derive(Debug, Clone)]
pub struct Gritea {
    conf: Arc<RwLock<Config>>,
    cli: Client,
}

impl Gritea {
    pub fn builder(host: impl Into<String>) -> GriteaBuilder {
        GriteaBuilder::new(host)
    }

    pub fn new(conf: Config, cli: Client) -> Self {
        Self {
            conf: Arc::new(RwLock::new(conf)),
            cli,
        }
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

    /// Get the user who owns the auth_token
    pub async fn current_user(&self) -> Result<User> {
        let resp = self.request(Method::GET, "user")?.send().await?;

        resp_json(resp, "get user failed").await
    }

    /// List all the repos which the user has permission to
    pub async fn list_repos(&self, page: &Pagination) -> Result<Vec<Repository>> {
        let resp = self
            .request(Method::GET, "user/repos")?
            .query(&page.to_query())
            .send()
            .await?;

        resp_json(resp, "list repos failed").await
    }

    // ===============================================
    // Repository related apis
    // ===============================================

    /// Get the specified repo
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let resp = self
            .request(Method::GET, &format!("repos/{}/{}", owner, repo))?
            .send()
            .await?;

        resp_json(resp, "get repo failed").await
    }

    /// Create a commit status
    pub async fn create_status(
        &self,
        owner: &str,
        repo: &str,
        commit: &str,
        option: &CreateStatusOption,
    ) -> Result<CommitStatus> {
        let resp = self
            .request(
                Method::POST,
                &format!("repos/{}/{}/statuses/{}", owner, repo, commit),
            )?
            .json(option)
            .send()
            .await?;

        resp_json(resp, "create commit status failed").await
    }

    /// Create a webhook
    pub async fn create_hook(
        &self,
        owner: &str,
        repo: &str,
        opt: &CreateHookOption,
    ) -> Result<Hook> {
        let resp = self
            .request(Method::POST, &format!("repos/{}/{}/hooks", owner, repo))?
            .json(opt)
            .send()
            .await?;

        resp_json(resp, "create hook failed").await
    }

    /// Delete a webhook
    pub async fn delete_hook(&self, owner: &str, repo: &str, id: i64) -> Result<()> {
        let resp = self
            .request(
                Method::DELETE,
                &format!("repos/{}/{}/hooks/{}", owner, &repo, id),
            )?
            .send()
            .await?;

        check_success(resp, "delete hook failed").await
    }

    /// List webhooks of a repo
    pub async fn list_hooks(
        &self,
        owner: &str,
        repo: &str,
        page: &Pagination,
    ) -> Result<Vec<Hook>> {
        let resp = self
            .request(Method::GET, &format!("repos/{}/{}/hooks", owner, repo))?
            .query(&page.to_query())
            .send()
            .await?;

        resp_json(
            resp,
            &format!("list hooks of repo {}/{} failed", owner, repo),
        )
        .await
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

pub async fn check_success(resp: reqwest::Response, err_mes: &str) -> Result<()> {
    if !resp.status().is_success() {
        Err(Error::GiteaError(format!(
            "{}: [{}] {}",
            err_mes,
            resp.status(),
            resp.text().await?
        )))
    } else {
        Ok(())
    }
}
