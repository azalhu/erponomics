use crate::services::items::ItemService;
use db::grpc::items::item_server::ItemServer;
use std::net::{Ipv4Addr, SocketAddr};
use tonic::transport::Server;

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from db grpc!");
    let port = 1339u16;
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let item_service = ItemService::default();

    Server::builder()
        .add_service(ItemServer::new(item_service))
        .serve(addr)
        .await?;

    Ok(())
}
