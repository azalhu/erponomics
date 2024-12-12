use std::{future::Future, sync::Arc};

use derive_getters::{Dissolve, Getters};

use crate::{
    item::{repository, Error},
    Id, Item,
};

// MARK: Get

pub trait Get: Send + Sync + 'static {
    fn get(&self, request: GetRequest) -> impl Future<Output = Result<Item, Error>> + Send;
}

pub struct GetRequest {
    id: String,
}

impl GetRequest {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self { id }
    }
}

// MARK: List

pub trait List: Send + Sync + 'static {
    fn list(
        &self,
        request: ListRequest,
    ) -> impl Future<Output = Result<ListResponse, Error>> + Send;
}

#[derive(Getters)]
pub struct ListRequest {
    page_size: i32,
    page_token: Option<String>,
    order_by: Option<String>,
    filter: Option<String>,
}

impl ListRequest {
    #[must_use]
    pub fn new(
        page_size: Option<i32>,
        page_token: Option<String>,
        order_by: Option<String>,
        filter: Option<String>,
    ) -> Self {
        Self {
            page_size: page_size.unwrap_or(50).min(1000),
            page_token,
            order_by,
            filter,
        }
    }
}

#[derive(Dissolve)]
pub struct ListResponse {
    items: Vec<Item>,
    next_page_token: Option<String>,
    total_size: i32,
}

impl ListResponse {
    #[must_use]
    pub const fn new(items: Vec<Item>, next_page_token: Option<String>, total_size: i32) -> Self {
        Self {
            items,
            next_page_token,
            total_size,
        }
    }
}

// MARK: Service

#[derive(Debug, Clone)]
pub struct Service<IR: repository::Get + repository::List + Clone> {
    item_repository: Arc<IR>,
}

impl<IR> Service<IR>
where
    IR: repository::Get + repository::List + Clone,
{
    #[must_use]
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }
}

fn validate_get_request(request: GetRequest) -> Result<Id, Error> {
    let id: Id = request.id.try_into()?;

    Ok(id)
}

impl<IR> Get for Service<IR>
where
    IR: repository::Get + repository::List + Clone,
{
    async fn get(&self, request: GetRequest) -> Result<Item, Error> {
        let id = validate_get_request(request)?;

        self.item_repository.get(&id).await
    }
}

impl<IR> List for Service<IR>
where
    IR: repository::Get + repository::List + Clone,
{
    async fn list(&self, request: ListRequest) -> Result<ListResponse, Error> {
        self.item_repository.list(&request).await
    }
}
