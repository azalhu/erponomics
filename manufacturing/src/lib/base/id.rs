use std::convert::TryInto;

use derive_more::derive::From;
use uuid::Uuid;

use crate::{Id, ThisError};

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    #[must_use]
    pub const fn value(&self) -> &String {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.to_string()
    }
}

impl From<Id> for Option<String> {
    fn from(value: Id) -> Self {
        Some(value.into())
    }
}

impl TryFrom<String> for Id {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(EmptyError.into());
        }

        if !value
            .chars()
            .all(|c| c.is_lowercase() || c.is_numeric() || c == '-')
            || value.starts_with('-')
            || value.ends_with('-')
        {
            return Err(InvalidFormatError.into());
        }

        Ok(Self(value))
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
    InvalidFormat(#[from] InvalidFormatError),
    #[error(transparent)]
    Duplicate(#[from] DuplicateError),
    #[error(transparent)]
    NotFound(#[from] NotFoundError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("id cannot be empty")]
pub struct EmptyError;

#[derive(Clone, Debug, ThisError, From)]
#[error("id format is invalid")]
pub struct InvalidFormatError;

#[derive(Clone, Debug, ThisError, From)]
#[error("id already exists")]
pub struct DuplicateError(pub Id);

#[derive(Clone, Debug, ThisError, From)]
#[error("id not found")]
pub struct NotFoundError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_id_is_a_valid_non_nil_uuid() -> anyhow::Result<()> {
        let id = Id::new();
        let id_as_uuid = Uuid::try_parse(id.value()).map_err(|e| anyhow::anyhow!(e))?;
        assert!(!Uuid::is_nil(&id_as_uuid));

        Ok(())
    }

    #[test]
    fn new_id_is_valid_from_string() -> anyhow::Result<()> {
        let id = Id::new();
        let id_from_string = Id::try_from(id.value().to_string())?;
        assert_eq!(id, id_from_string);

        Ok(())
    }
}
