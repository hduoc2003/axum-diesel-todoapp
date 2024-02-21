use axum::Router;
use crate::v1::api::db::database::Pool;

use super::{signin, signup};

pub fn router() -> Router<Pool> {
    Router::new().nest("/auth", Router::new().
        merge(signup::router()).
        merge(signin::router())
    )
}
