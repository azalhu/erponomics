use crate::{id, item};

use super::Error;

impl From<id::Error> for Error {
    fn from(value: id::Error) -> Self {
        item::Error::Id(value).into()
    }
}
