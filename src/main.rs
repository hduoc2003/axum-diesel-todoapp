use axum::Router;
use v1::api::{
    db::database::get_connection_pool, routes::signup, types::env::ENV, utils::env::get_env,
};
// use v1::api::routes::signup::handle_signup;
mod config;
mod schema;
mod v1;

#[tokio::main]
async fn main() {
    config::config().await;
    let connection_pool = get_connection_pool().await;

    let app = Router::new()
        .nest("/v1/api", signup::routes())
        .with_state(connection_pool);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", get_env(ENV::HOST), get_env(ENV::PORT)))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();
}