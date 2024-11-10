use derive_more::derive::From;

use crate::{Description, ThisError};

impl TryFrom<String> for Description {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(EmptyError.into())
        } else {
            Ok(Self(trimmed.to_string()))
        }
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
        let description = Description::try_from(description_string.clone()).unwrap();
        assert_eq!(description_string, description.0);
    }
}
