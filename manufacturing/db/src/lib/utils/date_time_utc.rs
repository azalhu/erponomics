use std::time::SystemTime;

use chrono::{DateTime, Utc};
use derive_more::derive::From;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DateTimeUtc(DateTime<Utc>);

impl DateTimeUtc {
    pub fn now() -> DateTimeUtc {
        Utc::now().into()
    }
}

impl From<DateTimeUtc> for SystemTime {
    fn from(value: DateTimeUtc) -> Self {
        value.0.into()
    }
}
