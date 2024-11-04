use std::{convert::TryInto, str::FromStr};

use derive_more::derive::From;
use ulid::{DecodeError, Ulid};

use crate::{Id, ThisError};

impl Id {
    #[must_use]
    pub const fn value(&self) -> &Ulid {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Ulid::new().into()
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ulid::try_from(s).map_or_else(
            |err| {
                Err(InvalidFormatError {
                    raw: s.to_string(),
                    inner_error: err,
                }
                .into())
            },
            |ulid| Ok(ulid.into()),
        )
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<Option<&str>> for Id {
    type Error = Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Err(EmptyError.into()), TryInto::try_into)
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Empty(#[from] EmptyError),
    #[error(transparent)]
    InvalidFormat(#[from] InvalidFormatError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("id cannot be empty")]
pub struct EmptyError;

#[derive(Clone, Debug, ThisError, From)]
#[error("id format is invalid: expected ULID, got {raw}, inner: {inner_error}")]
pub struct InvalidFormatError {
    raw: String,
    inner_error: DecodeError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_id_is_a_valid_ulid() {
        let id = Id::default();
        assert!(!Ulid::is_nil(id.value()));
    }
}
