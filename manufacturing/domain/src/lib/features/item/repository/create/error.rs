use crate::{code, item};

use super::Error;

impl From<Error> for item::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::Unknown(err),
            Error::Duplicate { code } => Self::Code(code::DuplicateError(code).into()),
            Error::InvalidItem(err) => err,
        }
    }
}
