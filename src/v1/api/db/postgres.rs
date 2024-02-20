use axum::http::StatusCode;
use diesel_async::RunQueryDsl;
use diesel::{insert_into, SelectableHelper};

use crate::v1::api::utils::errors::ErrorResponse;
use crate::{schema::users, v1::api::models::users::{NewUser, User}};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use super::base::DbBase;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

// pub struct PostgresDb(
//     pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
// );
// #[derive(Clone)]
pub struct PostgresDb {
    pub conn: bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl DbBase for PostgresDb {
    async fn add_user(&mut self, username: &str, pass: &str) -> Result<(), ErrorResponse> {
        match insert_into(users::table)
        .values(&NewUser {
            username,
            password_hash: pass,
        })
        .returning(User::as_returning())
        .get_result(&mut self.conn).await {
            Ok(data) => {
                println!("{:?}", data);
                Ok(())
            },
            Err(err) => {
                eprintln!("{}", err);
                Err(ErrorResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    msg: err.to_string(),
                })
            },
        }
    }

}
