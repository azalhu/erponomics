use anyhow::anyhow;
use tonic::{Code, Status};

use super::delete::Error;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
            Error::NotFound => Self::not_found(value.to_string()),
            Error::Invalid(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl From<Status> for Error {
    fn from(value: Status) -> Self {
        match value.code() {
            Code::NotFound => Self::NotFound,
            _ => Self::Unknown(anyhow!(value)),
        }
    }
}
