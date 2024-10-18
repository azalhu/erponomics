use chrono::{DateTime, Utc};
use cqrs_es::{Aggregate, DomainEvent};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};
use tonic::async_trait;

pub mod create_item;
pub mod delete_item;
pub mod get_item;
pub mod update_item;
pub mod validate_item;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Item {
    id: ItemId,
    code: String,
    created_at: DateTime<Utc>,
    last_updated_at: DateTime<Utc>,
}

#[async_trait]
impl Aggregate for Item {
    type Command = ItemCommand;
    type Event = ItemEvent;
    type Error = ItemError;
    type Services = ItemServices;

    fn aggregate_type() -> String {
        "item".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ItemEvent::ItemCreated {
                item_id: _,
                created_at,
                last_updated_at,
            } => {
                self.created_at = created_at;
                self.last_updated_at = last_updated_at;
            }
            ItemEvent::ItemUpdated {
                item_id: _,
                last_updated_at,
            } => {
                self.last_updated_at = last_updated_at;
            }
            ItemEvent::ItemDeleted(..) => {}
        };
    }
}

#[derive(Debug)]
pub enum ItemCommand {
    Create,
    Update,
    Delete,
    Validate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemEvent {
    ItemCreated {
        item_id: ItemId,
        created_at: DateTime<Utc>,
        last_updated_at: DateTime<Utc>,
    },
    ItemUpdated {
        item_id: ItemId,
        last_updated_at: DateTime<Utc>,
    },
    ItemDeleted(ItemId),
}

impl DomainEvent for ItemEvent {
    fn event_type(&self) -> String {
        let event_type = match self {
            ItemEvent::ItemCreated { .. } => "ItemCreated",
            ItemEvent::ItemUpdated { .. } => "ItemUpdated",
            ItemEvent::ItemDeleted { .. } => "ItemDeleted",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemId(u32);

impl Deref for ItemId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct ItemError(String);

impl Display for ItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ItemError {}

impl From<&str> for ItemError {
    fn from(message: &str) -> Self {
        Self(message.to_string())
    }
}

pub struct ItemServices;

impl ItemServices {
    async fn create_item(&self, code: String) -> Result<(), ItemError> {
        Ok(())
    }

    async fn update_item(&self, id: ItemId, code: String) -> Result<(), ItemError> {
        Ok(())
    }

    async fn delete_item(&self, id: ItemId) -> Result<(), ItemError> {
        Ok(())
    }

    async fn validate_item(&self, id: ItemId, code: String) -> Result<(), ItemError> {
        Ok(())
    }
}
