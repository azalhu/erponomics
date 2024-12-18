use std::{future::Future, sync::Arc};

use anyhow::{anyhow, Context};
use num_traits::ToPrimitive;
use sqlx::{Executor, Sqlite, Transaction};

use crate::{
    id, item,
    sqlx::{Error as SqlxError, SqliteConnection, SqliteError},
    sync::{Operation, OperationMetadata},
    Id, Item, Timestamp,
};

use super::{
    query::{ListRequest, ListResponse},
    sync::Metadata,
    Error,
};

// MARK: Get

/// `Get` represents a store of item data.
pub trait Get: Send + Sync + 'static {
    /// Get an [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`get::Error::NotFound`] if an [`Item`] with the given [`Id`] does not exist.
    fn get(&self, id: &Id) -> impl Future<Output = Result<Item, Error>> + Send;
}

// MARK: List

/// `List` represents a store of item data.
pub trait List: Send + Sync + 'static {
    /// List [`Item`]s.
    ///
    /// # Errors
    ///
    /// - MUST return [`list::Error::NotFound`] if an [`Item`] with the given [`Id`] does not exist.
    fn list(
        &self,
        request: &ListRequest,
    ) -> impl Future<Output = Result<ListResponse, Error>> + Send;
}

// MARK: Create

/// `Create` represents a store of item data.
pub trait Create: Send + Sync + 'static {
    /// Persist a new [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`create::Error::Duplicate`] if an [`Item`] with the same [`Code`] already exists.
    fn create(
        &self,
        operation: &Operation<Metadata>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

// MARK: Update

/// `Update` represents a store of item data.
pub trait Update: Send + Sync + 'static {
    /// Update an [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`update::Error::Duplicate`] if an [`Item`] with the same [`Code`] already exists.
    fn update(
        &self,
        operation: &Operation<Metadata>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

// MARK: Delete

/// `Delete` represents a store of item data.
pub trait Delete: Send + Sync + 'static {
    /// Delete an [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`delete::Error::NotFound`] if an [`Item`] with the given [`Id`] does not exist.
    fn delete(
        &self,
        operation: &Operation<Metadata>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

// MARK: Service

#[derive(Debug, Clone)]
pub struct Service<DB: SqliteConnection> {
    db: Arc<DB>,
}

impl<DB> Service<DB>
where
    DB: SqliteConnection + Clone,
{
    #[must_use]
    pub const fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    async fn fetch_item(&self, id: &Id) -> Result<Item, Error> {
        let id = id.value();

        let query = sqlx::query!(
            "SELECT
                id,
                display_name,
                title,
                description,
                state,
                etag,
                uid,
                create_time,
                update_time
            FROM item WHERE id = $1",
            id
        );

        let result =
            query
                .fetch_one(self.db.pool())
                .await
                .map_err(|e| match SqlxError::from(&e) {
                    SqlxError::RowNotFound => Error::Id(id::NotFoundError.into()),
                    _ => Error::from(
                        anyhow!(e).context(format!("failed to fetch item with id {id:?}")),
                    ),
                })?;

        let id = Id::try_from(result.id)?;
        let display_name = result.display_name;
        let title = result.title;
        let description = result.description;
        let state = num_traits::FromPrimitive::from_i64(result.state).ok_or(Error::Unknown(
            anyhow!(format!("invalid state {0}", result.state)),
        ))?;
        let etag = result.etag.try_into()?;
        let uid =
            uuid::Uuid::try_parse(&result.uid).map_err(|e| item::Error::Unknown(anyhow!(e)))?;
        let create_time = Timestamp::try_from(result.create_time)?;
        let update_time = Timestamp::try_from(result.update_time)?;

        let item = Item::from((
            id,
            display_name,
            title,
            description,
            state,
            etag,
            uid,
            create_time,
            update_time,
        ));

        Ok(item)
    }

    async fn fetch_items(&self, request: &ListRequest) -> Result<ListResponse, Error> {
        let page_size = request.page_size();
        let page_token = request.page_token();
        let order_by = request.order_by();
        let filter = request.filter();

        let query = sqlx::query!(
            "SELECT
                id,
                display_name,
                title,
                description,
                state,
                etag,
                uid,
                create_time,
                update_time
            FROM item WHERE $1
            ORDER BY $2
            LIMIT $3 OFFSET $4",
            filter,
            order_by,
            page_size,
            page_token
        );

        let result =
            query
                .fetch_all(self.db.pool())
                .await
                .map_err(|e| match SqlxError::from(&e) {
                    SqlxError::RowNotFound => Error::Id(id::NotFoundError.into()),
                    _ => Error::from(anyhow!(e).context("failed to fetch items")),
                })?;

        let mut items: Vec<Item> = vec![];

        for r in result {
            let id = Id::try_from(r.id)?;
            let display_name = r.display_name;
            let title = r.title;
            let description = r.description;
            let state = num_traits::FromPrimitive::from_i64(r.state).ok_or(Error::Unknown(
                anyhow!(format!("invalid state {0}", r.state)),
            ))?;
            let etag = r.etag.try_into()?;
            let uid =
                uuid::Uuid::try_parse(&r.uid).map_err(|e| item::Error::Unknown(anyhow!(e)))?;
            let create_time = Timestamp::try_from(r.create_time)?;
            let update_time = Timestamp::try_from(r.update_time)?;

            let item = Item::from((
                id,
                display_name,
                title,
                description,
                state,
                etag,
                uid,
                create_time,
                update_time,
            ));

            items.push(item);
        }

        Ok(ListResponse::new(items, None, 0))
    }

    async fn save_item(&self, tx: &mut Transaction<'_, Sqlite>, item: &Item) -> Result<(), Error> {
        let id = &item.id.value();
        let display_name = &item.display_name;
        let title = &item.title;
        let description = &item.description;
        let state = &item.state.to_i64();
        let etag = &item.etag.to_string();
        let uid = &item.uid.to_string();
        let create_time = &item.create_time.value().to_string();
        let update_time = &item.update_time.value().to_string();

        let query = sqlx::query!(
            "INSERT INTO item (
                id,
                display_name,
                title,
                description,
                state,
                etag,
                uid,
                create_time,
                update_time
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            id,
            display_name,
            title,
            description,
            state,
            etag,
            uid,
            create_time,
            update_time,
        );

        tx.execute(query)
            .await
            .map_err(|e| match SqlxError::from(&e) {
                SqlxError::Sqlite { inner } => match inner {
                    SqliteError::UniqueConstraintViolationCode => {
                        Error::Id(id::DuplicateError(item.id.clone()).into())
                    }
                    SqliteError::Unknown { message } => anyhow!(e)
                        .context(format!(
                            "failed to insert item with id {:?}, with message from database: {:?}",
                            item.id(),
                            message
                        ))
                        .into(),
                },
                SqlxError::RowNotFound | SqlxError::Unknown => anyhow!(e)
                    .context(format!("failed to insert item with id {:?}", item.id()))
                    .into(),
            })?;

        Ok(())
    }

    async fn modify_item(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        item: &Item,
    ) -> Result<(), Error> {
        let id = &item.id.value();
        let display_name = &item.display_name;
        let title = &item.title;
        let description = &item.description;
        let state = &item.state.to_i64();
        let etag = &item.etag.to_string();
        let uid = &item.uid.to_string();
        let create_time = &item.create_time.value().to_string();
        let update_time = &item.update_time.value().to_string();

        let query = sqlx::query!(
            "UPDATE item SET
                id              = $1,
                display_name    = $2,
                title           = $3,
                description     = $4,
                state           = $5,
                etag            = $6,
                uid             = $7,
                create_time     = $8,
                update_time     = $9",
            id,
            display_name,
            title,
            description,
            state,
            etag,
            uid,
            create_time,
            update_time,
        );

        tx.execute(query)
            .await
            .map_err(|e| match SqlxError::from(&e) {
                SqlxError::RowNotFound => Error::Id(id::NotFoundError.into()),
                _ => {
                    Error::from(anyhow!(e).context(format!("failed to delete item with id {id:?}")))
                }
            })?;

        Ok(())
    }

    async fn remove_item(&self, tx: &mut Transaction<'_, Sqlite>, id: &Id) -> Result<(), Error> {
        let id = &id.to_string();

        let query = sqlx::query!("DELETE FROM item WHERE id = $1", id);

        tx.execute(query)
            .await
            .map_err(|e| match SqlxError::from(&e) {
                SqlxError::RowNotFound => Error::Id(id::NotFoundError.into()),
                _ => {
                    Error::from(anyhow!(e).context(format!("failed to delete item with id {id:?}")))
                }
            })?;

        Ok(())
    }
}

impl<DB> Get for Service<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn get(&self, id: &Id) -> Result<Item, Error> {
        let item = self.fetch_item(id).await?;
        Ok(item)
    }
}

impl<DB> List for Service<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn list(&self, request: &ListRequest) -> Result<ListResponse, Error> {
        let response = self.fetch_items(request).await?;
        Ok(response)
    }
}

impl<DB> Create for Service<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn create(&self, operation: &Operation<Metadata>) -> Result<(), Error> {
        let mut tx = self
            .db
            .pool()
            .begin()
            .await
            .with_context(|| "failed to start SQLite transaction")?;

        self.save_item(&mut tx, operation.metadata().entity())
            .await?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(())
    }
}

impl<DB> Update for Service<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn update(&self, operation: &Operation<Metadata>) -> Result<(), Error> {
        let mut tx = self
            .db
            .pool()
            .begin()
            .await
            .with_context(|| "failed to start SQLite transaction")?;

        self.modify_item(&mut tx, operation.metadata().entity())
            .await?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(())
    }
}

impl<DB> Delete for Service<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn delete(&self, operation: &Operation<Metadata>) -> Result<(), Error> {
        let mut tx = self
            .db
            .pool()
            .begin()
            .await
            .with_context(|| "failed to start SQLite transaction")?;

        self.remove_item(&mut tx, operation.metadata().entity().id())
            .await?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(())
    }
}
