mod cli;
mod env;
mod logging;
mod server;
mod client;

use clap::Parser;
use client::SyncerClientProxy;
use server::ServerProxy;
use core::panic;
use std::net::{Ipv4Addr, SocketAddrV4};
use log::trace;
use anyhow::{self, Ok};

use crate::client::client_stub::ListFilesRequest;


async fn handle_server_action(cmd: cli::ServerCommand) -> anyhow::Result<()> {
    match cmd {
        cli::ServerCommand::Start => {
            ServerProxy::new(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)).run().await?;
        }
        cli::ServerCommand::Stop => {
            trace!("Running ServerStop action");
        }
    };
    Ok(())
}

async fn handle_client_action(cmd: cli::FileCommand) -> anyhow::Result<()> {
    let mut client_proxy = SyncerClientProxy::new("http://127.0.0.1:8080".into()).await?;

    match cmd {
        cli::FileCommand::Add { file: _ } => {
            trace!("Running FileAdd action");
        }
        cli::FileCommand::Remove { file: _ } => {
            trace!("Running FileRemove action");
        }
        cli::FileCommand::List => {
            trace!("Running FileList action");
            let request = tonic::Request::new(ListFilesRequest {
                request: "This is request content".into(),
            });
            let response = client_proxy.client.list_files(request).await?;
            println!("Server response: {response:?}");
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

    let _xdg_dirs = env::ensure_file_structure_exists();

    match cli.command {
        cli::Command::File(subcmd) => handle_client_action(subcmd.command).await?,
        cli::Command::Server(subcmd) => handle_server_action(subcmd.command).await?,
    };

    Ok(())
}
