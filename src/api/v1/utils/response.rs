use axum::{response::IntoResponse, Json};
use serde_json::json;

pub struct OkResponse;

impl IntoResponse for OkResponse {
    fn into_response(self) -> axum::response::Response {
        Json(json!({
            "ok": true
        })).into_response()
    }
}
