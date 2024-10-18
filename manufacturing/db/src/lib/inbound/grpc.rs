use std::sync::Arc;

use crate::domain::item::ports::ItemRepository;

pub mod item {
    tonic::include_proto!("item");
}

pub mod services;

#[derive(Debug, Clone)]
struct AppState<IR: ItemRepository> {
    item_repository: Arc<IR>,
}
