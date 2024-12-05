pub use super::Error;

use std::{future::Future, sync::Arc};

use crate::Item;

use super::repository;

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

pub struct ListResponse {
    items: Vec<Item>,
    page_token: Option<String>,
    total_size: i32,
}

impl ListResponse {
    #[must_use]
    pub const fn new(items: Vec<Item>, page_token: Option<String>, total_size: i32) -> Self {
        Self {
            items,
            page_token,
            total_size,
        }
    }
}

// MARK: Service

#[derive(Debug)]
pub struct Service<IR: repository::Get + repository::List> {
    item_repository: Arc<IR>,
}

impl<IR> Service<IR>
where
    IR: repository::Get + repository::List,
{
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }
}

impl<IR> Get for Service<IR>
where
    IR: repository::Get + repository::List,
{
    async fn get(&self, _request: GetRequest) -> Result<Item, Error> {
        todo!()
        //        let request = request.into();
        //        let item = self.item_repository.get(request).await?;
        //
        //        Ok(item)
    }
}

impl<IR> List for Service<IR>
where
    IR: repository::Get + repository::List,
{
    async fn list(&self, _request: ListRequest) -> Result<ListResponse, Error> {
        todo!()
        //        let request = request.into();
        //        let items = self.item_repository.list(request).await?;
        //
        //        Ok(item)
    }
}
