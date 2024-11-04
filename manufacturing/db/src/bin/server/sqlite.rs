use sqlx::SqlitePool;

pub mod connection;

#[derive(Debug)]
pub struct Connection {
    pool: SqlitePool,
}
