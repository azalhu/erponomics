use config::Config;

mod config;
mod grpc;
mod sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    grpc::serve(&config).await?;

    #[allow(clippy::expect_used)]
    Ok(())
}
