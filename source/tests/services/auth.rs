use crate::{trunc_collection, DEFAULT_CONFIG};
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::init::init_app;
use motor_back::services::auth::AuthServiceIf;
use shaku::HasComponent;
use chrono::Utc;

#[actix_rt::test]
async fn test_registration_failed_if_password_weak() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;

    let auth: &dyn AuthServiceIf = ctr.resolve_ref();
    let reg_result = auth.register("U".to_string(), "1".to_string()).await;

    assert_eq!(
        reg_result,
        Err(AppError::check(
            "password length should be at least `2` characters"
        ))
    );
}

#[actix_rt::test]
async fn test_registration_success_if_password_strong() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;

    let reg_result = auth.register("U".to_string(), "12".to_string()).await;

    assert_eq!(reg_result, Ok(()));
}

#[actix_rt::test]
async fn test_registration_failed_if_username_exists() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;

    let reg_result = auth.register("User".to_string(), "12".to_string()).await;
    assert_eq!(reg_result, Ok(()));

    let reg_result = auth.register("User".to_string(), "12".to_string()).await;
    assert_eq!(
        reg_result,
        Err(AppError::check("Username `User` already taken"))
    );
}

#[actix_rt::test]
async fn test_can_not_login_with_wrong_creds() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;

    let reg_result = auth.register("User2".to_string(), "12".to_string()).await;
    assert_eq!(reg_result, Ok(()));

    let login_result = auth.login("User3".to_string(), "12".to_string(), Utc::now()).await;
    assert_eq!(login_result.is_err(), true);
}

#[actix_rt::test]
async fn test_login_with_ok_creds() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;

    let reg_result = auth.register("User3".to_string(), "123".to_string()).await;
    assert_eq!(reg_result, Ok(()));

    let login_result = auth.login("User3".to_string(), "123".to_string(), Utc::now()).await;
    assert_eq!(login_result.is_ok(), true);
}

/// TODO тесты на рефрешь токена (подкрутить конфиг для этого)
///      тест на то что пользователь по токены находится
///      тест на то что токен валидный/невалидный
