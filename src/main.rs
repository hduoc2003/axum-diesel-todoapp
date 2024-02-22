use config::env_config::get_env;

// use v1::api::routes::signup::handle_signup;
mod config;
mod schema;
mod api;

#[tokio::main]
async fn main() {
    let app = config::config::config().await;

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", get_env().HOST, get_env().PORT))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();
}
