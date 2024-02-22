use crate::api::v1::{db::database::Pool, routes::auth};
use axum::routing::Router;

use super::task;

pub fn get_all_routes() -> Router<Pool> {
    Router::new().nest("/api/v1",
Router::new()
        .merge(auth::router::router())
        .merge(task::router())
    )
}
