mod cli;
mod client;
mod context;
mod env;
mod logging;
mod server;
mod util;

use clap::Parser;
use client::SyncerClientProxy;
use context::Context;
use server::{db::DatabaseProxy, ServerProxy};

use anyhow;
use log::{error, info, trace};
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::client::client_stub::{AddFileRequest, ListFilesRequest, RemoveFileRequest};

async fn handle_group_action(mut ctx: Context, cmd: cli::GroupCommand) -> anyhow::Result<()> {
    let mut client_proxy = match SyncerClientProxy::new("http://127.0.0.1:8080".into()).await {
        Ok(client_proxy) => client_proxy,
        Err(err) => {
            error!("Creating connect to server failed with error: ${err:?}");
            return Err(err);
        }
    };

    match cmd {
        cli::GroupCommand::Add { name, prefix } => {
            trace!("Running GroupAdd action");
            client_proxy.add_group(name, prefix).await?;
        }
        cli::GroupCommand::Remove { name } => {
            trace!("Running GroupRemove action");
            client_proxy.remove_group(name).await?;
        }
        cli::GroupCommand::List => {
            trace!("Running GroupList action");
            let result = client_proxy.list_groups().await;
            match result {
                Ok(group_names) => {
                    group_names.iter().for_each(
                        |name| println!("{name}")
                    )
                }
                Err(err) => {
                    error!("Request failed");
                }
            }
        }
    }

    Ok(())
}

async fn handle_server_action(mut ctx: Context, cmd: cli::ServerCommand) -> anyhow::Result<()> {
    // When running server we have to make sure that the database exists
    let mut db_proxy = DatabaseProxy::new(ctx.app_dirs.get_data_dir().join("server.db3"))?;
    db_proxy.ensure_tables_exist();
    ctx.inject_db(db_proxy);

    match cmd {
        cli::ServerCommand::Start => {
            trace!("Running ServerStart action");
            ServerProxy::new(ctx, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
                .run()
                .await?;
        }
        cli::ServerCommand::Stop => {
            trace!("Running ServerStop action");
        }
    };
    Ok(())
}

async fn handle_file_action(_ctx: Context, cmd: cli::FileCommand) -> anyhow::Result<()> {
    let mut client_proxy = match SyncerClientProxy::new("http://127.0.0.1:8080".into()).await {
        Ok(client_proxy) => client_proxy,
        Err(err) => {
            error!("Creating connect to server failed with error: ${err:?}");
            return Err(err);
        }
    };

    match cmd {
        cli::FileCommand::Add { file } => {
            trace!("Running FileAdd action");
            let _result = client_proxy.add_file(file).await?;
        }
        cli::FileCommand::Remove { file } => {
            trace!("Running FileRemove action");
            let _result = client_proxy.remove_file(file);
        }
        cli::FileCommand::List => {
            trace!("Running FileList action");
            let _result = client_proxy.list_files(None);
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let _ = logging::init();

    let app_dirs = env::AppDirectories::new();
    let ctx = Context::new(app_dirs, None);

    match cli.command {
        cli::Command::Group(subcmd) => handle_group_action(ctx, subcmd.command).await?,
        cli::Command::File(subcmd) => handle_file_action(ctx, subcmd.command).await?,
        cli::Command::Server(subcmd) => handle_server_action(ctx, subcmd.command).await?,
    };

    Ok(())
}
