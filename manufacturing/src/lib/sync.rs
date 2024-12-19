use derive_getters::{Dissolve, Getters};

use crate::Id;

pub trait OperationState {
    type NextState;

    fn next(&self) -> Self::NextState;
}

pub struct OperationPending;
pub struct OperationInProgress;
pub struct OperationDone;

impl OperationState for OperationPending {
    type NextState = OperationInProgress;

    fn next(&self) -> Self::NextState {
        OperationInProgress
    }
}

impl OperationState for OperationInProgress {
    type NextState = OperationDone;

    fn next(&self) -> Self::NextState {
        OperationDone
    }
}

impl OperationState for OperationDone {
    type NextState = ();

    fn next(&self) -> Self::NextState {}
}

pub trait OperationEntity {
    type State: OperationState;

    fn id(&self) -> &Id;
    fn state(&self) -> &Self::State;
}

pub trait OperationMetadata {
    type Entity: OperationEntity;
    type Response;
    type Error;

    fn entity(&self) -> &Self::Entity;
}

#[derive(Dissolve, Getters)]
pub struct Operation<T: OperationMetadata> {
    id: Id,
    metadata: T,
    result: Option<Result<T::Response, T::Error>>,
}

impl<T: OperationMetadata> Operation<T> {
    #[must_use]
    pub const fn new(id: Id, metadata: T, result: Option<Result<T::Response, T::Error>>) -> Self {
        Self {
            id,
            metadata,
            result,
        }
    }
}
