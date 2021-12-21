pub mod dto;

pub use dto::*;

use url::Url;

use crate::Result;

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
