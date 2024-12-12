use std::str::FromStr;

use anyhow::Context;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

#[derive(Debug, Clone)]
pub struct Connection {
    pool: SqlitePool,
}

impl Connection {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        let connect_options = SqliteConnectOptions::from_str(path)
            .with_context(|| format!("invalid database path {path}"))?
            .pragma("foreign_keys", "on")
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(connect_options)
            .await
            .with_context(|| format!("failed to open database at {path}"))?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}

impl manufacturing::sqlx::SqliteConnection for Connection {
    fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
