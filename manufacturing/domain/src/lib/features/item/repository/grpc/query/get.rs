use std::ops::Deref;

use tonic::Request;

use crate::item::repository::{get, Get};

use super::{proto, Client};

pub mod error;
pub mod request;
pub mod response;

impl Get for Client {
    async fn get(&self, req: get::Request) -> Result<get::Response, get::Error> {
        let req: Request<proto::GetItemRequest> = req.into();

        self.query_service
            .deref()
            .clone()
            .get_item(req)
            .await?
            .try_into()
    }
}
