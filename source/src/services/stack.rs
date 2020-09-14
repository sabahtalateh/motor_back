use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::stack::StackRepoIf;
use crate::repos::tokens::{TokenPair, TokensRepoIf};
use crate::repos::users::{NewUser, UsersRepoIf};
use crate::services::check::CheckServiceIf;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait StackServiceIf: Interface {
    async fn stack(&self, short: &str) -> AppResult<()>;
}

#[shaku(interface = StackServiceIf)]
#[derive(Component, HasLogger)]
pub struct StackService {
    #[shaku(inject)]
    stack_repo: Arc<dyn StackRepoIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl StackServiceIf for StackService {
    async fn stack(&self, short: &str) -> AppResult<()> {
        unimplemented!()
    }
}
