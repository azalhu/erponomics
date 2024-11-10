use std::convert::TryInto;

use anyhow::anyhow;
use tonic::{Code, Status};

use super::proto;

use crate::{
    description::{EmptyError, Error},
    Description,
};

impl From<Description> for proto::Description {
    fn from(value: Description) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl From<Description> for Option<proto::Description> {
    fn from(value: Description) -> Self {
        Some(value.into())
    }
}

impl TryFrom<proto::Description> for Description {
    type Error = Error;

    fn try_from(value: proto::Description) -> Result<Self, Self::Error> {
        Self::try_from(value.value)
    }
}

impl TryFrom<Option<proto::Description>> for Description {
    type Error = Error;

    fn try_from(value: Option<proto::Description>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Err(EmptyError.into()), TryInto::try_into)
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown(err) => Self::unknown(err.to_string()),
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
