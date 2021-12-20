use anyhow::Context;
use gritea::{
    client::Gritea, hook::CreateHookOption, pagination::Pagination,
    repo::CreateStatusOption, Result,
};
use maplit::hashmap;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let (owner, repo) = ("chenao", "gritea");

    let cli =
        Gritea::builder("git.dreamszl.cc")
            .token(env::var("ACCESS_TOKEN").with_context(|| {
                format!("get environment variable ACCESS_TOKEN failed")
            })?)
            .build()?;

    let user = cli.current_user().await?;
    println!("current_user: {:#?}", user);

    let repo_ = cli.get_repo(owner, repo).await?;
    println!("repo: {:#?}", repo_);

    let _repos = cli.list_repos(&Pagination::default()).await?;
    // println!("{:#?}", _repos);

    let hook = cli
        .create_hook(
            owner,
            repo,
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
    println!("create_hook: {:#?}", hook);

    let hooks = cli.list_hooks(owner, repo, &Pagination::default()).await?;
    println!("hooks: {:#?}", hooks);

    cli.delete_hook(owner, repo, hook.id).await?;

    let opt = CreateStatusOption {
        state: gritea::repo::CommitStatusState::Success,
        target_url: "http://trg_url".to_string(),
        description: "test_description".to_string(),
        context: "test_context".to_string(),
    };
    let status = cli
        .create_status(
            owner,
            repo,
            "c0a03f7fd44f9fe42a108a24e30984779e6c85b4",
            &opt,
        )
        .await?;
    println!("create_status: {:#?}", status);

    Ok(())
}
