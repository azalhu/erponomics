use crate::item::grpc::proto::repository::item_query_service_client as grpc;
pub use crate::proto::item::repository as proto;

use std::sync::Arc;

use tonic::transport::Channel;

pub mod get;

// TODO pub(crate)
pub struct Client {
    service: Arc<grpc::ItemQueryServiceClient<Channel>>,
}
