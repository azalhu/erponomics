use std::str::FromStr;

use derive_more::derive::From;

use crate::{Description, ThisError};

impl FromStr for Description {
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

impl TryFrom<&str> for Description {
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
#[error("description cannot be empty")]
pub struct EmptyError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_is_convertible_from_string_reference() {
        let description_string = String::from("hello");
        let description = Description::from_str(&description_string).unwrap();
        assert_eq!(description_string, description.0);
    }
}
