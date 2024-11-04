use std::str::FromStr;

use anyhow::anyhow;
use chrono::DateTime;
use manufacturing::{
    item::repository::{
        get::{Error, Request, Response},
        Get,
    },
    Code, Description, Id, Item, Name,
};
use ulid::Ulid;

use crate::{sqlx::sqlite::Connection, SqlxError};

use super::SqliteClient;

impl<DB> Get for SqliteClient<DB>
where
    DB: Connection,
{
    async fn get(&self, req: Request) -> Result<Response, Error> {
        let item = self.fetch_item(req.id()).await?;
        Ok(Response::new(item))
    }
}

impl<DB> SqliteClient<DB>
where
    DB: Connection,
{
    async fn fetch_item(&self, id: &Id) -> Result<Item, Error> {
        let id = id.to_string();

        let query = sqlx::query!(
            "SELECT id, code, name, description, created_at FROM items WHERE id = $1",
            id
        );

        let res =
            query
                .fetch_one(self.db().pool())
                .await
                .map_err(|e| match SqlxError::from(&e) {
                    SqlxError::RowNotFound => Error::NotFound,
                    _ => Error::from(
                        anyhow!(e).context(format!("failed to fetch item with id {id:?}")),
                    ),
                })?;

        let id = Id::from(Ulid::from_str(&res.id).unwrap());
        let code = Code::from(res.code.to_string());
        let name = Name::from(res.name.to_string());
        let description = Description::from(res.description.to_string());
        let created_at = DateTime::from_str(&res.created_at).unwrap().into();

        let item = Item::from((id, code, name, description, created_at));

        Ok(item)
    }
}
