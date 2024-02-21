use axum::{routing::post, Router};

use crate::api::v1::{controllers::signup::handle_signup, db::database::Pool};

pub fn router() -> Router<Pool> {
    Router::new().route("/signup", post(handle_signup))
}
