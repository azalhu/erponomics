use derive_getters::{Dissolve, Getters};
use derive_more::From;

use crate::{Code, Description, Id, Name, Timestamp};

pub mod item;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct Item {
    id: Id,
    code: Code,
    name: Name,
    description: Description,
    created_at: Timestamp,
}
