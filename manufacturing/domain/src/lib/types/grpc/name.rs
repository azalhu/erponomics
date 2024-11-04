use std::{convert::TryInto, str::FromStr};

use anyhow::anyhow;
use tonic::{Code, Status};

use crate::{
    name::{EmptyError, Error},
    Name,
};

use super::proto;

impl From<Name> for proto::Name {
    fn from(value: Name) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl From<Name> for Option<proto::Name> {
    fn from(value: Name) -> Self {
        Some(value.into())
    }
}

impl TryFrom<proto::Name> for Name {
    type Error = Error;

    fn try_from(value: proto::Name) -> Result<Self, Self::Error> {
        Self::from_str(&value.value)
    }
}

impl TryFrom<Option<proto::Name>> for Name {
    type Error = Error;

    fn try_from(value: Option<proto::Name>) -> Result<Self, Self::Error> {
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
