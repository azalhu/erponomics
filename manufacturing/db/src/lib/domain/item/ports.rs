use std::future::Future;

#[allow(unused_imports)]
use crate::domain::item::models::item::ItemNumber;
use crate::domain::item::models::item::{CreateItemError, CreateItemRequest, Item};

/// `ItemRepository` represents a store of item data.
pub trait ItemRepository: Send + Sync + 'static {
    /// Persist a new [Item].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateItemError::Duplicate] if an [Item] with the same [ItemNumber] already exists.
    fn create_item(
        &self,
        req: &CreateItemRequest,
    ) -> impl Future<Output = Result<Item, CreateItemError>> + Send;
}
