mod cli;
mod github;
mod install;
mod shell;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let repo = matches.get_one::<String>("REPO").unwrap();
        install::install_binary(repo).await?;
    }

    Ok(())
}
