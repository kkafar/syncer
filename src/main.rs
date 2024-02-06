mod cli;
mod client;
mod context;
mod env;
mod logging;
mod server;

use clap::Parser;
use client::SyncerClientProxy;
use context::Context;
use server::{db::DatabaseProxy, ServerProxy};

use anyhow;
use log::{error, info, trace};
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::client::client_stub::{AddFileRequest, ListFilesRequest, RemoveFileRequest};

async fn handle_server_action(mut ctx: Context, cmd: cli::ServerCommand) -> anyhow::Result<()> {
    // When running server we have to make sure that the database exists
    let mut db_proxy = DatabaseProxy::new(ctx.app_dirs.get_data_dir().join("server.db3"))?;
    db_proxy.ensure_tables_exist();
    ctx.db = Some(db_proxy);

    match cmd {
        cli::ServerCommand::Start => {
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

async fn handle_client_action(ctx: Context, cmd: cli::FileCommand) -> anyhow::Result<()> {
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
            let request = tonic::Request::new(AddFileRequest {
                file_path: file.to_str().unwrap().to_string(),
            });
            let response = client_proxy.client.add_file(request).await?;
            info!("Server response: {response:?}");
        }
        cli::FileCommand::Remove { file } => {
            trace!("Running FileRemove action");
            let request = tonic::Request::new(RemoveFileRequest {
                file_path: file.to_str().unwrap().to_string(),
            });
            let response = client_proxy.client.remove_file(request).await?;
            info!("Server response: {response:?}");
        }
        cli::FileCommand::List => {
            trace!("Running FileList action");
            let request = tonic::Request::new(ListFilesRequest {
                request: "This is request content".into(),
            });
            let response = client_proxy.client.list_files(request).await?;
            info!("Server response: {response:?}");
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let content = include_str!("../data/input01.txt");
    //
    // println!("{content}");
    // let digest = md5::compute(content.as_bytes());
    // println!("{digest:?}");
    //
    // let Ok(binary_content) = fs::read(path::Path::new("data/input01.txt")) else {
    //     return;
    // };
    //
    // println!("{binary_content:?}");
    // let binary_digest = md5::compute(binary_content);
    // println!("{binary_digest:?}");
    //

    let cli = cli::Cli::parse();
    let _ = logging::init();

    let app_dirs = env::AppDirectories::new();
    let ctx = Context::new(app_dirs, None);

    match cli.command {
        cli::Command::File(subcmd) => handle_client_action(ctx, subcmd.command).await?,
        cli::Command::Server(subcmd) => handle_server_action(ctx, subcmd.command).await?,
    };

    Ok(())
}
