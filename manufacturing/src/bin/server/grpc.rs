use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use manufacturing::item::command::Service as ItemCommandService;
use manufacturing::item::grpc::Service as ItemGrpcService;
use manufacturing::item::query::Service as ItemQueryService;
use manufacturing::item::repository::ItemRepository;
use manufacturing::proto::item::item_command_service_server::ItemCommandServiceServer;
use tonic::transport::Server as TonicServer;

use crate::config::Config;
use crate::sqlite::Connection;

mod proto {
    pub const MANUFACTURING_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("manufacturing_descriptor");
}

/// # Errors
pub async fn serve() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.server_port));

    let sqlite_connection = Arc::new(Connection::new(&config.database_url).await?);
    let item_repository = Arc::new(ItemRepository::new(sqlite_connection));
    let item_command_service = Arc::new(ItemCommandService::new(item_repository.clone()));
    let item_query_service = Arc::new(ItemQueryService::new(item_repository.clone()));
    let item_grpc_service = ItemGrpcService::new(item_command_service, item_query_service);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::MANUFACTURING_DESCRIPTOR_SET)
        .build_v1()?;

    TonicServer::builder()
        .add_service(reflection_service)
        .add_service(ItemCommandServiceServer::new(item_grpc_service))
        .serve(addr)
        .await?;

    Ok(())
}
