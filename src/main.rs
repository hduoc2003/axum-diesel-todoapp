use axum::Router;
use config::{config, AppState};
use v1::api::{db::database::Database, routes::signup, types::{db::DatabaseType, env::ENV}, utils::env::get_env};
// use v1::api::routes::signup::handle_signup;
mod v1;
mod config;
mod schema;

#[tokio::main]
async fn main() {
    let config = config().await;

    let db: Database = match get_env(ENV::DATABASE).as_str() {
        "postgres" => Database::new(DatabaseType::Postgres).await,
        "in-memory" => Database::new(DatabaseType::InMemory).await,
        x => panic!("Only support postgres and in-memory database, not {}", x)
    };


    let app_state = AppState {
        db
    };

    let app = Router::new()
    .nest("/v1/api", signup::routes())
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", get_env(ENV::PORT))).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
