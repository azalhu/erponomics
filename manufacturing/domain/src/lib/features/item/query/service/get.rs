use crate::{item::repository, Id, Item};

use super::{Error, Get, Service};

impl<IR> Get for Service<IR>
where
    IR: repository::Get,
{
    async fn get(&self, id: Id) -> Result<Item, Error> {
        let request = id.into();
        let item = self.item_repository.get(request).await?.dissolve();

        Ok(item)
    }
}
