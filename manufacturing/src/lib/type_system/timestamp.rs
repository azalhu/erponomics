use std::str::FromStr;

use chrono::{DateTime, ParseError, Utc};
use derive_more::derive::From;

use crate::ThisError;

use super::Timestamp;

impl Timestamp {
    #[must_use]
    pub const fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn now() -> Self {
        Self::new(Utc::now())
    }

    #[must_use]
    pub const fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl TryFrom<String> for Timestamp {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(EmptyError.into());
        }

        DateTime::from_str(value).map_or_else(
            |err| {
                Err(InvalidFormatError {
                    raw: value.to_string(),
                    inner_error: err,
                }
                .into())
            },
            |value| Ok(Self::new(value)),
        )
    }
}

impl TryFrom<Option<String>> for Timestamp {
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
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("timestamp cannot be empty")]
pub struct EmptyError;

#[derive(Clone, Debug, ThisError, From)]
#[error("timestamp format is invalid: expected DateTime<Utc>, got {raw}, inner: {inner_error}")]
pub struct InvalidFormatError {
    raw: String,
    inner_error: ParseError,
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use chrono::Datelike;

    use super::*;

    #[test]
    fn new_returns_new_date_time_utc() {
        let old_time = Utc::now().with_year(2022).unwrap();
        let old_time_clone = old_time.clone();
        let date_time_utc = Timestamp::new(old_time);
        assert_eq!(old_time_clone, *date_time_utc.value());
    }

    #[test]
    fn now_returns_utc_now() {
        let before = Utc::now();
        sleep(Duration::from_millis(1));
        let now = *Timestamp::now().value();
        sleep(Duration::from_millis(1));
        let after = Utc::now();

        assert!(before < now && now < after);
    }

    #[test]
    fn value_should_return_initial_value() {
        let now = Timestamp::now();
        sleep(Duration::from_millis(1));
        let still_now = now.value();
        sleep(Duration::from_millis(1));
        assert_eq!(now.value(), still_now);
    }
}
