use gritea::{client::Gritea, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("client example");

    let cli = Gritea::builder("git.dreamszl.cc")
        .token("access_token")
        .build()?;
    let user = cli.current_user().await?;

    println!("{:#?}", user);

    Ok(())
}
