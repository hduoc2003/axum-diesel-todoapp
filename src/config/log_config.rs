use std::fs::{self, File};

use tracing_subscriber::EnvFilter;

use super::env_config::get_env;

pub fn config_log() {
    let env_filter = EnvFilter::try_new(&get_env().RUST_LOG).unwrap();

    fs::create_dir("logs").unwrap_or_else(|_| {});
    let file = File::create("./logs/all.log").expect("Could not create logfile");
    // let file_log = tracing_subscriber::fmt::layer()
    //     // .json()
    //     .with_writer(Arc::new(file));
    tracing_subscriber::fmt()
        .json()
        .with_writer(file)
        .with_env_filter(env_filter)
        .init();
    // let file_subscriber = tracing_subscriber::fmt()
    // .json()
    // .with_writer(std::io::stdout)
    // .with_env_filter(env_filter)
    // .finish();
    // tracing_subscriber::fmt().json().init();
    // let stdout_log = tracing_subscriber::fmt::layer().pretty();
    // tracing_subscriber::fmt()
    // // .with_max_level(tracing::Level::DEBUG)
    // .with_env_filter(env_filter)
    // .with(file_log)
    // .with_target(false)
    // .init();
    // tracing_subscriber::registry()
    //     .with(file_log)
    //     .f
}
