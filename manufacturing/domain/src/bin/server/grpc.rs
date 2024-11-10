use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use manufacturing::grpc::proto::item::command::item_command_service_server::ItemCommandServiceServer;
use manufacturing::item::command::{GrpcService as ItemGrpcService, Service as ItemService};
use manufacturing::item::repository::grpc::Client;
use manufacturing::proto::item::repository::item_command_service_client::ItemCommandServiceClient;
use manufacturing::proto::item::repository::item_query_service_client::ItemQueryServiceClient;
use tonic::transport::Server as TonicServer;

use crate::config::Config;

mod proto {
    pub const MANUFACTURING_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("manufacturing_descriptor");
}

/// # Errors
pub async fn serve() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.server_port));

    let item_db_command_client =
        Arc::new(ItemCommandServiceClient::connect(config.db_server_url.clone()).await?);
    let item_db_query_client =
        Arc::new(ItemQueryServiceClient::connect(config.db_server_url.clone()).await?);
    let item_db_client = Arc::new(Client::new(item_db_command_client, item_db_query_client));
    let item_service = Arc::new(ItemService::new(item_db_client));

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::MANUFACTURING_DESCRIPTOR_SET)
        .build_v1()?;

    TonicServer::builder()
        .add_service(reflection_service)
        .add_service(ItemCommandServiceServer::new(ItemGrpcService::new(
            item_service,
        )))
        .serve(addr)
        .await?;

    Ok(())
}
