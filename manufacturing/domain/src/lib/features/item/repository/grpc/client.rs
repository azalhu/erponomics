use std::sync::Arc;

use tonic::transport::Channel;

use super::{
    proto::{
        item_command_service_client::ItemCommandServiceClient,
        item_query_service_client::ItemQueryServiceClient,
    },
    Client,
};

impl Client {
    #[must_use]
    pub const fn new(
        command_service: Arc<ItemCommandServiceClient<Channel>>,
        query_service: Arc<ItemQueryServiceClient<Channel>>,
    ) -> Self {
        Self {
            command_service,
            query_service,
        }
    }
}
