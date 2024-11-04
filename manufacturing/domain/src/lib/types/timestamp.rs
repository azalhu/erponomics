use chrono::{DateTime, Utc};

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

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use chrono::Datelike;

    use crate::types::Timestamp;

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
