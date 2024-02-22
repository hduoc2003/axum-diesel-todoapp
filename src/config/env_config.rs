use dotenv::dotenv;
use tokio::sync::OnceCell;

use crate::api::v1::types::db::DatabaseType;

#[allow(non_snake_case)]
pub struct Env {
    pub HOST: String,
    pub PORT: i32,
    pub DATABASE: DatabaseType,
    pub DATABASE_URL: String,
    pub JWT_KEY: String,
    pub JWT_EXPIRED_TIME: usize,
    pub RUST_LOG: String
}

static ENV: OnceCell<Env> = OnceCell::const_new();

fn read_env(key: &'static str) -> String {
    std::env::var(key).expect(&format!("{} must be set", key))
}

pub async fn config_env() {
    dotenv().ok();
    let db_type: DatabaseType = match read_env("DATABASE").as_ref() {
        "postgres" => DatabaseType::Postgres,
        "in-memory" => DatabaseType::InMemory,
        x => panic!("Only support postgres and in-memory databse, not {}", x)
    };
    ENV.get_or_init(|| async {
        Env {
            HOST: read_env("HOST"),
            PORT: read_env("PORT").parse().expect("PORT must be number"),
            DATABASE: db_type,
            DATABASE_URL: read_env("DATABASE_URL"),
            JWT_KEY: read_env("JWT_KEY"),
            JWT_EXPIRED_TIME: read_env("JWT_EXPIRED_TIME").parse().expect("JWT_EXPIRED_TIME muse be number"),
            RUST_LOG: read_env("RUST_LOG"),
        }
    }).await;
}

pub fn get_env() -> &'static Env {
    ENV.get().unwrap()
}
