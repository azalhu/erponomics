use derive_more::derive::From;

use crate::{
    code, description, id, name, timestamp, Code, Description, Id, Item, Name, ThisError, Timestamp,
};

pub mod command;
pub mod grpc;
pub mod query;
pub mod repository;
pub mod sync;

impl Item {
    #[must_use]
    pub(crate) fn new(code: Code, name: Name, description: Description) -> Self {
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
    Timestamp(#[from] timestamp::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, ThisError, From)]
#[error("item cannot be empty")]
pub struct EmptyError;
