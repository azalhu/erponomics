use std::ops::Deref;

use tonic::Request;

use crate::item::repository::{create, Create};

use super::{proto, Client};

pub mod error;
pub mod request;

impl Create for Client {
    async fn create(&self, req: create::Request) -> Result<(), create::Error> {
        let req: Request<proto::CreateItemRequest> = req.into();

        self.command_service
            .deref()
            .clone()
            .create_item(req)
            .await?;

        Ok(())
    }
}
