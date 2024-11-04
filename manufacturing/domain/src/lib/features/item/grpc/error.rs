use anyhow::anyhow;
use tonic::Status;

use crate::item::Error;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
            Error::Description(err) => Self::failed_precondition(err.to_string()),
            Error::Name(err) => Self::failed_precondition(err.to_string()),
            Error::Code(err) => Self::failed_precondition(err.to_string()),
            Error::Id(err) => Self::failed_precondition(err.to_string()),
            Error::Empty(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl From<Status> for Error {
    fn from(value: Status) -> Self {
        Self::Unknown(anyhow!(value))
    }
}
