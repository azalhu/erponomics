use anyhow::anyhow;
use tonic::{Code, Status};

use crate::id::{EmptyError, Error};

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
            Error::NotFound(err) => Self::not_found(err.to_string()),
            Error::Duplicate(err) => Self::invalid_argument(err.to_string()),
            Error::InvalidFormat(err) => Self::invalid_argument(err.to_string()),
            Error::Empty(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl TryFrom<Status> for Error {
    type Error = anyhow::Error;

    fn try_from(value: Status) -> Result<Self, Self::Error> {
        match value.code() {
            Code::Unknown => Ok(Self::Unknown(anyhow!(value.message().to_string()))),
            Code::InvalidArgument => Ok(Self::Empty(EmptyError)),
            _ => Err(anyhow!(value)),
        }
    }
}
