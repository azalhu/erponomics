use crate::{item::repository, Item};

use super::{Create, Error, Service};

impl<IR> Create for Service<IR>
where
    IR: repository::Create + repository::Get + repository::Delete,
{
    async fn create(&self, item: Item) -> Result<(), Error> {
        let request = item.into();

        // TODO validate uniqueness of code
        // TODO sync before save

        self.item_repository.create(request).await?;

        Ok(())
    }
}
