use derive_getters::{Dissolve, Getters};
use derive_more::derive::From;

#[allow(unused_imports)]
use crate::Item;
use crate::{item, Id, ThisError};

pub mod error;

/// The fields required by the domain to delete an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct Request {
    id: Id,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("item with id not found")]
    NotFound,
    #[error(transparent)]
    Invalid(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
