use std::sync::Arc;

use manufacturing::proto::item::repository::item_service_server::ItemServiceServer;

use crate::{item, sqlx::sqlite::Connection};

pub fn item_service<DB: Connection>(
    sqlite_client: Arc<DB>,
) -> ItemServiceServer<item::GrpcService<item::SqliteClient<DB>>> {
    let item_sqlite_client = Arc::new(item::SqliteClient::new(sqlite_client));

    ItemServiceServer::new(item::GrpcService::new(item_sqlite_client))
}
