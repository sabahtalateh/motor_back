use crate::logger::AppLoggerIf;
use crate::repos::users::UserRepoIf;
use crate::services::check::CheckServiceIf;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait AuthServiceIf: Interface {
    async fn auth(&self) -> AppResult<bool>;
    async fn register(&self, login: String, password: String) -> AppResult<()>;
}

#[derive(Component, HasLogger)]
#[shaku(interface = AuthServiceIf)]
pub struct AuthService {
    #[shaku(inject)]
    user_repo: Arc<dyn UserRepoIf>,

    #[shaku(inject)]
    check_service: Arc<dyn CheckServiceIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl AuthServiceIf for AuthService {
    async fn auth(&self) -> AppResult<bool> {
        Ok(true)
    }

    async fn register(&self, login: String, password: String) -> AppResult<()> {
        self.check_service.strong_password(password.as_str())?;
        self.check_service.username_exists(login.as_str()).await?;

        let encrypted_password = hash(password, DEFAULT_COST)
            .log_on_err(self.logger())
            .into_app_err()?;

        self.user_repo.insert(login, encrypted_password).await
    }
}
