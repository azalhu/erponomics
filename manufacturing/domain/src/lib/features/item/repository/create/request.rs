use crate::Item;

use super::Request;

impl Request {
    #[must_use]
    pub const fn new(item: Item) -> Self {
        Self { item }
    }
}
