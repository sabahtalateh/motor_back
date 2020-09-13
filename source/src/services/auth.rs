use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::tokens::{TokenPair, TokensRepoIf};
use crate::repos::users::{NewUser, UsersRepoIf};
use crate::services::check::CheckServiceIf;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait AuthServiceIf: Interface {
    async fn login(&self, username: String, password: String) -> AppResult<TokenPair>;
    async fn register(&self, login: String, password: String) -> AppResult<()>;
}

#[derive(Component, HasLogger)]
#[shaku(interface = AuthServiceIf)]
pub struct AuthService {
    #[shaku(inject)]
    user_repo: Arc<dyn UsersRepoIf>,

    #[shaku(inject)]
    tokens_repo: Arc<dyn TokensRepoIf>,

    #[shaku(inject)]
    check_service: Arc<dyn CheckServiceIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,

    #[shaku(no_default)]
    access_token_lifetime: Duration,

    #[shaku(no_default)]
    refresh_token_lifetime: Duration,
}

#[async_trait]
impl AuthServiceIf for AuthService {
    async fn login(&self, username: String, password: String) -> AppResult<TokenPair> {
        let user = self
            .user_repo
            .find_by_username(username.as_str())
            .await
            .map_err(|_| AppError::unauthorized("Login Failed"))?;

        if !verify(password, &user.password).into_app_err()? {
            return Err(AppError::unauthorized("Login Failed"));
        }

        let current_time = Utc::now();

        let tokens = TokenPair {
            access: Uuid::new_v4().to_string().replace("-", ""),
            refresh: Uuid::new_v4().to_string().replace("-", ""),
            access_lifetime_secs: (current_time.timestamp()
                + self.access_token_lifetime.num_seconds())
                as i32,
            refresh_lifetime_secs: (current_time.timestamp()
                + self.refresh_token_lifetime.num_seconds())
                as i32,
            created_at: current_time,
            user_id: user.id.into(),
        };

        self.tokens_repo.insert(&tokens).await;

        Ok(tokens)
    }

    async fn register(&self, login: String, password: String) -> AppResult<()> {
        self.check_service.strong_password(password.as_str())?;
        self.check_service.username_exists(login.as_str()).await?;

        let encrypted_password = hash(password, DEFAULT_COST)
            .log_on_err(self.logger())
            .into_app_err()?;

        self.user_repo
            .insert(NewUser {
                username: login,
                password: encrypted_password,
            })
            .await
    }
}
