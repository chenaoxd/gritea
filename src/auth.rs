use chrono::{DateTime, Utc};

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct OAuth2Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: DateTime<Utc>,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub enum Auth {
    Token(String),
    OAuth2(OAuth2Token),
    None,
}

impl Auth {
    pub fn headers(&self) -> Result<(String, String)> {
        match &self {
            &Auth::Token(token) => Ok((
                http::header::AUTHORIZATION.to_string(),
                format!("token {}", token),
            )),
            &Auth::OAuth2(oauth_token) => Ok((
                http::header::AUTHORIZATION.to_string(),
                format!("{} {}", oauth_token.token_type, oauth_token.access_token),
            )),
            &Auth::None => Err(Error::Unauthorized("client token not set".to_string())),
        }
    }
}
