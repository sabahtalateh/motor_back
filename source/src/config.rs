use chrono::Duration;
use dotenv::dotenv;

use shaku::{Component, Interface};
use slog::Level;
use std::env;
use std::str::FromStr;

pub trait ConfigIf: Interface {}

#[derive(Component, Clone)]
#[shaku(interface = ConfigIf)]
pub struct Config {
    pub app_name: String,
    pub api_version: String,

    pub proto: String,
    pub host: String,
    pub port: u32,

    pub mongo_host: String,
    pub mongo_port: u32,
    pub mongo_pool_size: u32,
    pub db_name: String,

    pub pwd_min_len: u32,

    #[shaku(no_default)]
    pub access_token_lifetime: Duration,
    #[shaku(no_default)]
    pub refresh_token_lifetime: Duration,

    pub clear_logger_files: bool,
    pub loggers_json_pretty: bool,
    pub app_logger_file: String,
    #[shaku(no_default)]
    pub app_logger_level: Level,
}

impl ConfigIf for Config {}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Config {
            app_name: env::var("APP_NAME").expect("APP_NAME not set in .env"),
            api_version: "1.0".to_string(),

            proto: env::var("SERVER_PROTO").expect("SERVER_PROTO not set in .env"),
            host: env::var("SERVER_HOST").expect("SERVER_HOST not set in .env"),
            port: env::var("SERVER_PORT")
                .expect("SERVER_PORT not set in .env")
                .parse()
                .expect("SERVER_PORT must be a valid u32"),

            mongo_host: env::var("MONGO_HOST").expect("MONGO_HOST not set in .env"),
            mongo_port: env::var("MONGO_PORT")
                .expect("MONGO_PORT not set in .env")
                .parse()
                .expect("MONGO_PORT must be a valid u32"),
            mongo_pool_size: 100,
            db_name: env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME not set in .env"),

            pwd_min_len: 6,

            access_token_lifetime: Duration::hours(1),
            refresh_token_lifetime: Duration::days(14),

            clear_logger_files: env::var("CLEAR_LOGGERS_FILES")
                .expect("CLEAR_LOGGERS_FILES not set in .env")
                .parse()
                .expect("CLEAR_LOGGERS_FILES must be valid bool (true/false)"),
            loggers_json_pretty: env::var("LOGGERS_JSON_PRETTY")
                .expect("LOGGERS_JSON_PRETTY not set in .env")
                .parse()
                .expect("LOGGERS_JSON_PRETTY must be valid bool (true/false)"),
            app_logger_file: env::var("APP_LOGGER_FILE").expect("APP_LOGGER_FILE not set in .env"),
            app_logger_level: Level::from_str(
                env::var("APP_LOGGER_LEVEL")
                    .expect("APP_LOGGER_LEVEL not set in .env")
                    .as_str(),
            ).expect("can not parse APP_LOGGER_LEVEL. must be on of ['CRITICAL', 'ERROR', 'WARN', 'INFO', 'DEBUG', 'TRACE']"),
        }
    }
}
