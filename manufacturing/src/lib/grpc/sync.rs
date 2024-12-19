use prost_types::Any;
use tonic::{Request, Response, Status};

use crate::sync::{self, OperationMetadata};

use super::proto::google::{
    longrunning::{
        operation, operations_server::Operations, CancelOperationRequest, DeleteOperationRequest,
        GetOperationRequest, ListOperationsRequest, ListOperationsResponse, Operation,
        WaitOperationRequest,
    },
    rpc,
};

pub struct Service;

impl<T> From<sync::Operation<T>> for Operation
where
    T: OperationMetadata + Into<Option<Any>>,
    <T as OperationMetadata>::Response: Into<Any>,
    <T as OperationMetadata>::Error: Into<rpc::Status>,
{
    fn from(value: sync::Operation<T>) -> Self {
        let (id, metadata, result) = value.dissolve();

        let name = id.into();
        let metadata = metadata.into();
        let result = result.map(|res| match res {
            Ok(ent) => operation::Result::Response(ent.into()),
            Err(err) => operation::Result::Error(err.into()),
        });
        let done = result.is_some();

        Self {
            name,
            metadata,
            done,
            result,
        }
    }
}

// MARK: Service

#[tonic::async_trait]
impl Operations for Service {
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
