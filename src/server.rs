pub mod db;
pub mod service;

use anyhow;
use log::trace;
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::cli;
use crate::context::Context;
use crate::server::db::DatabaseProxy;

pub async fn handle_server_action(ctx: Context, cmd: cli::ServerCommand) -> anyhow::Result<()> {
    // When running server we have to make sure that the database exists
    let mut db_proxy = DatabaseProxy::new(ctx.app_dirs.get_data_dir().join("server.db3"))?;
    db_proxy.ensure_tables_exist();
    ctx.inject_db(db_proxy);

    match cmd {
        cli::ServerCommand::Start => {
            trace!("Running ServerStart action");
            service::ServerProxy::new(ctx, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
                .run()
                .await?;
        }
        cli::ServerCommand::Stop => {
            trace!("Running ServerStop action");
        }
    };
    Ok(())
}
