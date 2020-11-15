#[macro_use]
extern crate bson;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate thiserror;

use chrono::Duration;
use mongodb::Database;
use shaku::HasComponent;
use slog::Level;
use uuid::Uuid;

use motor_back::config::Config;
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::init::init_app;
use motor_back::repos::users::{User, UsersRepoIf};
use motor_back::services::auth::AuthServiceIf;

mod services;

const DEFAULT_PASSWORD: &str = "123123";

lazy_static! {
    static ref DEFAULT_CONFIG: Config = Config {
        app_name: "test".to_string(),
        api_version: "1.0".to_string(),
        proto: "http".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        allowed_origins: vec![],
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
        allowed_origins: vec![],
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

pub async fn setup_with_user(login: String, password: String, drop_db: bool) -> (Container, User) {
    let container: Container = setup().await;

    let db: &dyn DBIf = container.resolve_ref();
    let db = db.get();

    if drop_db {
        db.drop(None).await;
    }

    let user = db
        .collection("users")
        .find_one(Some(doc! {"username": login.clone()}), None)
        .await
        .unwrap();

    match user {
        Some(_) => {
            db.collection("users")
                .delete_one(doc! {"username": login.clone()}, None)
                .await;
        }
        _ => (),
    }

    let auth: &dyn AuthServiceIf = container.resolve_ref();
    auth.register(login.clone(), password).await.unwrap();

    let user_repo: &dyn UsersRepoIf = container.resolve_ref();
    let user = user_repo.find_by_username(&(login.clone())).await.unwrap();

    (container, user)
}

pub async fn setup_with_random_user() -> (Container, User) {
    setup_with_user(
        Uuid::new_v4().to_string().replace("-", ""),
        DEFAULT_PASSWORD.to_string(),
        false,
    )
    .await
}

pub async fn drop_and_setup_with_random_user() -> (Container, User) {
    setup_with_user(
        Uuid::new_v4().to_string().replace("-", ""),
        DEFAULT_PASSWORD.to_string(),
        true,
    )
    .await
}

pub async fn trunc_collection(db: &Database, collection: &str) -> () {
    db.collection(collection)
        .delete_many(doc! {}, None)
        .await
        .unwrap();
}

pub async fn drop_db(db: &Database) -> () {
    db.drop(None).await;
}
