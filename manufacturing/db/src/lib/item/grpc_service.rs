use std::sync::Arc;

use manufacturing::{
    item::repository::{Create, Delete, Get},
    proto::item::repository::{self as proto, item_service_server::ItemService},
};
use tonic::{Request, Response, Status};

use super::GrpcService;

impl<IR> GrpcService<IR>
where
    IR: Create + Get + Delete,
{
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }
}

#[tonic::async_trait]
impl<IR> ItemService for GrpcService<IR>
where
    IR: Create + Get + Delete,
{
    async fn create_item(
        &self,
        request: Request<proto::CreateItemRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_repository
            .create(request)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(()))
    }

    async fn get_item(
        &self,
        request: Request<proto::GetItemRequest>,
    ) -> Result<Response<proto::GetItemResponse>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        let response = self
            .item_repository
            .get(request)
            .await
            .map_err(Status::from)?;

        Ok(response.into())
    }

    async fn delete_item(
        &self,
        request: Request<proto::DeleteItemRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_repository
            .delete(request)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use manufacturing::{
        item::repository::{create, delete, get},
        Code, Description, Id, Item, Name, Timestamp,
    };

    use super::*;

    struct ItemRepositoryMock;

    impl Create for ItemRepositoryMock {
        async fn create(&self, _req: create::Request) -> Result<(), create::Error> {
            Ok(())
        }
    }

    impl Get for ItemRepositoryMock {
        async fn get(&self, _req: get::Request) -> Result<get::Response, get::Error> {
            todo!()
        }
    }

    impl Delete for ItemRepositoryMock {
        async fn delete(&self, _req: delete::Request) -> Result<(), delete::Error> {
            todo!()
        }
    }

    #[tokio::test]
    async fn create_item_returns_ok_response() -> anyhow::Result<()> {
        let item_grpc_service = GrpcService::new(Arc::new(ItemRepositoryMock {}));
        let id = Id::default();
        let code = Code::from_str("Hello")?;
        let name = Name::from_str("Hello")?;
        let description = Description::from_str("Hello")?;
        let timestamp = Timestamp::now();
        let item = Item::from((id, code, name, description, timestamp));
        let create_item_req = create::Request::new(item).into();

        item_grpc_service.create_item(create_item_req).await?;

        Ok(())
    }
}
