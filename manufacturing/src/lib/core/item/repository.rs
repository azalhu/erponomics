use std::{future::Future, sync::Arc};

use anyhow::anyhow;
use derive_getters::{Dissolve, Getters};
use derive_more::derive::From;

use crate::{
    entity_tag, id, item,
    sqlx::{Error as SqlxError, SqliteConnection},
    timestamp, Id, Item, ThisError, Timestamp,
};

// MARK: Get

/// `Get` represents a store of item data.
pub trait Get: Send + Sync + 'static {
    /// Get an [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`get::Error::NotFound`] if an [`Item`] with the given [`Id`] does not exist.
    fn get(&self, request: GetRequest) -> impl Future<Output = Result<Item, GetError>> + Send;
}

/// The fields required by the domain to get an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct GetRequest {
    id: Id,
}

#[derive(Debug, ThisError)]
pub enum GetError {
    #[error("item with id not found")]
    NotFound,
    #[error(transparent)]
    Invalid(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<GetError> for item::Error {
    fn from(value: GetError) -> Self {
        match value {
            GetError::Unknown(err) => Self::Unknown(err),
            GetError::Invalid(err) => err,
            GetError::NotFound => Self::Id(id::NotFoundError.into()),
        }
    }
}

impl From<id::Error> for GetError {
    fn from(value: id::Error) -> Self {
        item::Error::Id(value).into()
    }
}

impl From<entity_tag::Error> for GetError {
    fn from(value: entity_tag::Error) -> Self {
        item::Error::Etag(value).into()
    }
}

impl From<timestamp::Error> for GetError {
    fn from(value: timestamp::Error) -> Self {
        item::Error::Timestamp(value).into()
    }
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
        request: ListRequest,
    ) -> impl Future<Output = Result<ListResponse, ListError>> + Send;
}

/// The fields required by the domain to list [Item]s.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct ListRequest {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct ListResponse {
    item: Vec<Item>,
}

#[derive(Debug, ThisError)]
pub enum ListError {
    #[error("item with id not found")]
    NotFound,
    #[error(transparent)]
    Invalid(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<ListError> for item::Error {
    fn from(value: ListError) -> Self {
        match value {
            ListError::Unknown(err) => Self::Unknown(err),
            ListError::Invalid(err) => err,
            ListError::NotFound => Self::Id(id::NotFoundError.into()),
        }
    }
}

impl From<id::Error> for ListError {
    fn from(value: id::Error) -> Self {
        item::Error::Id(value).into()
    }
}

impl From<timestamp::Error> for ListError {
    fn from(value: timestamp::Error) -> Self {
        item::Error::Timestamp(value).into()
    }
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
        request: CreateRequest,
    ) -> impl Future<Output = Result<(), CreateError>> + Send;
}

/// The fields required by the domain to create an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct CreateRequest {
    item: Item,
}

#[derive(Debug, ThisError)]
pub enum CreateError {
    #[error("item with id {id} already exists")]
    Duplicate { id: Id },
    #[error(transparent)]
    InvalidItem(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<CreateError> for item::Error {
    fn from(value: CreateError) -> Self {
        match value {
            CreateError::Unknown(err) => Self::Unknown(err),
            CreateError::Duplicate { id } => Self::Id(id::DuplicateError(id).into()),
            CreateError::InvalidItem(err) => err,
        }
    }
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
        request: UpdateRequest,
    ) -> impl Future<Output = Result<(), UpdateError>> + Send;
}

/// The fields required by the domain to update an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct UpdateRequest {
    item: Item,
}

#[derive(Debug, ThisError)]
pub enum UpdateError {
    #[error(transparent)]
    InvalidItem(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<UpdateError> for item::Error {
    fn from(value: UpdateError) -> Self {
        match value {
            UpdateError::Unknown(err) => Self::Unknown(err),
            UpdateError::InvalidItem(err) => err,
        }
    }
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
        request: DeleteRequest,
    ) -> impl Future<Output = Result<(), DeleteError>> + Send;
}

/// The fields required by the domain to delete an [Item].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters, Dissolve)]
pub struct DeleteRequest {
    id: Id,
}

#[derive(Debug, ThisError)]
pub enum DeleteError {
    #[error("item not found")]
    NotFound,
    #[error(transparent)]
    Invalid(#[from] item::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<id::Error> for DeleteError {
    fn from(value: id::Error) -> Self {
        item::Error::Id(value).into()
    }
}

impl From<DeleteError> for item::Error {
    fn from(value: DeleteError) -> Self {
        match value {
            DeleteError::Unknown(err) => Self::Unknown(err),
            DeleteError::NotFound => Self::Id(id::NotFoundError.into()),
            DeleteError::Invalid(err) => err,
        }
    }
}

// MARK: Service

#[derive(Debug, Clone)]
pub struct ItemRepository<DB: SqliteConnection> {
    db: Arc<DB>,
}

impl<DB> ItemRepository<DB>
where
    DB: SqliteConnection + Clone,
{
    #[must_use]
    pub const fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    #[must_use]
    async fn fetch_item(&self, id: &Id) -> Result<Item, GetError> {
        let id = id.to_string();

        let query = sqlx::query!(
            "SELECT id, display_name, title, description, state, etag, uid, create_time, update_time FROM item WHERE id = $1",
            id
        );

        let result =
            query
                .fetch_one(self.db.pool())
                .await
                .map_err(|e| match SqlxError::from(&e) {
                    SqlxError::RowNotFound => GetError::NotFound,
                    _ => GetError::from(
                        anyhow!(e).context(format!("failed to fetch item with id {id:?}")),
                    ),
                })?;

        let id = Id::try_from(result.id)?;
        let display_name = result.display_name;
        let title = result.title;
        let description = result.description;
        let state = num_traits::FromPrimitive::from_i64(result.state).ok_or(GetError::Invalid(
            item::Error::Unknown(anyhow!(format!("invalid state {0}", result.state))),
        ))?;
        let etag = result.etag.try_into()?;
        let uid = uuid::Uuid::try_parse(&result.uid)
            .map_err(|e| GetError::Invalid(item::Error::Unknown(anyhow!(e))))?;
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
}

impl<DB> Get for ItemRepository<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn get(&self, request: GetRequest) -> Result<Item, GetError> {
        let item = self.fetch_item(&request.id).await?;
        Ok(item)
    }
}

impl<DB> List for ItemRepository<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn list(&self, _request: ListRequest) -> Result<ListResponse, ListError> {
        todo!()
    }
}

impl<DB> Create for ItemRepository<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn create(&self, _request: CreateRequest) -> Result<(), CreateError> {
        todo!()
    }
}

impl<DB> Update for ItemRepository<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn update(&self, _request: UpdateRequest) -> Result<(), UpdateError> {
        todo!()
    }
}

impl<DB> Delete for ItemRepository<DB>
where
    DB: SqliteConnection + Clone,
{
    async fn delete(&self, _request: DeleteRequest) -> Result<(), DeleteError> {
        todo!()
    }
}
