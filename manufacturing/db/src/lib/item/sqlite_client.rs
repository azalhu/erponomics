use std::sync::Arc;

use crate::sqlx::sqlite::Connection;

use super::SqliteClient;

pub mod create_item;
pub mod delete_item;
pub mod get_item;

impl<DB> SqliteClient<DB>
where
    DB: Connection,
{
    pub(crate) const fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}
