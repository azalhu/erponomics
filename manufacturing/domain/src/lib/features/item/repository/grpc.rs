use std::sync::Arc;

use tonic::transport::Channel;

pub use crate::proto::item::repository as proto;

use self::proto::{
    item_command_service_client::ItemCommandServiceClient,
    item_query_service_client::ItemQueryServiceClient,
};

pub mod client;
pub mod command;
pub mod query;

#[derive(Debug)]
pub struct Client {
    command_service: Arc<ItemCommandServiceClient<Channel>>,
    query_service: Arc<ItemQueryServiceClient<Channel>>,
}
