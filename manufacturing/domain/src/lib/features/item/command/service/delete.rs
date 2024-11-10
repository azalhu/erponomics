use crate::{item::repository, Id};

use super::{Delete, Error, Service};

impl<IR> Delete for Service<IR>
where
    IR: repository::Create + repository::Get + repository::Delete,
{
    async fn delete(&self, id: Id) -> Result<(), Error> {
        let request = id.into();

        // TODO validate that item can be deleted
        // TODO sync before delete

        self.item_repository.delete(request).await?;

        Ok(())
    }
}
