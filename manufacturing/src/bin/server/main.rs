mod config;
mod grpc;
mod sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    grpc::serve().await?;

    Ok(())
}
