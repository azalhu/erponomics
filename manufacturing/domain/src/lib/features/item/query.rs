pub use super::Error;
pub use crate::proto::item::query as proto;

use std::{future::Future, sync::Arc};

use crate::{Id, Item};

use super::repository;

pub mod grpc;
pub mod service;

#[derive(Debug)]
pub struct Service<IR: repository::Get> {
    item_repository: Arc<IR>,
}

pub struct GrpcService<IQS: Get> {
    item_query_service: Arc<IQS>,
}

pub trait Get: Send + Sync + 'static {
    fn get(&self, id: Id) -> impl Future<Output = Result<Item, Error>> + Send;
}
