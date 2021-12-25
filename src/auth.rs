use crate::{
    error::{Error, Result},
    oauth::AccessToken,
};

#[derive(Debug, Clone)]
pub enum Auth {
    Token(String),
    OAuth2(AccessToken),
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
                format!("{:#?} {}", oauth_token.token_type, oauth_token.access_token),
            )),
            &Auth::None => Err(Error::Unauthorized("client token not set".to_string())),
        }
    }
}
