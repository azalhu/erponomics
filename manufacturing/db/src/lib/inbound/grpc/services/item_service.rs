use std::{sync::Arc, time::SystemTime};

use crate::domain::item::models::item as domain;
use crate::domain::item::ports;
use crate::inbound::grpc::item::{item_server::Item, ItemRequest, ItemResponse};
use prost_types::Timestamp;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ItemService<IR: ports::ItemRepository> {
    item_repository: Arc<IR>,
}

impl<IR> ItemService<IR>
where
    IR: ports::ItemRepository,
{
    fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }
}

#[tonic::async_trait]
impl<IR> Item for ItemService<IR>
where
    IR: ports::ItemRepository,
{
    async fn create_item(
        &self,
        request: Request<ItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        println!("Got a create request: {:?}", request);

        let request = request.try_into()?;

        self.item_repository
            .create_item(&request)
            .await
            .map_err(Status::from)
            .map(domain::Item::into)
    }

    async fn get_item(
        &self,
        request: Request<ItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        let response = Response::new(ItemResponse {
            id: "1".to_string(),
            number: req.number,
            name: req.name,
            created_at: Some(Timestamp::from(SystemTime::now())),
        });

        Ok(response)
    }
}

impl TryFrom<Request<ItemRequest>> for domain::CreateItemRequest {
    type Error = domain::CreateItemError;

    fn try_from(req: Request<ItemRequest>) -> Result<domain::CreateItemRequest, Self::Error> {
        let req = req.into_inner();

        let item_number = domain::ItemNumber::new(&req.number)?;
        let item_name = domain::ItemName::new(&req.name)?;

        Ok(domain::CreateItemRequest::new(item_number, item_name))
    }
}

impl From<domain::CreateItemError> for Status {
    fn from(create_item_error: domain::CreateItemError) -> Self {
        let error_message = create_item_error.to_string();
        match create_item_error {
            domain::CreateItemError::Duplicate { .. }
            | domain::CreateItemError::EmptyNumber(..)
            | domain::CreateItemError::EmptyName(..) => Status::invalid_argument(error_message),
            domain::CreateItemError::Unknown(..) => Status::unknown(error_message),
        }
    }
}

impl From<domain::Item> for Response<ItemResponse> {
    fn from(item: domain::Item) -> Self {
        Self::new(ItemResponse {
            id: item.id().to_string(),
            number: item.number().to_string(),
            name: item.name().to_string(),
            created_at: Some(item.created_at().clone().into()),
        })
    }
}

impl From<domain::DateTimeUtc> for Timestamp {
    fn from(value: domain::DateTimeUtc) -> Timestamp {
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ItemRepositoryMock;

    impl ports::ItemRepository for ItemRepositoryMock {
        async fn create_item(
            &self,
            _req: &domain::CreateItemRequest,
        ) -> Result<domain::Item, domain::CreateItemError> {
            todo!()
        }
    }

    #[tokio::test]
    async fn create_item_returns_ok_response() -> Result<(), Status> {
        let item_service = ItemService::new(Arc::new(ItemRepositoryMock {}));
        let create_item_req = ItemRequest {
            number: "Hello".to_owned(),
            name: "MEME".to_owned(),
        };

        let response = item_service
            .create_item(Request::new(create_item_req))
            .await?
            .into_inner();

        assert_eq!(response.id, "0");
        assert_eq!(response.number, "Hello");
        assert_eq!(response.name, "MEME");

        Ok(())
    }

    #[tokio::test]
    async fn get_item_returns_ok_response() -> Result<(), Status> {
        let item_service = ItemService::new(Arc::new(ItemRepositoryMock {}));
        let get_item_req = ItemRequest {
            number: String::from("Hello"),
            name: "MEME".to_owned(),
        };

        let response = item_service
            .get_item(Request::new(get_item_req))
            .await?
            .into_inner();

        assert_eq!(response.id, "1");
        assert_eq!(response.number, "Hello");
        assert_eq!(response.name, "MEME");

        Ok(())
    }
}
