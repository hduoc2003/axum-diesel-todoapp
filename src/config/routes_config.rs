use axum::Router;
use tower_http::trace::TraceLayer;

use crate::api::v1::{
    db::database::Pool,
    routes::all_routes::get_all_routes,
};

pub fn config_routes(connection_pool: Pool) -> Router {
    get_all_routes()
        .layer(TraceLayer::new_for_http())
        .with_state(connection_pool)
}
