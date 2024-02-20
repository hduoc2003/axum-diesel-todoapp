use axum::{http::StatusCode, Json};
use diesel::SelectableHelper;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;


use crate::{schema::users, v1::api::{db::database::DatabaseConnection, models::users::{NewUser, User}, utils::{errors::{internal_error, ErrorResponse}, hashing::hash_password, response::OkResponse}}};

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {

}

pub async fn handle_signup(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(req): Json<SignupRequest>
) -> Result<OkResponse, ErrorResponse> {
    let result = diesel::insert_into(users::table)
        .values(NewUser {
            username: &req.username,
            password_hash: &hash_password(&req.password),
        })
        // .returning(User::as_returning())
        // .get_result(&mut conn)
        .execute(&mut conn)
        .await;
    return match result {
        Ok(_) => Ok(OkResponse),
        Err(e) => {
            match e {
                diesel::result::Error::DatabaseError(db_e, _) => {
                    match db_e {
                        diesel::result::DatabaseErrorKind::UniqueViolation => {
                            return Err(ErrorResponse {
                                status: StatusCode::CONFLICT,
                                msg: format!("username {} exists", req.username),
                            })
                        }
                        _ => Err(internal_error(e)),
                    }
                },
                _ => Err(internal_error(e)),
            }
        },
    }
}
