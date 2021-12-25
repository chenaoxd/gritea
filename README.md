# Gritea

`Gritea` is a async gitea API sdk.


[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/gritea.svg
[crates-url]: https://crates.io/crates/gritea
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/chenaoxd/gritea/blob/master/LICENSE


## Usage example

```rust
use anyhow::Context;
use gritea::{client::Gritea, pagination::Pagination, repo::CreateHookOption, Result};
use maplit::hashmap;
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
    println!("{:#?}", repo);

    let _repos = cli.list_repos(&Pagination::default()).await?;
    // println!("{:#?}", _repos);

    let hook = cli
        .create_hook(
            "op",
            "jarvis",
            &CreateHookOption {
                type_: "gitea".to_string(),
                config: hashmap! {
                    "url".to_string() => "http://foo.bar/hook".to_string(),
                    "content_type".to_string() => "json".to_string(),
                    "secret".to_string() => "foo".to_string(),
                },
                events: vec!["push".to_string()],
                branch_filter: "*".to_string(),
                active: true,
            },
        )
        .await?;
    println!("{:#?}", hook);

    Ok(())
}
```

## License

This project is licensed under the [MIT license](./LICENSE).
