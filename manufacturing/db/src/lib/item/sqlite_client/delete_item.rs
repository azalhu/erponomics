use anyhow::{anyhow, Context};
use manufacturing::{
    item::repository::{
        delete::{Error, Request},
        Delete,
    },
    Id,
};
use sqlx::{Executor, Sqlite, Transaction};

use crate::{sqlx::sqlite::Connection, SqlxError};

use super::SqliteClient;

impl<DB> Delete for SqliteClient<DB>
where
    DB: Connection,
{
    async fn delete(&self, req: Request) -> Result<(), Error> {
        let mut tx = self
            .db()
            .pool()
            .begin()
            .await
            .with_context(|| "failed to start SQLite transaction")?;

        let id = req.id();

        self.remove_item(&mut tx, id).await?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(())
    }
}

impl<DB> SqliteClient<DB>
where
    DB: Connection,
{
    async fn remove_item(&self, tx: &mut Transaction<'_, Sqlite>, id: &Id) -> Result<(), Error> {
        let id = &id.to_string();

        let query = sqlx::query!("DELETE FROM items WHERE id = $1", id,);

        tx.execute(query)
            .await
            .map_err(|e| match SqlxError::from(&e) {
                SqlxError::RowNotFound => Error::NotFound,
                _ => {
                    Error::from(anyhow!(e).context(format!("failed to delete item with id {id:?}")))
                }
            })?;

        Ok(())
    }
}
