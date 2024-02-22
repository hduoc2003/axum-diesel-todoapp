use crate::api::v1::db::database::{get_connection_pool, Pool};

pub async fn config_db() -> Pool {
    get_connection_pool().await
}
