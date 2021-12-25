use reqwest::Client;
use url::Url;

use crate::{
    auth::Auth, client::Gritea, config::Config, error::Result, oauth::AccessToken,
};

pub struct GriteaBuilder {
    scheme: String,
    host: String,
    token: Auth,
    cli: Option<Client>,
}

impl GriteaBuilder {
    /// Create a new Gitea API client builder.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            scheme: "https".to_string(),
            host: host.into(),
            token: Auth::None,
            cli: None,
        }
    }

    /// Switch to an insecure protocol (http instead of https).
    pub fn insecure(&mut self) -> &mut Self {
        self.scheme = "http".to_string();
        self
    }

    /// Set the scheme of the Gitea server
    pub fn scheme(&mut self, scheme: impl Into<String>) -> &mut Self {
        self.scheme = scheme.into();
        self
    }

    /// Switch to using an Application token
    pub fn token(&mut self, token: impl Into<String>) -> &mut Self {
        self.token = Auth::Token(token.into());
        self
    }

    /// Switch to using an OAuth2 token instead of a personal access token
    pub fn oauth2_token(&mut self, oauth_token: &AccessToken) -> &mut Self {
        self.token = Auth::OAuth2(oauth_token.clone());
        self
    }

    /// Use the specified reqwest client, avoid to establish new http connections
    pub fn cli(&mut self, cli: Client) -> &mut Self {
        self.cli = Some(cli);
        self
    }

    pub fn build(&self) -> Result<Gritea> {
        let base_url = Url::parse(&format!("{}://{}/", self.scheme, self.host))?;

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
