pub mod error;
pub mod proxy;

use crate::cli;
use crate::client::proxy::SyncerClientProxy;
use crate::context::Context;
use crate::server::{db::DatabaseProxy, service::ServerProxy};
use clap::Parser;

use anyhow;
use log::{error, trace};
use std::net::{Ipv4Addr, SocketAddrV4};

pub async fn handle_group_action(_ctx: Context, cmd: cli::GroupCommand) -> anyhow::Result<()> {
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
                Ok(group_names) => group_names.iter().for_each(|name| println!("{name}")),
                Err(_err) => {
                    error!("Request failed");
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_file_action(_ctx: Context, cmd: cli::FileCommand) -> anyhow::Result<()> {
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
