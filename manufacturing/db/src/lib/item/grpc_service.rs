use std::sync::Arc;

use manufacturing::{
    item::repository::{Create, Delete, Get},
    proto::item::repository::{
        self as proto, item_command_service_server::ItemCommandService,
        item_query_service_server::ItemQueryService,
    },
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
impl<IR> ItemCommandService for GrpcService<IR>
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

#[tonic::async_trait]
impl<IR> ItemQueryService for GrpcService<IR>
where
    IR: Create + Get + Delete,
{
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
}

#[cfg(test)]
mod tests {
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
        let code = Code::try_from("Hello".to_string())?;
        let name = Name::try_from("Hello".to_string())?;
        let description = Description::try_from("Hello".to_string())?;
        let timestamp = Timestamp::now();
        let item = Item::from((id, code, name, description, timestamp));
        let create_item_req = create::Request::from(item).into();

        item_grpc_service.create_item(create_item_req).await?;

        Ok(())
    }
}
