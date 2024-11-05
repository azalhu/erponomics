use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use tonic::transport::Server as TonicServer;

use crate::config::Config;
use crate::sqlite::Connection;

/// # Errors
pub async fn serve() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let sqlite_connection = Arc::new(Connection::new(&config.database_url).await?);
    let item_sqlite_client = db::grpc::item::sqlite_client(sqlite_connection);
    let item_command_service = db::grpc::item::command_service(item_sqlite_client.clone());
    let item_query_service = db::grpc::item::query_service(item_sqlite_client.clone());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.server_port));

    TonicServer::builder()
        .add_service(item_command_service)
        .add_service(item_query_service)
        .serve(addr)
        .await?;

    Ok(())
}
