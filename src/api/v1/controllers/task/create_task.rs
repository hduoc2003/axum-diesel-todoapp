use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;
use diesel::prelude::*;


use crate::api::v1::models::tasks::Task;
use crate::api::v1::models::users;
use crate::api::v1::utils::errors::internal_error;
use crate::api::v1::{
    db::database::DatabaseConnection, models::tasks::NewTask, utils::errors::ErrorResponse,
};
use crate::schema::tasks;

#[derive(Debug, Deserialize)]
// pub struct CreateTaskReq(NewTask);
#[derive(Serialize)]
pub struct CreateTaskRes {
    task_id: i64,
}
// -> Result<Json<CreateTaskRes>, ErrorResponse>
#[tracing::instrument(skip(conn))]
pub async fn handle_create_task(
    Extension(user_id): Extension<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(mut new_task): Json<NewTask>,
) -> Result<Json<CreateTaskRes>, ErrorResponse> {
    new_task.user_id = Some(user_id);
    let res = diesel::insert_into(tasks::table)
        .values(new_task)
        .returning(Task::as_returning())
        .get_result::<Task>(&mut conn)
        .await;
    return match res {
        Ok(task) => {
            println!("{:?}", task);
            Ok(Json(CreateTaskRes {
                task_id: task.id
            }))
        },
        Err(e) => Err(internal_error(e)),
    };
}
