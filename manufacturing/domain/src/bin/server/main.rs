#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod config;
mod db;
mod grpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    grpc::serve().await?;

    Ok(())
}
