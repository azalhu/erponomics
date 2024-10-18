use std::net::{Ipv4Addr, SocketAddr};

use db::{config::Config, outbound::sqlite::Sqlite};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello from db grpc!");
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let _post_repo = Sqlite::new(&config.database_url).await?;

    let _port = 1339u16;
    let _addr = SocketAddr::from((Ipv4Addr::LOCALHOST, _port));
    //    let item_service = ItemService::default();
    //
    //    Server::builder()
    //        .add_service(ItemServer::new(item_service))
    //        .serve(addr)
    //        .await?;

    Ok(())
}
