use chrono::{DateTime, Utc};
use derive_getters::Dissolve;
use derive_more::derive::{Deref, Display, From};

pub mod entity_tag;
pub mod grpc;
pub mod id;
pub mod timestamp;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref)]
pub struct Id(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From, Dissolve)]
pub struct Timestamp(DateTime<Utc>);

#[derive(Clone, Debug, PartialEq, Eq, Display, From, Deref)]
pub struct EntityTag(etag::EntityTag);
