mod cli;
mod client;
mod context;
mod env;
mod logging;
mod server;
mod util;

use anyhow;
use clap::Parser;

use context::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let _ = logging::init();

    let app_dirs = env::AppDirectories::new();
    let ctx = Context::new(app_dirs, None);

    match cli.command {
        cli::Command::Group(subcmd) => client::handle_group_action(ctx, subcmd.command).await?,
        cli::Command::File(subcmd) => client::handle_file_action(ctx, subcmd.command).await?,
        cli::Command::Server(subcmd) => server::handle_server_action(ctx, subcmd.command).await?,
    };

    Ok(())
}
