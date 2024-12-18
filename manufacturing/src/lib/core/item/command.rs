use anyhow::anyhow;
use uuid::Uuid;

pub use super::Error;
use super::{repository, sync::Metadata};

use std::{future::Future, sync::Arc};

use crate::{id, sync::Operation, Id, Item};

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

#[derive(Debug, Clone)]
pub struct Service<
    IR: repository::Create + repository::Update + repository::Get + repository::Delete + Clone,
> {
    item_repository: Arc<IR>,
}

impl<IR> Service<IR>
where
    IR: repository::Get + repository::Create + repository::Update + repository::Delete + Clone,
{
    #[must_use]
    pub const fn new(item_repository: Arc<IR>) -> Self {
        Self { item_repository }
    }

    async fn validate_create_request(
        &self,
        request: CreateRequest,
    ) -> Result<Operation<Metadata>, Error> {
        let id = match request.id {
            Some(id) => match Id::try_from(id) {
                Ok(id) => match self.item_repository.get(&id).await {
                    Ok(item) => return Err(Error::Id(id::DuplicateError(item.id).into())),
                    Err(err) => match err {
                        Error::Id(id::Error::NotFound(_)) => id.to_string(),
                        _ => return Err(err),
                    },
                },
                Err(err) => return Err(Error::Id(err)),
            },
            _ => Uuid::new_v4().to_string(),
        };

        let item = Item::new(id, request.display_name, request.title, request.description)?;

        Ok(Operation::new(Id::new(), Metadata::new(item), None))
    }

    async fn validate_update_request(
        &self,
        request: UpdateRequest,
    ) -> Result<Operation<Metadata>, Error> {
        let id = Id::try_from(request.id)?;
        let item = self.item_repository.get(&id).await?;

        if request.etag != item.etag.to_string() {
            return Err(Error::Id(crate::id::EmptyError.into()));
        }

        let item = item.update(request.display_name, request.title, request.description)?;

        Ok(Operation::new(Id::new(), Metadata::new(item), None))
    }

    async fn validate_delete_request(
        &self,
        request: DeleteRequest,
    ) -> Result<Operation<Metadata>, Error> {
        let id = Id::try_from(request.id)?;
        let item = self.item_repository.get(&id).await?;

        if request.etag != item.etag.to_string() {
            return Err(Error::Id(crate::id::EmptyError.into()));
        }

        let item = item.delete()?;

        Ok(Operation::new(Id::new(), Metadata::new(item), None))
    }

    async fn validate_annihilate_request(
        &self,
        request: AnnihilateRequest,
    ) -> Result<Operation<Metadata>, Error> {
        let id = Id::try_from(request.id)?;
        let item = self.item_repository.get(&id).await?;

        if request.etag != item.etag.to_string() {
            return Err(Error::Id(crate::id::EmptyError.into()));
        }

        let item = item.annihilate()?;

        Ok(Operation::new(Id::new(), Metadata::new(item), None))
    }

    async fn validate_block_request(
        &self,
        request: BlockRequest,
    ) -> Result<Operation<Metadata>, Error> {
        let id = Id::try_from(request.id)?;
        let item = self.item_repository.get(&id).await?;

        if request.etag != item.etag.to_string() {
            return Err(Error::Unknown(anyhow!("invalid etag")));
        }

        let item = item.block()?;

        Ok(Operation::new(Id::new(), Metadata::new(item), None))
    }

    async fn validate_unblock_request(
        &self,
        request: UnblockRequest,
    ) -> Result<Operation<Metadata>, Error> {
        let id = Id::try_from(request.id)?;
        let item = self.item_repository.get(&id).await?;

        if request.etag != item.etag.to_string() {
            return Err(Error::Unknown(anyhow!("invalid etag")));
        }

        let item = item.unblock()?;

        Ok(Operation::new(Id::new(), Metadata::new(item), None))
    }
}

impl<IR> Create for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete + Clone,
{
    async fn create(&self, request: CreateRequest) -> Result<(), Error> {
        let operation = self.validate_create_request(request).await?;
        self.item_repository.create(&operation).await?;

        // TODO sync after create

        Ok(())
    }
}

impl<IR> Update for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete + Clone,
{
    async fn update(&self, request: UpdateRequest) -> Result<(), Error> {
        let operation = self.validate_update_request(request).await?;
        self.item_repository.update(&operation).await?;

        // TODO sync after update

        Ok(())
    }
}

