use anyhow::anyhow;
use uuid::Uuid;

pub use super::Error;

use std::{future::Future, sync::Arc};

use crate::{id, Id, Item};

use super::repository::{self, GetError};

// MARK: Create

pub trait Create: Send + Sync + 'static {
    fn create(&self, request: CreateRequest) -> impl Future<Output = Result<(), Error>> + Send;
}

pub struct CreateRequest {
    id: Option<String>,
    display_name: String,
    title: String,
    description: String,
}

impl CreateRequest {
    #[must_use]
    pub const fn new(
        id: Option<String>,
        display_name: String,
        title: String,
        description: String,
    ) -> Self {
        Self {
            id,
            display_name,
            title,
            description,
        }
    }
}

// MARK: Update

pub trait Update: Send + Sync + 'static {
    fn update(&self, request: UpdateRequest) -> impl Future<Output = Result<(), Error>> + Send;
}

pub struct UpdateRequest {
    id: String,
    display_name: Option<String>,
    title: Option<String>,
    description: Option<String>,
    etag: String,
}

impl UpdateRequest {
    #[must_use]
    pub const fn new(
        id: String,
        display_name: Option<String>,
        title: Option<String>,
        description: Option<String>,
        etag: String,
    ) -> Self {
        Self {
            id,
            display_name,
            title,
            description,
            etag,
        }
    }
}

// MARK: Delete

pub trait Delete: Send + Sync + 'static {
    fn delete(&self, request: DeleteRequest) -> impl Future<Output = Result<(), Error>> + Send;
}

pub struct DeleteRequest {
    id: String,
    etag: String,
}

impl DeleteRequest {
    #[must_use]
    pub const fn new(id: String, etag: String) -> Self {
        Self { id, etag }
    }
}

// MARK: Annihilate

pub trait Annihilate: Send + Sync + 'static {
    fn annihilate(
        &self,
        request: AnnihilateRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

pub struct AnnihilateRequest {
    id: String,
    etag: String,
}

impl AnnihilateRequest {
    #[must_use]
    pub const fn new(id: String, etag: String) -> Self {
        Self { id, etag }
    }
}

// MARK: Block

pub trait Block: Send + Sync + 'static {
    fn block(&self, request: BlockRequest) -> impl Future<Output = Result<(), Error>> + Send;
}

pub struct BlockRequest {
    id: String,
    etag: String,
}

impl BlockRequest {
    #[must_use]
    pub const fn new(id: String, etag: String) -> Self {
        Self { id, etag }
    }
}

// MARK: Unblock

pub trait Unblock: Send + Sync + 'static {
    fn unblock(&self, request: UnblockRequest) -> impl Future<Output = Result<(), Error>> + Send;
}

pub struct UnblockRequest {
    id: String,
    etag: String,
}

impl UnblockRequest {
    #[must_use]
    pub const fn new(id: String, etag: String) -> Self {
        Self { id, etag }
    }
}

// MARK: Service

#[derive(Debug)]
pub struct Service<
    IR: repository::Create + repository::Update + repository::Get + repository::Delete,
> {
    item_repository: Arc<IR>,
}

impl<IR> Service<IR>
where
    IR: repository::Get + repository::Create + repository::Update + repository::Delete,
{
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }

    async fn validate_create_request(&self, request: CreateRequest) -> Result<Item, Error> {
        let id = match request.id {
            Some(id) => match Id::try_from(id) {
                Ok(id) => match self.item_repository.get(id.clone().into()).await {
                    Ok(item) => return Err(Error::Id(id::DuplicateError(item.id).into())),
                    Err(err) => match err {
                        GetError::NotFound => id.to_string(),
                        _ => return Err(err.into()),
                    },
                },
                Err(_) => Uuid::new_v4().to_string(),
            },
            _ => Uuid::new_v4().to_string(),
        };

        Ok(Item::new(
            id,
            request.display_name,
            request.title,
            request.description,
        )?)
    }

    async fn validate_update_request(&self, request: UpdateRequest) -> Result<Item, Error> {
        let id = Id::try_from(request.id)?;
        let response = self.item_repository.get(id.into()).await?;

        if request.etag != response.etag.to_string() {
            return Err(Error::Id(crate::id::EmptyError.into()));
        }

        Ok(response.update(request.display_name, request.title, request.description)?)
    }

    async fn validate_delete_request(&self, request: DeleteRequest) -> Result<Item, Error> {
        let id = Id::try_from(request.id)?;
        let response = self.item_repository.get(id.into()).await?;

        if request.etag != response.etag.to_string() {
            return Err(Error::Id(crate::id::EmptyError.into()));
        }

        Ok(response.delete()?)
    }

    async fn validate_annihilate_request(&self, request: AnnihilateRequest) -> Result<Item, Error> {
        let id = Id::try_from(request.id)?;
        let response = self.item_repository.get(id.into()).await?;

        if request.etag != response.etag.to_string() {
            return Err(Error::Id(crate::id::EmptyError.into()));
        }

        Ok(response.annihilate()?)
    }

    async fn validate_block_request(&self, request: BlockRequest) -> Result<Item, Error> {
        let id = Id::try_from(request.id)?;
        let response = self.item_repository.get(id.into()).await?;

        if request.etag != response.etag.to_string() {
            return Err(Error::Unknown(anyhow!("invalid etag")));
        }

        Ok(response.block()?)
    }

    async fn validate_unblock_request(&self, request: UnblockRequest) -> Result<Item, Error> {
        let id = Id::try_from(request.id)?;
        let response = self.item_repository.get(id.into()).await?;

        if request.etag != response.etag.to_string() {
            return Err(Error::Unknown(anyhow!("invalid etag")));
        }

        Ok(response.unblock()?)
    }
}

impl<IR> Create for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete,
{
    async fn create(&self, request: CreateRequest) -> Result<(), Error> {
        let item = self.validate_create_request(request).await?;
        let request = item.into();
        self.item_repository.create(request).await?;

        // TODO sync after save

        Ok(())
    }
}

impl<IR> Update for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete,
{
    async fn update(&self, request: UpdateRequest) -> Result<(), Error> {
        let item = self.validate_update_request(request).await?;
        let request = item.into();
        self.item_repository.update(request).await?;

        // TODO sync after save

        Ok(())
    }
}

impl<IR> Delete for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete,
{
    async fn delete(&self, request: DeleteRequest) -> Result<(), Error> {
        let item = self.validate_delete_request(request).await?;
        let request = item.into();
        self.item_repository.update(request).await?;

        // TODO sync after delete

        Ok(())
    }
}

impl<IR> Annihilate for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete,
{
    async fn annihilate(&self, request: AnnihilateRequest) -> Result<(), Error> {
        let item = self.validate_annihilate_request(request).await?;
        let request = item.into();
        self.item_repository.update(request).await?;

        // TODO sync after delete

        Ok(())
    }
}

impl<IR> Block for Service<IR>
where
    IR: repository::Get + repository::Create + repository::Update + repository::Delete,
{
    async fn block(&self, request: BlockRequest) -> Result<(), Error> {
        let item = self.validate_block_request(request).await?;
        let request = item.into();
        self.item_repository.update(request).await?;

        // TODO sync after block

        Ok(())
    }
}

impl<IR> Unblock for Service<IR>
where
    IR: repository::Get + repository::Create + repository::Update + repository::Delete,
{
    async fn unblock(&self, request: UnblockRequest) -> Result<(), Error> {
        let item = self.validate_unblock_request(request).await?;
        let request = item.into();
        self.item_repository.update(request).await?;

        // TODO sync after block

        Ok(())
    }
}
