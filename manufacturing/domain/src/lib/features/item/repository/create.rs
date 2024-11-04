use derive_getters::{Dissolve, Getters};
use derive_more::derive::From;

use crate::{item, Code, Item, ThisError};

pub mod request;

/// The fields required by the domain to create an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct Request {
    item: Item,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("item with code {code} already exists")]
    Duplicate { code: Code },
    #[error(transparent)]
    InvalidItem(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
