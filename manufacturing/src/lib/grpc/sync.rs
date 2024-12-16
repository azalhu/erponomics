use tonic::{Request, Response, Status};

use super::proto::google::longrunning::{
    operations_server::Operations, CancelOperationRequest, DeleteOperationRequest,
    GetOperationRequest, ListOperationsRequest, ListOperationsResponse, Operation,
    WaitOperationRequest,
};

pub struct Service;

// MARK: Operations

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
