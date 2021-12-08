use reqwest::Client;
use url::Url;

use crate::{
    auth::{Auth, OAuth2Token},
    client::Gritea,
    config::Config,
    error::Result,
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

        Ok(Gritea::new(
            Config {
                base_url,
                token: self.token.clone(),
            },
            cli,
        ))
    }
}
