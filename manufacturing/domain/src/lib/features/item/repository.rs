use std::future::Future;

#[allow(unused_imports)]
use crate::{Code, Id, Item};

pub mod create;
pub mod delete;
pub mod get;
pub mod grpc;
pub mod update;

/// `Create` represents a store of item data.
pub trait Create: Send + Sync + 'static {
    /// Persist a new [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`create::Error::Duplicate`] if an [`Item`] with the same [`Code`] already exists.
    fn create(
        &self,
        req: create::Request,
    ) -> impl Future<Output = Result<(), create::Error>> + Send;
}

/// `Delete` represents a store of item data.
pub trait Delete: Send + Sync + 'static {
    /// Delete an [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`delete::Error::NotFound`] if an [`Item`] with the given [`Id`] does not exist.
    fn delete(
        &self,
        req: delete::Request,
    ) -> impl Future<Output = Result<(), delete::Error>> + Send;
}

/// `Get` represents a store of item data.
pub trait Get: Send + Sync + 'static {
    /// Get an [`Item`].
    ///
    /// # Errors
    ///
    /// - MUST return [`get::Error::NotFound`] if an [`Item`] with the given [`Id`] does not exist.
    fn get(
        &self,
        req: get::Request,
    ) -> impl Future<Output = Result<get::Response, get::Error>> + Send;
}
