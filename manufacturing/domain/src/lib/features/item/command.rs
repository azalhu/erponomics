pub use super::Error;
pub use crate::proto::item::command as proto;

use std::{future::Future, sync::Arc};

use crate::{Id, Item};

use super::repository;

pub mod grpc;
pub mod service;

#[derive(Debug)]
pub struct Service<IR: repository::Create + repository::Get + repository::Delete> {
    item_repository: Arc<IR>,
}

#[derive(Debug)]
pub struct GrpcService<ICS: Create + Delete> {
    item_command_service: Arc<ICS>,
}

pub trait Create: Send + Sync + 'static {
    fn create(&self, item: Item) -> impl Future<Output = Result<(), Error>> + Send;
}

pub trait Delete: Send + Sync + 'static {
    fn delete(&self, id: Id) -> impl Future<Output = Result<(), Error>> + Send;
}
