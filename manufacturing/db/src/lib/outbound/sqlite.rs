use std::str::FromStr;

use anyhow::{anyhow, Context};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, SqlitePool, Transaction};

use crate::domain::item::models::item::DateTimeUtc;
use crate::domain::item::{
    models::item::{CreateItemError, CreateItemRequest, Item, ItemId, ItemName, ItemNumber},
    ports::ItemRepository,
};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> anyhow::Result<Sqlite> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }

    async fn save_item(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        number: &ItemNumber,
        name: &ItemName,
    ) -> Result<(ItemId, DateTimeUtc), sqlx::Error> {
        let id = ItemId::new();
        let id_as_string = id.to_string();
        let number = &number.to_string();
        let name = &name.to_string();
        let created_at = DateTimeUtc::now();

        let query = sqlx::query!(
            "INSERT INTO items (id, number, name, created_at) VALUES ($1, $2, $3, $4)",
            id_as_string,
            number,
            name,
            created_at,
        );

        tx.execute(query).await?;

        Ok((id, created_at))
    }
}

impl ItemRepository for Sqlite {
    async fn create_item(&self, req: &CreateItemRequest) -> Result<Item, CreateItemError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

        let (id, created_at) = self
            .save_item(&mut tx, req.number(), req.name())
            .await
            .map_err(|e| {
                if is_unique_constraint_violation(&e) {
                    CreateItemError::Duplicate {
                        number: req.number().clone(),
                    }
                } else {
                    anyhow!(e)
                        .context(format!("failed to save item with number {:?}", req.name()))
                        .into()
                }
            })?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(Item::new(
            id,
            req.number().clone(),
            req.name().clone(),
            created_at,
        ))
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}
