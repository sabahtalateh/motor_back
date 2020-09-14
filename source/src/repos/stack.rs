use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bson::Document;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait StackRepoIf: Interface {
    async fn insert(&self, short: &str) -> AppResult<()>;
}

#[shaku(interface = StackRepoIf)]
#[derive(Component, HasLogger)]
pub struct StackRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    pub app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl StackRepoIf for StackRepo {
    async fn insert(&self, short: &str) -> AppResult<()> {
        Ok(())
    }
}
