use axum::{Extension, Json};
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
// use diesel::RunQueryDsl;
// use diesel::prelude::*;
use diesel::prelude::*;
// use diesel_demo::*;

use crate::api::v1::utils::errors::internal_error;
use crate::api::v1::utils::response::OkResponse;
use crate::api::v1::{db::database::DatabaseConnection, utils::errors::ErrorResponse};


#[derive(Debug, Deserialize)]
pub struct DelTaskReq {
    task_id: i64,
}

#[derive(Debug, Serialize)]
pub struct DelTaskRes {}

#[tracing::instrument(skip(conn))]
pub async fn handle_delete_task(
    Extension(user_id): Extension<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(req): Json<DelTaskReq>,
) -> Result<OkResponse, ErrorResponse> {
    use crate::schema::tasks::dsl::*;
    let query = diesel::delete(tasks.filter(id.eq(req.task_id)))
        .execute(&mut conn)
        .await;
    match query {
        Ok(_num_deleted) => Ok(OkResponse),
        Err(e) => Err(internal_error(e))
    }
}

