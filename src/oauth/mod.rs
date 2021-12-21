pub mod dto;

pub use dto::*;

use http::Method;
use reqwest::Client;
use url::Url;

use crate::{client::resp_json, Result};

pub fn oauth2_url(
    base_url: &str,
    client_id: &str,
    redirect_uri: &str,
    response_type: &str,
    state: &str,
) -> Result<Url> {
    let mut url = Url::parse(base_url)?.join("login/oauth/authorize")?;
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", response_type)
        .append_pair("state", state);

    Ok(url)
}

pub async fn access_token(
    base_url: &str,
    ac_form: AccessTokenForm,
    http_cli: Client,
) -> Result<AccessTokenResponse> {
    let url = Url::parse(base_url)?.join("login/oauth/access_token")?;

    let resp = http_cli
        .request(Method::POST, url)
        .json(&ac_form)
        .send()
        .await?;

    resp_json(resp, "get access_token failed").await
}
