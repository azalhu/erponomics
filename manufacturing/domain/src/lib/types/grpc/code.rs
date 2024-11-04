use std::{convert::TryInto, str::FromStr};

use anyhow::anyhow;
use tonic::Status;

use super::proto;

use crate::{
    code::{EmptyError, Error},
    Code,
};

impl From<Code> for proto::Code {
    fn from(value: Code) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl From<Code> for Option<proto::Code> {
    fn from(value: Code) -> Self {
        Some(value.into())
    }
}

impl TryFrom<proto::Code> for Code {
    type Error = Error;

    fn try_from(value: proto::Code) -> Result<Self, Self::Error> {
        Self::from_str(&value.value)
    }
}

impl TryFrom<Option<proto::Code>> for Code {
    type Error = Error;

    fn try_from(value: Option<proto::Code>) -> Result<Self, Self::Error> {
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
            tonic::Code::Unknown => Ok(Self::Unknown(anyhow!(value.message().to_string()))),
            tonic::Code::InvalidArgument => Ok(Self::Empty(EmptyError)),
            _ => Err(anyhow!(value)),
        }
    }
}
