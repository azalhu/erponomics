use std::convert::TryInto;

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

impl TryFrom<String> for Id {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(EmptyError.into());
        }

        Ulid::from_string(value).map_or_else(
            |err| {
                Err(InvalidFormatError {
                    raw: value.to_string(),
                    inner_error: err,
                }
                .into())
            },
            |ulid| Ok(ulid.into()),
        )
    }
}

impl TryFrom<Option<String>> for Id {
    type Error = Error;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Err(EmptyError.into()), TryInto::try_into)
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Empty(#[from] EmptyError),
    #[error(transparent)]
    NotFound(#[from] NotFoundError),
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

#[derive(Clone, Debug, ThisError, From)]
#[error("id not found")]
pub struct NotFoundError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_id_is_a_valid_ulid() {
        let id = Id::default();
        assert!(!Ulid::is_nil(id.value()));
    }
}
