use anyhow::{anyhow, Context};
use manufacturing::{
    item::repository::{
        create::{Error, Request},
        Create,
    },
    Item,
};
use sqlx::{Executor, Sqlite, Transaction};

use crate::{sqlx::sqlite::Connection, SqliteError, SqlxError};

use super::SqliteClient;

impl<DB> Create for SqliteClient<DB>
where
    DB: Connection,
{
    async fn create(&self, req: Request) -> Result<(), Error> {
        let mut tx = self
            .db()
            .pool()
            .begin()
            .await
            .with_context(|| "failed to start SQLite transaction")?;

        let item = req.item();

        self.save_item(&mut tx, item).await?;

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
    async fn save_item(&self, tx: &mut Transaction<'_, Sqlite>, item: &Item) -> Result<(), Error> {
        let id = &item.id().to_string();
        let code = &item.code().to_string();
        let name = &item.name().to_string();
        let description = &item.description().to_string();
        let created_at = item.created_at().value().to_string();

        let query = sqlx::query!(
            "INSERT INTO items (
                id,
                code,
                name,
                description,
                created_at
            ) VALUES ($1, $2, $3, $4, $5)",
            id,
            code,
            name,
            description,
            created_at,
        );

        tx.execute(query)
            .await
            .map_err(|e| match SqlxError::from(&e) {
                SqlxError::Sqlite { inner } => match inner {
                    SqliteError::UniqueConstraintViolationCode => Error::Duplicate {
                        code: item.code().clone(),
                    },
                    SqliteError::Unknown { message } => anyhow!(e)
                        .context(format!(
                            "failed to save item with code {:?}, with message from database: {:?}",
                            item.code(),
                            message
                        ))
                        .into(),
                },
                SqlxError::RowNotFound | SqlxError::Unknown => anyhow!(e)
                    .context(format!("failed to save item with code {:?}", item.code()))
                    .into(),
            })?;

        Ok(())
    }
}
