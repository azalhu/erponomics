use crate::item::grpc::proto::repository::item_service_client as grpc;
pub use crate::proto::item::repository as proto;

use std::sync::Arc;

use tonic::transport::Channel;

pub mod create;
pub mod delete;
pub mod get;

// TODO pub(crate)
pub struct Client {
    item_service_client: Arc<grpc::ItemServiceClient<Channel>>,
}
