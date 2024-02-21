use axum::{http::StatusCode, Json};
use diesel::ExpressionMethods;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;
use diesel::prelude::*;
use serde_json::json;


use crate::{schema::users::{self, password_hash}, v1::api::{db::database::DatabaseConnection, models::users::User, types::{env::ENV, session::Session}, utils::{env::get_env, errors::{internal_error, ErrorResponse}, hashing::{encrypt_session, verify_password}}}};


#[derive(Debug, Deserialize)]
pub struct SigninRequest {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    jwt: String
}
// -> Result<SigninResponse, ErrorResponse>
pub async fn handle_signin(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(req): Json<SigninRequest>
) -> Result<Json<SigninResponse>, ErrorResponse> {
    let query: Result<(i64, String), diesel::result::Error> = users::table
    .select((users::id, users::password_hash))
    .filter(users::username.eq(&req.username))
    .first::<(i64, String)>(&mut conn)
    // .load::<(i64, String)>(&mut conn)
    .await;
    match query {
        Ok(data) => {
            let user_id = &data.0;
            let pw_hash = &data.1;
            if verify_password(&req.password, pw_hash) {
                return Ok(Json(SigninResponse {
                    jwt: encrypt_session(&Session {
                        user_id: *user_id,
                        exp: get_env(ENV::JWT_EXPIRED_TIME).parse().expect("JWT_EXPIRED_TIME must be number"),
                    }),
                }))
            } else {
                return Err(ErrorResponse {
                    status: StatusCode::UNAUTHORIZED,
                    msg: "wrong password".to_string(),
                })
            }
        },
        Err(e) => {
            return match e {
                diesel::result::Error::NotFound => Err(ErrorResponse {
                    status: StatusCode::NOT_FOUND,
                    msg: format!("username {} does not exist", &req.username)
                    // msg: e.to_string()
                }),
                _ => Err(internal_error(e)),
            }
        }
    }
}
