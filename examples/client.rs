use anyhow::Context;
use gritea::{client::Gritea, pagination::Pagination, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let cli =
        Gritea::builder("git.dreamszl.cc")
            .token(env::var("ACCESS_TOKEN").with_context(|| {
                format!("get environment variable ACCESS_TOKEN failed")
            })?)
            .build()?;

    let user = cli.current_user().await?;
    println!("{:#?}", user);

    let repo = cli.get_repo("op", "jarvis").await?;
    println!("{}", serde_json::to_string_pretty(&repo)?);

    let repos = cli.list_repos(Pagination::default()).await?;
    println!("{}", serde_json::to_string_pretty(&repos)?);

    Ok(())
}