impl<IR> Delete for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete + Clone,
{
    async fn delete(&self, request: DeleteRequest) -> Result<(), Error> {
        let operation = self.validate_delete_request(request).await?;
        self.item_repository.update(&operation).await?;

        // TODO sync after delete

        Ok(())
    }
}

impl<IR> Annihilate for Service<IR>
where
    IR: repository::Create + repository::Update + repository::Get + repository::Delete + Clone,
{
    async fn annihilate(&self, request: AnnihilateRequest) -> Result<(), Error> {
        let operation = self.validate_annihilate_request(request).await?;
        self.item_repository.update(&operation).await?;

        // TODO sync after annihilate

        Ok(())
    }
}

impl<IR> Block for Service<IR>
where
    IR: repository::Get + repository::Create + repository::Update + repository::Delete + Clone,
{
    async fn block(&self, request: BlockRequest) -> Result<(), Error> {
        let operation = self.validate_block_request(request).await?;
        self.item_repository.update(&operation).await?;

        // TODO sync after block

        Ok(())
    }
}

impl<IR> Unblock for Service<IR>
where
    IR: repository::Get + repository::Create + repository::Update + repository::Delete + Clone,
{
    async fn unblock(&self, request: UnblockRequest) -> Result<(), Error> {
        let operation = self.validate_unblock_request(request).await?;
        self.item_repository.update(&operation).await?;

        // TODO sync after unblock

        Ok(())
    }
}

//#[cfg(test)]
//mod tests {
//    use std::sync::Mutex;
//
//    use crate::{EntityTag, ItemState, Timestamp};
//
//    use super::*;
//
//    impl Default for Item {
//        fn default() -> Self {
//            Self {
//                id: Id::new(),
//                display_name: String::default(),
//                title: String::default(),
//                description: String::default(),
//                state: ItemState::Creating,
//                etag: EntityTag::new(),
//                uid: Uuid::new_v4(),
//                create_time: Timestamp::now(),
//                update_time: Timestamp::now(),
//            }
//        }
//    }
//
//    struct ItemRepositoryMock {
//        item: Arc<Mutex<Item>>,
//        get_item: Option<Mutex<Box<dyn FnMut(&Id) -> Result<Item, Error> + Send + Sync + 'static>>>,
//        create_item:
//            Option<Mutex<Box<dyn FnMut(&Item) -> Result<(), Error> + Send + Sync + 'static>>>,
//    }
//
//    impl repository::Get for ItemRepositoryMock {
//        async fn get(&self, id: &Id) -> Result<Item, Error> {
//            match &self.get_item {
//                Some(f) => (f.lock().unwrap())(id),
//                None => panic!(),
//            }
//        }
//    }
//
//    impl repository::Create for ItemRepositoryMock {
//        async fn create(&self, item: &Item) -> Result<(), Error> {
//            match &self.create_item {
//                Some(f) => (f.lock().unwrap())(item),
//                None => panic!(),
//            }
//        }
//    }
//
//    impl repository::Update for ItemRepositoryMock {
//        async fn update(&self, _item: &Item) -> Result<(), Error> {
//            panic!()
//        }
//    }
//
//    impl repository::Delete for ItemRepositoryMock {
//        async fn delete(&self, _id: &Id) -> Result<(), Error> {
//            panic!()
//        }
//    }
//
//    impl Clone for ItemRepositoryMock {
//        fn clone(&self) -> Self {
//            panic!()
//        }
//    }
//
//    #[tokio::test]
//    async fn create_item_with_id_from_request() -> anyhow::Result<()> {
//        // Arrange
//        let id = String::from("BETAMAX");
//        let request = CreateRequest {
//            id: id.clone().into(),
//            display_name: String::new(),
//            title: String::new(),
//            description: String::new(),
//        };
//
//        let mut expected_item = Arc::new(Mutex::new(Item::default()));
//
//        let get_item = |_| Err(Error::Id(id::NotFoundError.into()));
//
//        let create_item = |item: &Item| {
//            expected_item.lock().unwrap().id = item.id;
//
//            Ok(())
//        };
//
//        let item_repository_mock = Arc::new(ItemRepositoryMock {
//            get_item: Some(Mutex::new(Box::new(get_item))),
//            create_item: Some(Mutex::new(Box::new(create_item))),
//        });
//
//        let sut = Service::new(item_repository_mock);
//
//        // Act
//        sut.create(request).await?;
//
//        // Assert
//        assert_eq!(id, expected_item.lock().unwrap().id.to_string());
//        Ok(())
//    }
//}
