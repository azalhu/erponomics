use tonic::{IntoRequest, Request};

use crate::Id;

use super::{get, proto};

impl From<get::Request> for Request<proto::GetItemRequest> {
    fn from(value: get::Request) -> Self {
        let id = value.dissolve();

        proto::GetItemRequest {
            id: Some(id.into()),
        }
        .into_request()
    }
}

impl TryFrom<Request<proto::GetItemRequest>> for get::Request {
    type Error = get::Error;

    fn try_from(value: Request<proto::GetItemRequest>) -> Result<Self, Self::Error> {
        let id: Id = value.into_inner().id.try_into()?;

        Ok(Self::from(id))
    }
}
