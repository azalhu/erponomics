use std::convert::TryInto;

use anyhow::anyhow;
use chrono::DateTime;
use tonic::{Code, Status};

use crate::id;
use crate::Timestamp;

// MARK: Id

impl From<id::Error> for Status {
    fn from(value: id::Error) -> Self {
        match value {
            id::Error::Unknown(err) => Self::unknown(err.to_string()),
            id::Error::NotFound(err) => Self::not_found(err.to_string()),
            id::Error::Duplicate(err) => Self::invalid_argument(err.to_string()),
            id::Error::InvalidFormat(err) => Self::invalid_argument(err.to_string()),
            id::Error::Empty(err) => Self::invalid_argument(err.to_string()),
        }
    }
}

impl TryFrom<Status> for id::Error {
    type Error = anyhow::Error;

    fn try_from(value: Status) -> Result<Self, Self::Error> {
        match value.code() {
            Code::Unknown => Ok(Self::Unknown(anyhow!(value.message().to_string()))),
            Code::InvalidArgument => Ok(Self::Empty(id::EmptyError)),
            _ => Err(anyhow!(value)),
        }
    }
}

// MARK: Timestamp

impl From<Timestamp> for prost_types::Timestamp {
    fn from(value: Timestamp) -> Self {
        let value = value.dissolve();
        Self {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos().try_into().unwrap_or(0),
        }
    }
}

impl From<Timestamp> for Option<prost_types::Timestamp> {
    fn from(value: Timestamp) -> Self {
        Some(value.into())
    }
}

impl TryFrom<prost_types::Timestamp> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: prost_types::Timestamp) -> Result<Self, Self::Error> {
        let mut new_value = value;
        new_value.normalize();
        let new_value =
            DateTime::from_timestamp(new_value.seconds, new_value.nanos.try_into().unwrap_or(0));

        new_value.map_or_else(
            || anyhow::bail!("invalid timestamp: {}", value),
            |new_value| Ok(Self::new(new_value)),
        )
    }
}

impl TryFrom<Option<prost_types::Timestamp>> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: Option<prost_types::Timestamp>) -> Result<Self, Self::Error> {
        value.map_or_else(|| anyhow::bail!("timestamp must exist"), TryInto::try_into)
    }
}
