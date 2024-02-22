use axum::{http::StatusCode, Json};
use chrono::Utc;
use diesel::ExpressionMethods;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;
use diesel::prelude::*;
use tracing::{event, Level};

use crate::{api::v1::{db::database::DatabaseConnection, types::session::Session, utils::{errors::{internal_error, ErrorResponse}, hashing::{encrypt_session, verify_password}}}, config::env_config::get_env, schema::users};


#[derive(Debug, Deserialize)]
pub struct SigninRequest {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    jwt: String
}

// #[tracing::instrument(err, ret)]
#[tracing::instrument(skip_all, fields(username = %req.username))]
pub async fn handle_signin(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(req): Json<SigninRequest>
) -> Result<Json<SigninResponse>, ErrorResponse> {
    // event!(Level::INFO, username = req.username);

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
                        exp: Utc::now().checked_add_signed(chrono::Duration::seconds(get_env().JWT_EXPIRED_TIME as i64)).unwrap().timestamp() as usize,
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
                _ => {
                    event!(Level::ERROR, "{}", e.to_string());
                    Err(internal_error(e))
                },
            }
        }
    }
}
