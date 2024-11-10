use tonic::{Request, Response};

use crate::{Id, Item};

use super::{proto, Error};

impl From<Item> for Response<proto::GetItemResponse> {
    fn from(value: Item) -> Self {
        Self::new(proto::GetItemResponse { item: value.into() })
    }
}

impl TryFrom<Request<proto::GetItemRequest>> for Id {
    type Error = Error;

    fn try_from(value: Request<proto::GetItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        let id = value.id.try_into()?;

        Ok(id)
    }
}
