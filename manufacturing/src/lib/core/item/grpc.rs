use std::sync::Arc;

use anyhow::anyhow;
use num_traits::ToPrimitive;
use tonic::{Request, Response, Status};

use crate::{
    grpc::proto::google::longrunning::Operation,
    item::{EmptyError, Error},
    proto::{
        self,
        item::{
            item_command_service_server::ItemCommandService,
            item_query_service_server::ItemQueryService, CreateItemRequest, DeleteItemRequest,
            GetItemRequest, ListItemsRequest, ListItemsResponse, UpdateItemRequest,
        },
    },
    Item,
};

use super::{
    command::{self, Create, Delete, Update},
    query::{self, Get, List},
};

#[derive(Debug, Clone)]
pub struct Service<ICS: Create + Update + Delete + Clone, IQS: Get + List + Clone> {
    item_command_service: Arc<ICS>,
    item_query_service: Arc<IQS>,
}

impl From<Item> for proto::item::Item {
    fn from(value: Item) -> Self {
        Self {
            name: value.id.to_string(),
            display_name: value.display_name,
            title: value.title,
            description: value.description,
            state: value.state.to_i32().unwrap_or(0),
            etag: value.etag.to_string(),
            uid: value.uid.to_string(),
            create_time: value.create_time.into(),
            update_time: value.update_time.into(),
        }
    }
}

impl From<Item> for Response<proto::item::Item> {
    fn from(value: Item) -> Self {
        Response::new(value.into())
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
            Error::Timestamp(err) => Self::invalid_argument(err.to_string()),
            Error::Etag(err) => Self::invalid_argument(err.to_string()),
            Error::Id(err) => Self::invalid_argument(err.to_string()),
            Error::Empty(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl From<Status> for Error {
    fn from(value: Status) -> Self {
        Self::Unknown(anyhow!(value))
    }
}

impl<ICS, IQS> Service<ICS, IQS>
where
    ICS: Create + Update + Delete + Clone,
    IQS: Get + List + Clone,
{
    pub const fn new(item_command_service: Arc<ICS>, item_query_service: Arc<IQS>) -> Self {
        Self {
            item_command_service,
            item_query_service,
        }
    }
}

// MARK: Command

#[tonic::async_trait]
impl<ICS, IQS> ItemCommandService for Service<ICS, IQS>
where
    ICS: Create + Update + Delete + Clone,
    IQS: Get + List + Clone,
{
    async fn create_item(
        &self,
        request: Request<CreateItemRequest>,
    ) -> Result<Response<Operation>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .create(request)
            .await
            .map_err(Status::from)?;

        todo!()
    }

    async fn update_item(
        &self,
        request: Request<UpdateItemRequest>,
    ) -> Result<Response<Operation>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .update(request)
            .await
            .map_err(Status::from)?;

        todo!()
    }

    async fn delete_item(
        &self,
        request: Request<DeleteItemRequest>,
    ) -> Result<Response<Operation>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .delete(request)
            .await
            .map_err(Status::from)?;

        todo!()
    }
}

impl TryFrom<Request<CreateItemRequest>> for command::CreateRequest {
    type Error = Error;

    fn try_from(value: Request<CreateItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        match value.item {
            None => Err(EmptyError.into()),
            Some(item) => Ok(Self::new(
                value.item_id,
                item.display_name,
                item.title,
                item.description,
            )),
        }
    }
}

impl TryFrom<Request<UpdateItemRequest>> for command::UpdateRequest {
    type Error = Error;

    fn try_from(value: Request<UpdateItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        match value.item {
            None => Err(EmptyError.into()),
            Some(item) => Ok(Self::new(
                item.name,
                Some(item.display_name),
                Some(item.title),
                Some(item.description),
                value.etag,
            )),
        }
    }
}

impl TryFrom<Request<DeleteItemRequest>> for command::DeleteRequest {
    type Error = Error;

    fn try_from(value: Request<DeleteItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        Ok(Self::new(value.name, value.etag))
    }
}

// MARK: Query

#[tonic::async_trait]
impl<ICS, IQS> ItemQueryService for Service<ICS, IQS>
where
    ICS: Create + Update + Delete + Clone,
    IQS: Get + List + Clone,
{
    async fn get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<proto::item::Item>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_query_service
            .get(request)
            .await
            .map_err(Status::from)?;

        todo!()
    }

    async fn list_items(
        &self,
        request: Request<ListItemsRequest>,
    ) -> Result<Response<ListItemsResponse>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_query_service
            .list(request)
            .await
            .map_err(Status::from)?;

        todo!()
    }
}

impl TryFrom<Request<GetItemRequest>> for query::GetRequest {
    type Error = Error;

    fn try_from(value: Request<GetItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        Ok(Self::new(value.name))
    }
}

impl TryFrom<Request<ListItemsRequest>> for query::ListRequest {
    type Error = Error;

    fn try_from(value: Request<ListItemsRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        Ok(Self::new(
            value.page_size,
            value.page_token,
            value.order_by,
            value.filter,
        ))
    }
}
