use axum::{routing::get, Router};

use crate::{config::AppState, v1::api::controllers::signup::handle_signup};

pub fn routes() -> Router<AppState> {
    Router::new().route("/signup", get(handle_signup))
}
