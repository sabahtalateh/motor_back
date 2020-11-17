use chrono::{Duration, Utc};
use shaku::HasComponent;

use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::init::init_app;
use motor_back::services::auth::AuthServiceIf;

use crate::{DEFAULT_CONFIG, trunc_collection};

// #[actix_rt::test]
// async fn rrr() -> () {
//     let mut config = (&*DEFAULT_CONFIG).clone();
//     config.pwd_min_len = 0;
//     let ctr: Container = init_app(&config).await;
//
//     let db: &dyn DBIf = ctr.resolve_ref();
//     trunc_collection(&db.get(), "users").await;
//
//     let auth: &dyn AuthServiceIf = ctr.resolve_ref();
//     let reg_result = auth.register("U".to_string(), "1".to_string()).await;
//     println!("{:#?}", reg_result);
//
//     let reg_result = auth.register("U".to_string(), "1".to_string()).await;
//     println!("{:#?}", reg_result);
// }

#[actix_rt::test]
async fn registration_failed_if_password_weak() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let auth: &dyn AuthServiceIf = ctr.resolve_ref();
    let reg_result = auth.register("U", "1").await;

    assert_eq!(
        reg_result,
        Err(AppError::validation(
            "password length should be at least `2` characters"
        ))
    );
}

#[actix_rt::test]
async fn registration_success_if_password_strong() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let reg_result = auth.register("U", "12").await;

    assert_eq!(reg_result, Ok(()));
}

#[actix_rt::test]
async fn registration_failed_if_username_exists() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let reg_result = auth.register("User", "12").await;
    assert_eq!(reg_result, Ok(()));

    let reg_result = auth.register("User", "12").await;
    assert_eq!(
        reg_result,
        Err(AppError::validation("Username `User` already taken"))
    );
}

#[actix_rt::test]
async fn can_not_login_with_incorrect_creds() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;

    let reg_result = auth.register("User2", "12").await;
    assert_eq!(reg_result, Ok(()));

    let login_result = auth.login("User3", "12", Utc::now()).await;
    assert_eq!(login_result.is_err(), true);
}

#[actix_rt::test]
async fn can_login_with_ok_creds() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    config.pwd_min_len = 2;
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let reg_result = auth.register("User3", "123").await;
    assert_eq!(reg_result, Ok(()));

    let login_result = auth.login("User3", "123", Utc::now()).await;
    assert!(login_result.is_ok());
}

#[actix_rt::test]
async fn refresh_fails_with_incorrect_token() -> () {
    let config = (&*DEFAULT_CONFIG).clone();
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let reg_result = auth.register("User8", "321123").await;
    assert_eq!(reg_result, Ok(()));

    let refresh_result = auth.refresh_token("incorrect_token", Utc::now()).await;
    assert_eq!(refresh_result.map(|_| ()), Err(AppError::unauthorized()));
}


#[actix_rt::test]
async fn refresh_success_with_correct_token() -> () {
    let config = (&*DEFAULT_CONFIG).clone();
    let ctr: Container = init_app(&config).await;
    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let reg_result = auth.register("User101", "321000").await;
    assert_eq!(reg_result, Ok(()));

    let tokens = auth.login("User101", "321000", Utc::now()).await.unwrap();

    let refresh_result = auth.refresh_token(&tokens.refresh, Utc::now()).await;
    assert!(refresh_result.is_ok());
}

#[actix_rt::test]
async fn refresh_failed_with_expired_token() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    // refresh and access token expires just as created
    config.access_token_lifetime = Duration::seconds(0);
    config.refresh_token_lifetime = Duration::seconds(0);
    let ctr: Container = init_app(&config).await;

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let reg_result = auth.register("User1011", "321000").await;
    assert_eq!(reg_result, Ok(()));

    let tokens = auth.login("User1011", "321000", Utc::now()).await.unwrap();

    let refresh_result = auth.refresh_token(&tokens.refresh, Utc::now()).await;
    assert_eq!(refresh_result.map(|_|()), Err(AppError::unauthorized()));
}

#[actix_rt::test]
async fn validation_failed_for_incorrect_access() -> () {
    let mut config = (&*DEFAULT_CONFIG).clone();
    // refresh and access token expires just as created
    config.access_token_lifetime = Duration::seconds(0);
    config.refresh_token_lifetime = Duration::seconds(0);
    let ctr: Container = init_app(&config).await;

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let reg_result = auth.register("User10115", "321000").await;
    assert_eq!(reg_result, Ok(()));

    let result = auth.validate_access("incorrect_access", &Utc::now()).await;
    assert_eq!(result.map(|_|()), Err(AppError::unauthorized()));
}

#[actix_rt::test]
async fn validation_passed_for_correct_access() -> () {
    let config = (&*DEFAULT_CONFIG).clone();
    let ctr: Container = init_app(&config).await;

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "users").await;
    trunc_collection(&db.get(), "tokens").await;

    let auth: &dyn AuthServiceIf = ctr.resolve_ref();

    let reg_result = auth.register("User10112", "321000").await;
    assert_eq!(reg_result, Ok(()));

    let tokens = auth.login("User10112", "321000", &Utc::now()).await.unwrap();

    let result = auth.validate_access(&tokens.access, &Utc::now()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().username, "User10112");
}
