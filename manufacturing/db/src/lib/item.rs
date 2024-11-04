use std::sync::Arc;

use derive_getters::Getters;
use manufacturing::item::repository::{Create, Delete, Get};

use crate::sqlx::sqlite::Connection;

pub mod grpc_service;
pub mod sqlite_client;

#[derive(Debug)]
pub struct GrpcService<IR: Create + Get + Delete> {
    item_repository: Arc<IR>,
}

#[derive(Debug, Getters)]
pub struct SqliteClient<DB: Connection> {
    db: Arc<DB>,
}
