use crate::Id;

use super::Request;

impl Request {
    #[must_use]
    pub const fn new(id: Id) -> Self {
        Self { id }
    }
}
