use std::str::FromStr;

use derive_more::derive::From;

use crate::{Code, ThisError};

impl FromStr for Code {
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

impl TryFrom<&str> for Code {
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
#[error("code cannot be empty")]
pub struct EmptyError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_is_convertible_from_string_reference() {
        let code_string = String::from("hello");
        let code = Code::from_str(&code_string).unwrap();
        assert_eq!(code_string, code.0);
    }
}
