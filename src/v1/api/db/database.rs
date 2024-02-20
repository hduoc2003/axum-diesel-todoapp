use diesel::prelude::*;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};

use crate::v1::api::{
    types::{db::DatabaseType, env::ENV},
    utils::env::get_env,
};

use super::{base::DbBase, in_memory::InMemoryDb, postgres::PostgresDb};

#[derive(Clone)]
pub struct Database {
    database: Box<dyn DbBase>,
    // database: Box<PostgresDb>
}

impl Database {
    pub async fn new(db_type: DatabaseType) -> Self {
        Database {
            database: match db_type {
                DatabaseType::Postgres => {
                    let database_url = get_env(ENV::DATABASE_URL);
                    // let conn = PgConnection::establish(&database_url)
                    //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
                    let config =
                        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
                            get_env(ENV::DATABASE_URL)
                        );
                    let pool = Pool::builder().build(config).await.unwrap();

                    // checkout a connection from the pool
                    let mut conn = pool.get().await.unwrap();
                    Box::new(PostgresDb { conn })
                    // PostgresDb { conn }
                }
                DatabaseType::InMemory => Box::new(InMemoryDb),
                // DatabaseType::InMemory => !panic!(),
            },
        }
    }

    pub async fn add_user(
        &mut self,
        username: &str,
        pass: &str,
    ) -> Result<(), crate::v1::api::utils::errors::ErrorResponse> {
        self.database.add_user(username, pass).await
    }
}

// impl Database {
//     fn add_user(
//         &self,
//         username: &str,
//         pass: &str,
//     ) -> Result<(), crate::v1::api::utils::errors::ErrorResponse> {
//         self.database.add_user(username, pass)
//     }
// }
