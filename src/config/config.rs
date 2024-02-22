use axum::Router;

use super::{db_config::config_db, env_config, log_config::config_log, routes_config::config_routes};

pub async fn config() -> Router {
    env_config::config_env().await;
    let connection_pool = config_db().await;
    let app = config_routes(connection_pool);
    config_log();
    return app;
}
