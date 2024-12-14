use std::sync::Arc;

use anyhow::anyhow;
use tonic::{Request, Response, Status};

use crate::{
    entity_tag,
    grpc::proto::google::longrunning::Operation,
    item::{
        command::{self, Block, Create, Delete, Unblock, Update},
        query::{self, Get, List},
        EmptyError, Error,
    },
    proto::{
        self, item_service_server::ItemService, BlockItemRequest, CreateItemRequest,
        DeleteItemRequest, GetItemRequest, ListItemsRequest, ListItemsResponse, UnblockItemRequest,
        UpdateItemRequest,
    },
    Item, ItemState,
};

use super::proto::google::longrunning::{
    operations_server::Operations, CancelOperationRequest, DeleteOperationRequest,
    GetOperationRequest, ListOperationsRequest, ListOperationsResponse, WaitOperationRequest,
};

#[derive(Debug, Clone)]
pub struct Service<ICS: Create + Update + Delete + Block + Unblock + Clone, IQS: Get + List + Clone>
{
    item_command_service: Arc<ICS>,
    item_query_service: Arc<IQS>,
}

impl From<Item> for proto::Item {
    fn from(value: Item) -> Self {
        let (id, display_name, title, description, state, etag, uid, create_time, update_time) =
            value.dissolve();

        Self {
            name: id.to_string(),
            display_name: display_name.into(),
            title: title.into(),
            description: description.into(),
            state: state.into(),
            etag: etag.to_string().into(),
            uid: uid.to_string().into(),
            create_time: create_time.into(),
            update_time: update_time.into(),
        }
    }
}

impl From<Item> for Response<proto::Item> {
    fn from(value: Item) -> Self {
        Self::new(value.into())
    }
}

impl From<ItemState> for proto::item::State {
    fn from(value: ItemState) -> Self {
        match value {
            ItemState::Creating => Self::Creating,
            ItemState::Updating => Self::Updating,
            ItemState::Deleting => Self::Deleting,
            ItemState::Annihilating => Self::Annihilating,
            ItemState::Blocking => Self::Blocking,
            ItemState::Unblocking => Self::Unblocking,
            ItemState::Active => Self::Active,
            ItemState::Blocked => Self::Blocked,
        }
    }
}

impl From<ItemState> for Option<i32> {
    fn from(value: ItemState) -> Self {
        Some(proto::item::State::from(value) as i32)
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
    ICS: Create + Update + Delete + Block + Unblock + Clone,
    IQS: Get + List + Clone,
{
    pub const fn new(item_command_service: Arc<ICS>, item_query_service: Arc<IQS>) -> Self {
        Self {
            item_command_service,
            item_query_service,
        }
    }
}

// MARK: ItemService

#[tonic::async_trait]
impl<ICS, IQS> ItemService for Service<ICS, IQS>
where
    ICS: Create + Update + Delete + Block + Unblock + Clone,
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

        Ok(Response::new(Operation {
            name: String::new(),
            metadata: None,
            done: false,
            result: None,
        }))
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

        Ok(Response::new(Operation {
            name: String::new(),
            metadata: None,
            done: false,
            result: None,
        }))
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

        Ok(Response::new(Operation {
            name: String::new(),
            metadata: None,
            done: false,
            result: None,
        }))
    }

    async fn block_item(
        &self,
        request: Request<BlockItemRequest>,
    ) -> Result<Response<Operation>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .block(request)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(Operation {
            name: String::new(),
            metadata: None,
            done: false,
            result: None,
        }))
    }

    async fn unblock_item(
        &self,
        request: Request<UnblockItemRequest>,
    ) -> Result<Response<Operation>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        self.item_command_service
            .unblock(request)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(Operation {
            name: String::new(),
            metadata: None,
            done: false,
            result: None,
        }))
    }

    async fn get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<proto::Item>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        let item = self
            .item_query_service
            .get(request)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(item.into()))
    }

    async fn list_items(
        &self,
        request: Request<ListItemsRequest>,
    ) -> Result<Response<ListItemsResponse>, Status> {
        let request = request.try_into().map_err(Status::from)?;

        let response = self
            .item_query_service
            .list(request)
            .await
            .map_err(Status::from)?;

        Ok(Response::new(response.into()))
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
                item.display_name.unwrap_or(String::new()),
                item.title.unwrap_or(String::new()),
                item.description.unwrap_or(String::new()),
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
                item.display_name,
                item.title,
                item.description,
                item.etag
                    .ok_or(Error::Etag(entity_tag::EmptyError.into()))?,
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

impl TryFrom<Request<BlockItemRequest>> for command::BlockRequest {
    type Error = Error;

    fn try_from(value: Request<BlockItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        Ok(Self::new(value.name, value.etag))
    }
}

impl TryFrom<Request<UnblockItemRequest>> for command::UnblockRequest {
    type Error = Error;

    fn try_from(value: Request<UnblockItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        Ok(Self::new(value.name, value.etag))
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

impl From<query::ListResponse> for ListItemsResponse {
    fn from(value: query::ListResponse) -> Self {
        let (items, next_page_token, total_size) = value.dissolve();

        Self {
            items: items.into_iter().map(Item::into).collect(),
            next_page_token,
            total_size,
        }
    }
}

// MARK: Operations

#[tonic::async_trait]
impl<ICS, IQS> Operations for Service<ICS, IQS>
where
    ICS: Create + Update + Delete + Block + Unblock + Clone,
    IQS: Get + List + Clone,
{
    async fn list_operations(
        &self,
        _request: Request<ListOperationsRequest>,
    ) -> Result<Response<ListOperationsResponse>, Status> {
        todo!()
    }

    async fn get_operation(
        &self,
        _request: Request<GetOperationRequest>,
    ) -> Result<Response<Operation>, Status> {
        todo!()
    }

    async fn delete_operation(
        &self,
        _request: Request<DeleteOperationRequest>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn cancel_operation(
        &self,
        _request: Request<CancelOperationRequest>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn wait_operation(
        &self,
        _request: Request<WaitOperationRequest>,
    ) -> Result<Response<Operation>, Status> {
        todo!()
    }
}
