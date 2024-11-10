pub use super::{Error, Get, Service};

use std::sync::Arc;

use crate::item::repository;

pub mod get;

impl<IR> Service<IR>
where
    IR: repository::Get,
{
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }
}
