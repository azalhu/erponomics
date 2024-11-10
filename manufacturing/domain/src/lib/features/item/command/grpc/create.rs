use tonic::Request;

use crate::Item;

use super::{proto, Error};

impl TryFrom<Request<proto::CreateItemRequest>> for Item {
    type Error = Error;

    fn try_from(value: Request<proto::CreateItemRequest>) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        let code = value.code.try_into()?;
        let name = value.name.try_into()?;
        let description = value.description.try_into()?;

        Ok(Self::new(code, name, description))
    }
}
