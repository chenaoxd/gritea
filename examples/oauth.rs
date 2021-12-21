use gritea::{oauth, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let oauth2_url = oauth::oauth2_url(
        "https://git.dreamszl.cc",
        "fake_client_id",
        "https://jarvis.chenaoxd.com/v1/gitea/callback",
        "code",
        "asdf",
    )?;
    println!("{}", oauth2_url.as_str());

    Ok(())
}
