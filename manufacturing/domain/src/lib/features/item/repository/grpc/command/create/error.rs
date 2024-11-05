use anyhow::anyhow;
use tonic::Status;

use super::create::Error;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
            Error::Duplicate { code } => Self::failed_precondition(code.to_string()),
            Error::InvalidItem(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl From<Status> for Error {
    fn from(value: Status) -> Self {
        Self::Unknown(anyhow!(value))
    }
}
