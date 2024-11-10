use anyhow::anyhow;
use manufacturing::{
    item::repository::{
        get::{Error, Request, Response},
        Get,
    },
    Code, Description, Id, Item, Name, Timestamp,
};

use crate::{sqlx::sqlite::Connection, SqlxError};

use super::SqliteClient;

impl<DB> Get for SqliteClient<DB>
where
    DB: Connection,
{
    async fn get(&self, req: Request) -> Result<Response, Error> {
        let item = self.fetch_item(req.id()).await?;
        Ok(Response::from(item))
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

        let id = Id::try_from(res.id)?;
        let code = Code::try_from(res.code)?;
        let name = Name::try_from(res.name)?;
        let description = Description::try_from(res.description)?;
        let created_at = Timestamp::try_from(res.created_at)?;

        let item = Item::from((id, code, name, description, created_at));

        Ok(item)
    }
}
