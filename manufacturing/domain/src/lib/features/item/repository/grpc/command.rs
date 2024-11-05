use crate::item::grpc::proto::repository::item_command_service_client as grpc;
pub use crate::proto::item::repository as proto;

use std::sync::Arc;

use tonic::transport::Channel;

pub mod create;
pub mod delete;

// TODO pub(crate)
pub struct Client {
    service: Arc<grpc::ItemCommandServiceClient<Channel>>,
}
