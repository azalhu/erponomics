use crate::{id, item};

use super::Error;

impl From<id::Error> for Error {
    fn from(value: id::Error) -> Self {
        item::Error::Id(value).into()
    }
}

impl From<Error> for item::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::Unknown(err),
            Error::NotFound => Self::Id(id::NotFoundError.into()),
            Error::Invalid(err) => err,
        }
    }
}
