pub mod error;
pub mod proxy;

use std::path::PathBuf;

use crate::cli;
use crate::client::proxy::SyncerClientProxy;
use crate::context::Context;

use anyhow;
use log::{error, trace};

pub async fn handle_group_action(ctx: Context, cmd: cli::GroupCommand) -> anyhow::Result<()> {
    let client_proxy = match SyncerClientProxy::new("http://127.0.0.1:8080".into()).await {
        Ok(client_proxy) => client_proxy,
        Err(err) => {
            error!("Creating connect to server failed with error: ${err:?}");
            return Err(err);
        }
    };

    match cmd {
        cli::GroupCommand::Add { name, prefix } => {
            handle_group_add_action(ctx, client_proxy, name, prefix).await?
        }
        cli::GroupCommand::Remove { name } => {
            handle_group_remove_action(ctx, client_proxy, name).await?
        }
        cli::GroupCommand::List => handle_group_list_action(ctx, client_proxy).await?,
    }

    Ok(())
}

pub async fn handle_file_action(ctx: Context, cmd: cli::FileCommand) -> anyhow::Result<()> {
    let client_proxy = match SyncerClientProxy::new("http://127.0.0.1:8080".into()).await {
        Ok(client_proxy) => client_proxy,
        Err(err) => {
            error!("Creating connect to server failed with error: ${err:?}");
            return Err(err);
        }
    };

    match cmd {
        cli::FileCommand::Add { file } => handle_file_add_action(ctx, client_proxy, file).await?,
        cli::FileCommand::Remove { file } => {
            handle_file_remove_action(ctx, client_proxy, file).await?
        }
        cli::FileCommand::List => handle_file_list_action(ctx, client_proxy).await?,
    };
    Ok(())
}

async fn handle_group_add_action(
    _ctx: Context,
    mut client: SyncerClientProxy,
    name: String,
    prefix: PathBuf,
) -> anyhow::Result<()> {
    trace!("Running GroupAdd action");
    Ok(client.add_group(name, prefix).await?)
}

async fn handle_group_remove_action(
    _ctx: Context,
    mut client: SyncerClientProxy,
    name: String,
) -> anyhow::Result<()> {
    trace!("Running GroupRemove action");
    Ok(client.remove_group(name).await?)
}

async fn handle_group_list_action(
    _ctx: Context,
    mut client: SyncerClientProxy,
) -> anyhow::Result<()> {
    trace!("Running GroupList action");
    let result = client.list_groups().await;
    match result {
        Ok(group_names) => group_names.iter().for_each(|name| println!("{name}")),
        Err(_err) => {
            error!("Request failed");
        }
    }
    Ok(())
}

async fn handle_file_add_action(
    _ctx: Context,
    mut client: SyncerClientProxy,
    file: PathBuf,
) -> anyhow::Result<()> {
    trace!("Running FileAdd action");
    Ok(client.add_file(file).await?)
}

async fn handle_file_remove_action(
    _ctx: Context,
    mut client: SyncerClientProxy,
    file: PathBuf,
) -> anyhow::Result<()> {
    trace!("Running FileRemove action");
    Ok(client.remove_file(file).await?)
}

async fn handle_file_list_action(
    _ctx: Context,
    mut client: SyncerClientProxy,
) -> anyhow::Result<()> {
    trace!("Running FileList action");
    Ok(client.list_files(None).await?)
}
