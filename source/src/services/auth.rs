use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::tokens::{TokenPair, TokensRepoIf};
use crate::repos::users::{NewUser, User, UsersRepoIf};
use crate::services::check::CheckServiceIf;
use crate::utils::{AppResult, IntoAppErr, LogErrWith};
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait AuthServiceIf: Interface {
    async fn login(&self, username: String, password: String) -> AppResult<TokenPair>;
    async fn register(&self, login: String, password: String) -> AppResult<()>;
    async fn find_user_by_access(&self, access: String) -> Option<User>;
}

#[derive(Component, HasLogger)]
#[shaku(interface = AuthServiceIf)]
pub struct AuthService {
    #[shaku(inject)]
    users_repo: Arc<dyn UsersRepoIf>,

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
            .users_repo
            .find_by_username(username.as_str())
            .await
            .ok_or(AppError::login_failed())?;

        if !verify(password, &user.password).into_app_err()? {
            return Err(AppError::login_failed());
        }

        let current_time = Utc::now();
        let access_lifetime =
            (current_time.timestamp() + self.access_token_lifetime.num_seconds()) as i32;
        let refresh_lifetime =
            (current_time.timestamp() + self.refresh_token_lifetime.num_seconds()) as i32;

        let mut tokens = TokenPair {
            access: Uuid::new_v4().to_string().replace("-", ""),
            refresh: Uuid::new_v4().to_string().replace("-", ""),
            access_lifetime_secs: access_lifetime,
            refresh_lifetime_secs: refresh_lifetime,
            created_at: current_time,
            user_id: user.id.into(),
        };

        self.tokens_repo.insert(&tokens).await;

        tokens.access_lifetime_secs = tokens.access_lifetime_secs - current_time.timestamp() as i32;
        tokens.refresh_lifetime_secs =
            tokens.refresh_lifetime_secs - current_time.timestamp() as i32;

        Ok(tokens)
    }

    async fn register(&self, login: String, password: String) -> AppResult<()> {
        self.check_service.strong_password(password.as_str())?;
        if self.check_service.username_exists(login.as_str()).await {
            return Err(AppError::check(
                format!("Username `{}` already taken", login).as_str(),
            ));
        }

        let encrypted_password = hash(password, DEFAULT_COST)
            .log_err_with(self.logger())
            .into_app_err()?;

        self.users_repo
            .insert(&NewUser {
                username: login,
                password: encrypted_password,
            })
            .await;

        Ok(())
    }

    async fn find_user_by_access(&self, access: String) -> Option<User> {
        let token = self.tokens_repo.find_by_access(access).await?;
        self.users_repo.find(token.user_id).await
    }
}
