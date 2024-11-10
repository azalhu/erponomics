use tonic::Request;

use crate::Id;

use super::{proto, Error};

impl TryFrom<Request<proto::DeleteItemRequest>> for Id {
    type Error = Error;

    fn try_from(value: Request<proto::DeleteItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        let id = value.id.try_into()?;

        Ok(id)
    }
}
