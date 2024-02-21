use dotenv::dotenv;
use tokio::sync::OnceCell;

use crate::api::v1::{types::env::ENV, utils::env::get_env};

// Define a struct to represent server configuration
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

// Define a struct to represent database configuration
#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

// Define a struct that aggregates server and database configuration
#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
}

// Implement methods for the Config struct to access configuration values
impl Config {
    // Getter method for the database URL
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    // Getter method for the server host
    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    // Getter method for the server port
    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}

// Create a static OnceCell to store the application configuration
pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

// Asynchronously initialize the configuration
async fn init_config() -> Config {
    // Load environment variables from a .env file if present
    dotenv().ok();

    // Create a ServerConfig instance with default values or values from environment variables
    let server_config = ServerConfig {
        host: get_env(ENV::HOST),
        port: get_env(ENV::PORT).parse().unwrap(),
    };

    // Create a DatabaseConfig instance with a required DATABASE_URL environment variable
    let database_config = DatabaseConfig {
        url: get_env(ENV::DATABASE_URL),
    };

    // Create a Config instance by combining server and database configurations
    Config {
        server: server_config,
        db: database_config,
    }
}

// Asynchronously retrieve the application configuration, initializing it if necessary
pub async fn config() -> &'static Config {
    // Get the configuration from the OnceCell or initialize it if it hasn't been set yet
    CONFIG.get_or_init(init_config).await
}
