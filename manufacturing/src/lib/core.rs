use derive_getters::{Dissolve, Getters};
use derive_more::From;
use uuid::Uuid;

use crate::{EntityTag, Id, Timestamp};

pub mod item;
pub mod sync;

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum ItemState {
    Creating = 1,
    Updating = 2,
    Deleting = 3,
    Annihilating = 4,
    Blocking = 5,
    Unblocking = 6,
    Active = 10,
    Blocked = 11,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct Item {
    id: Id,
    display_name: String,
    title: String,
    description: String,
    state: ItemState,
    etag: EntityTag,
    uid: Uuid,
    create_time: Timestamp,
    update_time: Timestamp,
}
