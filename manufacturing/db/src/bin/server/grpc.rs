use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use tonic::transport::Server as TonicServer;

use crate::config::Config;
use crate::sqlite::Connection;

/// # Errors
pub async fn serve() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let sqlite_client = Arc::new(Connection::new(&config.database_url).await?);
    let item_service = db::grpc::item_service(sqlite_client);

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.server_port));

    TonicServer::builder()
        .add_service(item_service)
        .serve(addr)
        .await?;

    Ok(())
}
