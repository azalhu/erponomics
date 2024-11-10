use anyhow::anyhow;
use tonic::Status;

use crate::item::Error;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
            Error::Timestamp(err) => Self::invalid_argument(err.to_string()),
            Error::Description(err) => Self::invalid_argument(err.to_string()),
            Error::Name(err) => Self::invalid_argument(err.to_string()),
            Error::Code(err) => Self::invalid_argument(err.to_string()),
            Error::Id(err) => Self::invalid_argument(err.to_string()),
            Error::Empty(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl From<Status> for Error {
    fn from(value: Status) -> Self {
        Self::Unknown(anyhow!(value))
    }
}
