use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::api::v1::types::db::ProgressStatus;

#[derive(Debug)]
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: ProgressStatus /* TODO: unknown type StatusEnum */,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub planned_start_date: Option<NaiveDateTime>,
    pub planned_end_date: Option<NaiveDateTime>,
    pub actual_start_date: Option<NaiveDateTime>,
    pub actual_end_date: Option<NaiveDateTime>,
    pub content: Option<String>,
}

#[derive(serde::Deserialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub user_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub planned_start_date: Option<NaiveDateTime>,
    pub planned_end_date: Option<NaiveDateTime>,
    pub content: String
}
