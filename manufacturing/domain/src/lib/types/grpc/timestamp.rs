use std::convert::TryInto;

use chrono::DateTime;

use crate::Timestamp;

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
