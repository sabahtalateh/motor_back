use chrono::Duration;
use dotenv::dotenv;

use shaku::{Component, Interface};
use slog::Level;
use std::env;
use std::str::FromStr;

pub trait ConfigIf: Interface {
    fn api_version(&self) -> String;
}

#[derive(Debug, Component, Clone)]
#[shaku(interface = ConfigIf)]
pub struct Config {
    pub app_name: String,
    pub api_version: String,

    pub proto: String,
    pub host: String,
    pub port: u32,

    pub allowed_origins: Vec<String>,

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

impl ConfigIf for Config {
    fn api_version(&self) -> String {
        self.api_version.clone()
    }
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Config {
            pwd_min_len: 6,

            access_token_lifetime: Duration::hours(1),
            refresh_token_lifetime: Duration::days(14),

            mongo_pool_size: 100,

            api_version: env!("CARGO_PKG_VERSION").to_string(),

            // всё что ниже читается из .env файлы

            app_name: read_var("APP_NAME"),

            proto: read_var("SERVER_PROTO"),
            host: read_var("SERVER_HOST"),
            port: read_var("SERVER_PORT")
                .parse()
                .expect("SERVER_PORT must be a valid u32"),

            allowed_origins: read_var("ALLOWED_ORIGINS")
                .parse::<String>()
                .expect("ALLOWED_ORIGINS must be `;` separated string")
                .split(";")
                .map(|x| x.trim().to_owned())
                .collect(),

            mongo_host: read_var("MONGO_HOST"),
            mongo_port: read_var("MONGO_PORT")
                .parse()
                .expect("MONGO_PORT must be a valid u32"),

            db_name: read_var("MONGO_DB_NAME"),

            clear_logger_files: read_var("CLEAR_LOGGERS_FILES")
                .parse()
                .expect("CLEAR_LOGGERS_FILES must be valid bool (true/false)"),
            loggers_json_pretty: read_var("LOGGERS_JSON_PRETTY")
                .parse()
                .expect("LOGGERS_JSON_PRETTY must be valid bool (true/false)"),
            app_logger_file: read_var("APP_LOGGER_FILE"),
            app_logger_level: Level::from_str(
                read_var("APP_LOGGER_LEVEL")
                    .as_str(),
            ).expect("can not parse APP_LOGGER_LEVEL. must be on of ['CRITICAL', 'ERROR', 'WARN', 'INFO', 'DEBUG', 'TRACE']"),
        }
    }
}

fn read_var(var_name: &str) -> String {
    env::var(var_name).expect(format!("{} not set in .env", var_name).as_str())
}
