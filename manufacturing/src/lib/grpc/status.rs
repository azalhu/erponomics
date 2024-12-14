use anyhow::anyhow;
use tonic::{Code, Status};

use crate::id;

impl From<id::Error> for Status {
    fn from(value: id::Error) -> Self {
        match value {
            id::Error::Unknown(err) => Self::unknown(err.to_string()),
            id::Error::NotFound(err) => Self::not_found(err.to_string()),
            id::Error::Duplicate(err) => Self::invalid_argument(err.to_string()),
            id::Error::InvalidFormat(err) => Self::invalid_argument(err.to_string()),
            id::Error::Empty(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl TryFrom<Status> for id::Error {
    type Error = anyhow::Error;

    fn try_from(value: Status) -> Result<Self, Self::Error> {
        match value.code() {
            Code::Unknown => Ok(Self::Unknown(anyhow!(value.message().to_string()))),
            Code::InvalidArgument => Ok(Self::Empty(id::EmptyError)),
            _ => Err(anyhow!(value)),
        }
    }
}
