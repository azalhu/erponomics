use std::sync::Arc;

use manufacturing::proto::item::repository::{
    item_command_service_server::ItemCommandServiceServer,
    item_query_service_server::ItemQueryServiceServer,
};

use crate::{item, sqlx::sqlite::Connection};

pub fn sqlite_client<DB: Connection>(sqlite_connection: Arc<DB>) -> Arc<item::SqliteClient<DB>> {
    Arc::new(item::SqliteClient::new(sqlite_connection))
}

#[must_use]
pub fn command_service<DB: Connection>(
    sqlite_client: Arc<item::SqliteClient<DB>>,
) -> ItemCommandServiceServer<item::GrpcService<item::SqliteClient<DB>>> {
    ItemCommandServiceServer::new(item::GrpcService::new(sqlite_client))
}

#[must_use]
pub fn query_service<DB: Connection>(
    sqlite_client: Arc<item::SqliteClient<DB>>,
) -> ItemQueryServiceServer<item::GrpcService<item::SqliteClient<DB>>> {
    ItemQueryServiceServer::new(item::GrpcService::new(sqlite_client))
}
