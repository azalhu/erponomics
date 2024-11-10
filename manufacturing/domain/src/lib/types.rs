use chrono::{DateTime, Utc};
use derive_getters::Dissolve;
use derive_more::derive::{Display, From};
use ulid::Ulid;

pub mod code;
pub mod description;
pub mod grpc;
pub mod id;
pub mod name;
pub mod timestamp;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From)]
pub struct Id(Ulid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Code(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Name(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Description(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From, Dissolve)]
pub struct Timestamp(DateTime<Utc>);
