mod config;
mod grpc;
mod sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    grpc::serve().await?;

    Ok(())
}
