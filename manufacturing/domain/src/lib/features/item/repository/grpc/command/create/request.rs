use tonic::{IntoRequest, Request};

use crate::Item;

use super::{create, proto};

impl From<create::Request> for Request<proto::CreateItemRequest> {
    fn from(value: create::Request) -> Self {
        let item = value.dissolve();

        proto::CreateItemRequest {
            item: Some(item.into()),
        }
        .into_request()
    }
}

impl TryFrom<Request<proto::CreateItemRequest>> for create::Request {
    type Error = create::Error;

    fn try_from(value: Request<proto::CreateItemRequest>) -> Result<Self, Self::Error> {
        let item: Item = value.into_inner().item.try_into()?;

        Ok(Self::from(item))
    }
}
