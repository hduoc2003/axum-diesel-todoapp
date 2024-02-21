use axum::{routing::post, Router};

use crate::api::v1::{controllers::signin::handle_signin, db::database::Pool};

pub fn router() -> Router<Pool> {
    Router::new().route("/signin", post(handle_signin))
}
