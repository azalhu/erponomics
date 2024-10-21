use thiserror::Error;
use ulid::Ulid;

use crate::utils::date_time_utc::DateTimeUtc;

pub mod create_item;
pub mod delete_item;
pub mod get_item;
pub mod update_item;

pub mod item {
    tonic::include_proto!("db.item");
}

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(Ulid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemNumber(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
