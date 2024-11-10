use crate::{code, description, id, item, name, timestamp};

use super::Error;

impl From<Error> for item::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::Unknown(err),
            Error::Invalid(err) => err,
            Error::NotFound => Self::Id(id::NotFoundError.into()),
        }
    }
}

impl From<id::Error> for Error {
    fn from(value: id::Error) -> Self {
        item::Error::Id(value).into()
    }
}

impl From<code::Error> for Error {
    fn from(value: code::Error) -> Self {
        item::Error::Code(value).into()
    }
}

impl From<name::Error> for Error {
    fn from(value: name::Error) -> Self {
        item::Error::Name(value).into()
    }
}

impl From<description::Error> for Error {
    fn from(value: description::Error) -> Self {
        item::Error::Description(value).into()
    }
}

impl From<timestamp::Error> for Error {
    fn from(value: timestamp::Error) -> Self {
        item::Error::Timestamp(value).into()
    }
}
