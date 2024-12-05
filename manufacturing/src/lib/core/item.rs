use anyhow::anyhow;
use derive_more::derive::From;
use uuid::Uuid;

use crate::{entity_tag, id, timestamp, EntityTag, Item, ItemState, ThisError, Timestamp};

pub mod command;
pub mod grpc;
pub mod query;
pub mod repository;
pub mod sync;

impl ItemState {
    fn is_transitioning(&self) -> bool {
        matches!(
            self,
            ItemState::Creating
                | ItemState::Updating
                | ItemState::Deleting
                | ItemState::Annihilating
                | ItemState::Blocking
                | ItemState::Unblocking
        )
    }
}

impl Item {
    #[must_use]
    pub(crate) fn new(
        id: String,
        display_name: String,
        title: String,
        description: String,
    ) -> Result<Self, Error> {
        let now = Timestamp::now();

        Ok(Self {
            id: id.try_into()?,
            display_name,
            title,
            description,
            state: ItemState::Creating,
            etag: EntityTag::new(),
            uid: Uuid::new_v4(),
            create_time: now.clone(),
            update_time: now,
        })
    }

    #[must_use]
    pub(crate) fn update(
        self,
        display_name: Option<String>,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<Item, Error> {
        if self.state.is_transitioning() {
            return Err(Error::Unknown(anyhow!("invalid state")));
        }

        Ok(Item {
            id: self.id,
            display_name: display_name.unwrap_or(self.display_name),
            title: title.unwrap_or(self.title),
            description: description.unwrap_or(self.description),
            state: ItemState::Updating,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }

    #[must_use]
    pub(crate) fn delete(self) -> Result<Item, Error> {
        if self.state.is_transitioning() {
            return Err(Error::Unknown(anyhow!("invalid state")));
        }

        Ok(Item {
            id: self.id,
            display_name: self.display_name,
            title: self.title,
            description: self.description,
            state: ItemState::Deleting,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }

    #[must_use]
    pub(crate) fn annihilate(self) -> Result<Item, Error> {
        Ok(Item {
            id: self.id,
            display_name: self.display_name,
            title: self.title,
            description: self.description,
            state: ItemState::Annihilating,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }

    #[must_use]
    pub(crate) fn block(self) -> Result<Item, Error> {
        if self.state.is_transitioning() || self.state == ItemState::Blocked {
            return Err(Error::Unknown(anyhow!("invalid state")));
        }

        Ok(Item {
            id: self.id,
            display_name: self.display_name,
            title: self.title,
            description: self.description,
            state: ItemState::Blocking,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }

    #[must_use]
    pub(crate) fn unblock(self) -> Result<Item, Error> {
        if self.state.is_transitioning() || self.state == ItemState::Active {
            return Err(Error::Unknown(anyhow!("invalid state")));
        }

        Ok(Item {
            id: self.id,
            display_name: self.display_name,
            title: self.title,
            description: self.description,
            state: ItemState::Unblocking,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }

    #[must_use]
    pub(crate) fn active(self) -> Result<Item, Error> {
        if !matches!(
            self.state,
            ItemState::Creating | ItemState::Updating | ItemState::Unblocking,
        ) {
            return Err(Error::Unknown(anyhow!("invalid state")));
        }

        Ok(Item {
            id: self.id,
            display_name: self.display_name,
            title: self.title,
            description: self.description,
            state: ItemState::Active,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }

    #[must_use]
    pub(crate) fn blocked(self) -> Result<Item, Error> {
        if self.state != ItemState::Blocking {
            return Err(Error::Unknown(anyhow!("invalid state")));
        }

        Ok(Item {
            id: self.id,
            display_name: self.display_name,
            title: self.title,
            description: self.description,
            state: ItemState::Blocked,
            etag: EntityTag::new(),
            uid: self.uid,
            create_time: self.create_time,
            update_time: Timestamp::now(),
        })
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Empty(#[from] EmptyError),
    #[error(transparent)]
    Id(#[from] id::Error),
    #[error(transparent)]
    Etag(#[from] entity_tag::Error),
    #[error(transparent)]
    Timestamp(#[from] timestamp::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("item cannot be empty")]
pub struct EmptyError;
