use derive_more::derive::From;

use crate::{code, description, id, name, Code, Description, Id, Item, Name, ThisError, Timestamp};

pub mod grpc;
pub mod repository;

impl Item {
    #[must_use]
    // TODO pub(crate)
    pub fn new(code: Code, name: Name, description: Description) -> Self {
        Self {
            id: Id::default(),
            code,
            name,
            description,
            created_at: Timestamp::now(),
        }
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Empty(#[from] EmptyError),
    #[error(transparent)]
    Id(#[from] id::Error),
    #[error(transparent)]
    Code(#[from] code::Error),
    #[error(transparent)]
    Name(#[from] name::Error),
    #[error(transparent)]
    Description(#[from] description::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("item cannot be empty")]
pub struct EmptyError;
