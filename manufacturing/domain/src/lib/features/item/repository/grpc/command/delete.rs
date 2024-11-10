use std::ops::Deref;

use tonic::Request;

use crate::item::repository::{delete, Delete};

use super::{proto, Client};

pub mod error;
pub mod request;

impl Delete for Client {
    async fn delete(&self, req: delete::Request) -> Result<(), delete::Error> {
        let req: Request<proto::DeleteItemRequest> = req.into();

        self.command_service
            .deref()
            .clone()
            .delete_item(req)
            .await?;

        Ok(())
    }
}
