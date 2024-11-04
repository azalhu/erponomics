use std::str::FromStr;

use derive_more::derive::From;

use crate::{Name, ThisError};

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(EmptyError.into())
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Empty(#[from] EmptyError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("name cannot be empty")]
pub struct EmptyError;
