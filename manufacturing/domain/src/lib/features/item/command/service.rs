pub use super::{Create, Delete, Error, Service};

use std::sync::Arc;

use crate::item::repository;

pub mod create;
pub mod delete;

impl<IR> Service<IR>
where
    IR: repository::Create + repository::Get + repository::Delete,
{
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }
}
