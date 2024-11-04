use std::convert::TryInto;

use crate::{item, Item};

use super::proto;

impl From<Item> for proto::Item {
    fn from(value: Item) -> Self {
        let (id, code, name, description, created_at) = value.dissolve();

        Self {
            id: id.into(),
            code: code.into(),
            name: name.into(),
            description: description.into(),
            created_at: created_at.into(),
        }
    }
}

impl From<Item> for Option<proto::Item> {
    fn from(value: Item) -> Self {
        Some(value.into())
    }
}

impl TryFrom<proto::Item> for Item {
    type Error = item::Error;

    fn try_from(value: proto::Item) -> Result<Self, Self::Error> {
        let id = value.id.try_into()?;
        let code = value.code.try_into()?;
        let name = value.name.try_into()?;
        let description = value.description.try_into()?;
        let created_at = value.created_at.try_into()?;
        Ok(Self::from((id, code, name, description, created_at)))
    }
}

impl TryFrom<Option<proto::Item>> for Item {
    type Error = item::Error;

    fn try_from(value: Option<proto::Item>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Err(item::EmptyError.into()), TryInto::try_into)
    }
}
