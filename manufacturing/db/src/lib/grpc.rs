use std::sync::Arc;

use tonic::transport::server::Router;

use crate::Connection;

pub mod item;

pub trait ItemServices<DB: Connection> {
    fn add_item_services(self, db_connection: Arc<DB>) -> Router;
}
