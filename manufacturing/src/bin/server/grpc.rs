use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use manufacturing::grpc::item::Service as GrpcItemService;
use manufacturing::grpc::proto::google::longrunning::operations_server::OperationsServer as GoogleOperationsServer;
use manufacturing::grpc::sync::Service as GrpcSyncService;
use manufacturing::item::command::Service as ItemCommandService;
use manufacturing::item::query::Service as ItemQueryService;
use manufacturing::item::repository::Service as ItemRepositoryService;
use manufacturing::proto::item_service_server::ItemServiceServer;
use tonic::transport::Server as TonicServer;

use crate::config::Config;
use crate::sqlite::Connection;

mod proto {
    pub const MANUFACTURING_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("manufacturing_descriptor");
}

/// # Errors
pub async fn serve(config: &Config) -> anyhow::Result<()> {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.server_port));

    let sqlite_connection = Arc::new(Connection::new(&config.database_url).await?);

    // MARK: Item
    let item_repository = Arc::new(ItemRepositoryService::new(sqlite_connection));
    let item_command_service = Arc::new(ItemCommandService::new(item_repository.clone()));
    let item_query_service = Arc::new(ItemQueryService::new(item_repository.clone()));
    let grpc_item_service = GrpcItemService::new(item_command_service, item_query_service);

    // MARK: Sync
    let grpc_sync_service = GrpcSyncService;

    // MARK: Reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::MANUFACTURING_DESCRIPTOR_SET)
        .build_v1()?;

    TonicServer::builder()
        .add_service(reflection_service)
        .add_service(ItemServiceServer::new(grpc_item_service))
        .add_service(GoogleOperationsServer::new(grpc_sync_service))
        .serve(addr)
        .await?;

    Ok(())
}
