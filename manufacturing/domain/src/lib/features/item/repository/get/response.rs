use crate::Item;

use super::Response;

impl Response {
    #[must_use]
    pub const fn new(item: Item) -> Self {
        Self { item }
    }
}
