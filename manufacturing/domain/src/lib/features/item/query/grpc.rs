pub use super::{proto, Error};
pub use proto::item_query_service_server::ItemQueryService;

use tonic::{Request, Response, Status};

use super::{Get, GrpcService};

pub mod get;

#[tonic::async_trait]
impl<IQS> ItemQueryService for GrpcService<IQS>
where
    IQS: Get,
{
    async fn get_item(
        &self,
        request: Request<proto::GetItemRequest>,
    ) -> Result<Response<proto::GetItemResponse>, Status> {
        let id = request.try_into()?;
        let item = self.item_query_service.get(id).await?;

        Ok(item.into())
    }
}
