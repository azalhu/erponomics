use db::grpc::items::{item_server::Item, ItemRequest, ItemResponse};
use prost_types::Timestamp;
use std::time::SystemTime;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ItemService;

#[tonic::async_trait]
impl Item for ItemService {
    async fn create_item(
        &self,
        request: Request<ItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        println!("Got a create request: {:?}", request);

        let req = request.into_inner();

        let response = Response::new(ItemResponse {
            id: 0,
            code: req.code,
            created_at: Some(Timestamp::from(SystemTime::now())),
        });

        Ok(response)
    }

    async fn get_item(
        &self,
        request: Request<ItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        let response = Response::new(ItemResponse {
            id: 1,
            code: req.code,
            created_at: Some(Timestamp::from(SystemTime::now())),
        });

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_item_returns_ok_response() -> Result<(), Status> {
        let item_service = ItemService::default();
        let create_item_req = ItemRequest {
            code: "Hello".to_owned(),
        };

        let response = item_service
            .create_item(Request::new(create_item_req))
            .await?
            .into_inner();

        assert_eq!(response.id, 0);
        assert_eq!(response.code, "Hello");

        Ok(())
    }

    #[tokio::test]
    async fn get_item_returns_ok_response() -> Result<(), Status> {
        let item_service = ItemService::default();
        let get_item_req = ItemRequest {
            code: String::from("Hello"),
        };

        let response = item_service
            .get_item(Request::new(get_item_req))
            .await?
            .into_inner();

        assert_eq!(response.id, 1);
        assert_eq!(response.code, "Hello");

        Ok(())
    }
}
