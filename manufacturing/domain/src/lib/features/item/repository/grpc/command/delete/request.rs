use tonic::{IntoRequest, Request};

use crate::Id;

use super::{delete, proto};

impl From<delete::Request> for Request<proto::DeleteItemRequest> {
    fn from(value: delete::Request) -> Self {
        let id = value.dissolve();

        proto::DeleteItemRequest {
            id: Some(id.into()),
        }
        .into_request()
    }
}

impl TryFrom<Request<proto::DeleteItemRequest>> for delete::Request {
    type Error = delete::Error;

    fn try_from(value: Request<proto::DeleteItemRequest>) -> Result<Self, Self::Error> {
        let id: Id = value.into_inner().id.try_into()?;

        Ok(Self::from(id))
    }
}
