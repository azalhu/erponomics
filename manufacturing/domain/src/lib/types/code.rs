use derive_more::derive::From;

use crate::{Code, ThisError};

impl TryFrom<String> for Code {
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
    Duplicate(#[from] DuplicateError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("code cannot be empty")]
pub struct EmptyError;

#[derive(Clone, Debug, ThisError, From)]
#[error("code already exists")]
pub struct DuplicateError(pub Code);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_is_convertible_from_string_reference() {
        let code_string = String::from("hello");
        let code = Code::try_from(code_string.clone()).unwrap();
        assert_eq!(code_string, code.0);
    }
}
