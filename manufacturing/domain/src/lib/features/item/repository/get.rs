use derive_getters::{Dissolve, Getters};
use derive_more::derive::From;

use crate::{item, Id, Item, ThisError};

pub mod error;
pub mod request;
pub mod response;

/// The fields required by the domain to get an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct Request {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct Response {
    item: Item,
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
