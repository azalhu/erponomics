use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use derive_more::derive::From;
use uuid::Uuid;

use crate::{EntityTag, ThisError};

impl EntityTag {
    pub fn new() -> Self {
        let tag = Uuid::new_v4().to_string();
        let etag = etag::EntityTag::weak(&tag);
        Self(etag)
    }
}

impl Ord for EntityTag {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weak.cmp(&other.weak) {
            Ordering::Equal => self.tag().cmp(other.tag()),
            ordering => ordering,
        }
    }
}

impl PartialOrd for EntityTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for EntityTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.weak.hash(state);
        self.tag().hash(state);
    }
}

impl From<EntityTag> for String {
    fn from(value: EntityTag) -> Self {
        value.to_string()
    }
}

impl From<EntityTag> for Option<String> {
    fn from(value: EntityTag) -> Self {
        Some(value.into())
    }
}

impl TryFrom<String> for EntityTag {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(EmptyError.into())
        } else {
            Ok(Self(::etag::EntityTag::new(
                trimmed.starts_with("W"),
                trimmed,
            )))
        }
    }
}

impl TryFrom<Option<String>> for EntityTag {
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
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("name cannot be empty")]
pub struct EmptyError;
