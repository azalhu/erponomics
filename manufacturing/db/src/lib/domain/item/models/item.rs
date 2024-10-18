use std::time::SystemTime;

use chrono::{DateTime, Utc};
use derive_more::derive::{Display, From};
use sqlx::{sqlite::SqliteTypeInfo, Sqlite, Type};
use thiserror::Error;
use ulid::Ulid;

/// A uniquely identifiable item.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    id: ItemId,
    number: ItemNumber,
    name: ItemName,
    created_at: DateTimeUtc,
}

impl Item {
    pub fn new(id: ItemId, number: ItemNumber, name: ItemName, created_at: DateTimeUtc) -> Self {
        Self {
            id,
            number,
            name,
            created_at,
        }
    }

    pub fn id(&self) -> &ItemId {
        &self.id
    }

    pub fn number(&self) -> &ItemNumber {
        &self.number
    }

    pub fn name(&self) -> &ItemName {
        &self.name
    }

    pub fn created_at(&self) -> &DateTimeUtc {
        &self.created_at
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct ItemId(Ulid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct ItemNumber(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct ItemName(String);

#[derive(Clone, Debug, Error)]
#[error("item number cannot be empty")]
pub struct ItemNumberEmptyError;

#[derive(Clone, Debug, Error)]
#[error("item name cannot be empty")]
pub struct ItemNameEmptyError;

impl ItemId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl ItemNumber {
    pub fn new(raw: &str) -> Result<Self, ItemNumberEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(ItemNumberEmptyError)
        } else {
            Ok(ItemNumber(trimmed.to_string()))
        }
    }
}

impl ItemName {
    pub fn new(raw: &str) -> Result<Self, ItemNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(ItemNameEmptyError)
        } else {
            Ok(ItemName(trimmed.to_string()))
        }
    }
}

/// The fields required by the domain to create an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateItemRequest {
    number: ItemNumber,
    name: ItemName,
}

impl CreateItemRequest {
    pub fn new(number: ItemNumber, name: ItemName) -> Self {
        Self { number, name }
    }

    pub fn number(&self) -> &ItemNumber {
        &self.number
    }

    pub fn name(&self) -> &ItemName {
        &self.name
    }
}

#[derive(Debug, Error)]
pub enum CreateItemError {
    #[error("item with number {number} already exists")]
    Duplicate { number: ItemNumber },
    #[error(transparent)]
    EmptyNumber(#[from] ItemNumberEmptyError),
    #[error(transparent)]
    EmptyName(#[from] ItemNameEmptyError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Type)]
pub struct DateTimeUtc(DateTime<Utc>);

impl DateTimeUtc {
    pub fn now() -> DateTimeUtc {
        Utc::now().into()
    }
}

impl Type<Sqlite> for DateTimeUtc {
    fn type_info() -> SqliteTypeInfo {
        DateTime::<Utc>::type_info()
    }
}

impl From<DateTimeUtc> for SystemTime {
    fn from(value: DateTimeUtc) -> SystemTime {
        value.0.into()
    }
}
