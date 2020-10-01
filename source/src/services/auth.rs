use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::tokens::{TokenPair, TokensRepoIf};
use crate::repos::users::{NewUser, User, UsersRepoIf};
use crate::repos::Id;
use crate::services::check::CheckServiceIf;
use crate::utils::{AppResult, IntoAppErr, LogErrWith, OkOrUnauthorized};
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
    async fn login(
        &self,
        username: String,
        password: String,
        now: DateTime<Utc>,
    ) -> AppResult<TokenPair>;
    async fn register(&self, login: String, password: String) -> AppResult<()>;
    async fn refresh_token(&self, refresh: &str, now: DateTime<Utc>) -> AppResult<TokenPair>;
    async fn find_user_by_access(&self, access: &str) -> Option<User>;
    async fn validate_access(&self, access: &str, now: DateTime<Utc>) -> AppResult<User>;
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
    async fn login(
        &self,
        username: String,
        password: String,
        now: DateTime<Utc>,
    ) -> AppResult<TokenPair> {
        let user = self
            .users_repo
            .find_by_username(username.as_str())
            .await
            .ok_or(AppError::login_failed())?;

        if !verify(password, &user.password).into_app_err()? {
            return Err(AppError::login_failed());
        }

        let token = self.construct_token(user.id, now);
        self.tokens_repo.insert(&token).await;

        Ok(token)
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

    async fn refresh_token(&self, refresh: &str, now: DateTime<Utc>) -> AppResult<TokenPair> {
        let token = self
            .tokens_repo
            .find_by_refresh(refresh)
            .await
            .ok_or_unauthorized()?;

        if token.refresh_lifetime < now {
            return Err(AppError::unauthorized());
        }

        let token = self.construct_token(token.user_id, now);
        self.tokens_repo.insert(&token).await;

        Ok(token)
    }

    async fn find_user_by_access(&self, access: &str) -> Option<User> {
        let token = self.tokens_repo.find_by_access(access).await?;
        self.users_repo.find(token.user_id).await
    }

    async fn validate_access(&self, access: &str, now: DateTime<Utc>) -> AppResult<User> {
        let token = self
            .tokens_repo
            .find_by_access(access)
            .await
            .ok_or_unauthorized()?;

        if token.access_lifetime < now && token.refresh_lifetime <= now {
            return Err(AppError::unauthorized());
        }

        if token.access_lifetime < now && token.refresh_lifetime > now {
            return Err(AppError::access_expire());
        }

        self.find_user_by_access(&token.access)
            .await
            .ok_or_unauthorized()
    }
}

impl AuthService {
    fn construct_token(&self, user_id: Id, current_time: DateTime<Utc>) -> TokenPair {
        let access_lifetime =
            current_time + Duration::seconds(self.access_token_lifetime.num_seconds());
        let refresh_lifetime =
            current_time + Duration::seconds(self.refresh_token_lifetime.num_seconds());

        let token = TokenPair {
            access: Uuid::new_v4().to_string().replace("-", ""),
            refresh: Uuid::new_v4().to_string().replace("-", ""),
            access_lifetime,
            refresh_lifetime,
            created_at: current_time,
            user_id: user_id.into(),
        };
        token
    }
}
