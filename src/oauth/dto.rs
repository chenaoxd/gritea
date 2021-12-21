use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Bearer,
    Mac,
}

/// For issuing access tokens from authorization codes or refresh tokens
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenForm {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: String,
    pub refresh_token: String,
    // pub code_verifier: String,
}

/// Represents a successful access token response
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: TokenType,
    pub expires_in: i64,
    pub refresh_token: String,
    // pub id_token: String,
}
