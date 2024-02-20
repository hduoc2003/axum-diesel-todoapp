use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{config::AppState, v1::api::{db::base::DbBase, utils::errors::ErrorResponse}};

#[derive(Debug, Deserialize)]
struct Request {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
struct Response {

}

pub async fn handle_signup(
    State(mut state): State<AppState>,
    Json(req): Json<Request>
) -> Result<Json<Response>, ErrorResponse> {
    match state.db.add_user(&req.username, &req.password).await {
        Ok(_) => {
            Ok(Json(Response {

            }))
        },
        Err(err) => {
            Err(err)
        },
    }

}
