use axum::Router;
use api::v1::routes::auth;
use api::v1::{
    db::database::get_connection_pool, types::env::ENV, utils::env::get_env,
};
// use v1::api::routes::signup::handle_signup;
mod config;
mod schema;
mod api;

#[tokio::main]
async fn main() {
    config::config().await;
    let connection_pool = get_connection_pool().await;

    let app = Router::new()
        .nest("/api/v1", Router::new()
            .merge(auth::router::router())
        )
        .with_state(connection_pool);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", get_env(ENV::HOST), get_env(ENV::PORT)))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();
}
