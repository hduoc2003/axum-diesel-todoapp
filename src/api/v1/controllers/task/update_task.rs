use chrono::NaiveDateTime;
use serde::Deserialize;
use axum::{Extension, Json};

use diesel_async::RunQueryDsl;
// use diesel::prelude::*;
use diesel::prelude::*;
// use diesel_demo::*;

use crate::api::v1::types::db::ProgressStatus;
use crate::api::v1::utils::errors::internal_error;
use crate::api::v1::utils::response::OkResponse;
use crate::api::v1::{db::database::DatabaseConnection, utils::errors::ErrorResponse};
use crate::schema::tasks;

#[derive(Queryable, AsChangeset)]
#[derive(Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdTaskReq {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<ProgressStatus>, /* TODO: unknown type StatusEnum */
    pub updated_at: Option<NaiveDateTime>,
    pub planned_start_date: Option<NaiveDateTime>,
    pub planned_end_date: Option<NaiveDateTime>,
    pub actual_start_date: Option<NaiveDateTime>,
    pub actual_end_date: Option<NaiveDateTime>,
    pub content: Option<String>,
}

pub async fn handle_update_task(
    Extension(user_id): Extension<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(data): Json<UpdTaskReq>
) -> Result<OkResponse, ErrorResponse> {
    match diesel::update(tasks::table).set(data).execute(&mut conn).await {
        Ok(_) => Ok(OkResponse),
        Err(e) => Err(internal_error(e)),
    }
}
