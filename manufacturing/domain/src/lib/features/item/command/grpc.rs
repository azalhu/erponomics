pub use super::{proto, Error};
pub use proto::item_command_service_server::ItemCommandService;

use std::sync::Arc;

use tonic::{Request, Response, Status};

use super::{Create, Delete, GrpcService};

pub mod create;
pub mod delete;

impl<ICS> GrpcService<ICS>
where
    ICS: Create + Delete,
{
    pub const fn new(item_command_service: Arc<ICS>) -> Self {
        Self {
            item_command_service,
        }
    }
}

#[tonic::async_trait]
impl<ICS> ItemCommandService for GrpcService<ICS>
where
    ICS: Create + Delete,
{
    async fn create_item(
        &self,
        request: Request<proto::CreateItemRequest>,
    ) -> anyhow::Result<Response<()>, Status> {
        let item = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .create(item)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(()))
    }

    async fn delete_item(
        &self,
        request: Request<proto::DeleteItemRequest>,
    ) -> Result<Response<()>, Status> {
        let id = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .delete(id)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(()))
    }
}
