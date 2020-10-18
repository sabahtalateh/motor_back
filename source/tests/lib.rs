mod services;

#[macro_use]
extern crate bson;

#[macro_use]
extern crate lazy_static;

use chrono::{Duration, Utc};
use mongodb::Database;
use motor_back::config::Config;
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::init::init_app;
use motor_back::services::auth::AuthServiceIf;
use motor_back::utils::AppResult;
use shaku::HasComponent;
use slog::Level;
use std::sync::Arc;

const DEFAULT_USER_NAME: &str = "Ivan";
const DEFAULT_PASSWORD: &str = "123123";

lazy_static! {
    static ref DEFAULT_CONFIG: Config = Config {
        app_name: "test".to_string(),
        api_version: "1.0".to_string(),
        proto: "http".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        mongo_host: "localhost".to_string(),
        mongo_port: 27017,
        mongo_pool_size: 100,
        db_name: "motor_test".to_string(),
        pwd_min_len: 6,
        access_token_lifetime: Duration::hours(1),
        refresh_token_lifetime: Duration::days(14),
        clear_logger_files: true,
        loggers_json_pretty: true,
        app_logger_file: "../log/app_test.json".to_string(),
        app_logger_level: Level::Debug,
    };
}

pub async fn setup() -> Container {
    let config: Config = Config {
        app_name: "test".to_string(),
        api_version: "1.0".to_string(),
        proto: "http".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        mongo_host: "localhost".to_string(),
        mongo_port: 27017,
        mongo_pool_size: 100,
        db_name: "motor_test".to_string(),
        pwd_min_len: 6,
        access_token_lifetime: Duration::hours(1),
        refresh_token_lifetime: Duration::days(14),
        clear_logger_files: true,
        loggers_json_pretty: true,
        app_logger_file: "../log/app_test.json".to_string(),
        app_logger_level: Level::Debug,
    };

    init_app(&config).await
}

pub async fn setup_with_user(login: String, password: String) -> Container {
    let container: Container = setup().await;

    let db: &dyn DBIf = container.resolve_ref();
    db.get()
        .collection("users")
        .delete_one(doc! {"username": login.clone()}, None)
        .await;

    let auth: &dyn AuthServiceIf = container.resolve_ref();
    auth.register(login, password).await.unwrap();

    container
}

pub async fn setup_with_default_user() -> Container {
    setup_with_user(DEFAULT_USER_NAME.to_string(), DEFAULT_PASSWORD.to_string()).await
}

pub async fn trunc_collection(db: &Database, collection: &str) -> () {
    db.collection(collection).delete_many(doc! {}, None).await;
}
