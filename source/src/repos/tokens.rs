use crate::db::DBIf;
use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::Document;
use chrono::{DateTime, Utc};
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use juniper::GraphQLObject;
use crate::repos::Id;

#[async_trait]
pub trait TokensRepoIf: Interface {
    async fn insert(&self, tokens: &TokenPair) -> AppResult<()>;
}

#[shaku(interface = TokensRepoIf)]
#[derive(Component, HasLogger)]
pub struct TokensRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    pub app_logger: Arc<dyn AppLoggerIf>,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct TokenPair {
    pub access: String,
    pub refresh: String,
    pub access_lifetime_secs: i32,
    pub refresh_lifetime_secs: i32,
    pub created_at: DateTime<Utc>,

    #[graphql(skip)]
    pub user_id: Id,
}

#[async_trait]
impl TokensRepoIf for TokensRepo {
    async fn insert(&self, tokens: &TokenPair) -> AppResult<()> {
        let inserting_doc: Document = bson::to_bson(&tokens)
            .unwrap()
            .as_document()
            .unwrap()
            .clone();

        self.db
            .get()
            .collection("tokens")
            .insert_one(inserting_doc, None)
            .await
            .map(|_| Ok(()))
            .log_on_err(self.logger())
            .into_app_err()?
    }
}
