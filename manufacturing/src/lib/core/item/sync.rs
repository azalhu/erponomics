use derive_getters::Getters;

use crate::{
    item,
    sync::{OperationEntity, OperationMetadata, OperationState},
    Id, Item, ItemState,
};

impl OperationState for ItemState {
    type NextState = Option<Self>;

    fn next(&self) -> Self::NextState {
        match self {
            Self::Creating | Self::Updating | Self::Unblocking => Some(Self::Active),
            Self::Blocking => Some(Self::Blocked),
            Self::Deleting | Self::Annihilating | Self::Active | Self::Blocked => None,
        }
    }
}

impl OperationEntity for Item {
    type State = ItemState;

    fn id(&self) -> &Id {
        self.id()
    }

    fn state(&self) -> &Self::State {
        self.state()
    }
}

#[derive(Getters)]
pub struct Metadata {
    item: Item,
}

impl Metadata {
    #[must_use]
    pub const fn new(item: Item) -> Self {
        Self { item }
    }
}

impl OperationMetadata for Metadata {
    type Entity = Item;
    type Response = Item;
    type Error = item::Error;

    fn entity(&self) -> &Self::Entity {
        self.item()
    }
}
