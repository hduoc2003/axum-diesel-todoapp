use crate::api::v1::{
    controllers::task::{create_task::handle_create_task, delete_task::handle_delete_task, update_task::handle_update_task},
    db::database::Pool,
    middlewares::auth::authentication,
};
use axum::{
    middleware,
    routing::{delete, patch, post},
    Router,
};
use tower::ServiceBuilder;

pub fn router() -> Router<Pool> {
    Router::new().nest(
        "/task",
        Router::new()
            .route("/create", post(handle_create_task))
            .route("/delete", delete(handle_delete_task))
            .route("/update", patch(handle_update_task))
            .layer(middleware::from_fn(authentication)),
    )
}
