use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
use crate::utils::{deserialize_bson, AppResult, IntoAppErr, LogErrWith};
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::Document;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait TokensRepoIf: Interface {
    async fn find_by_access(&self, access: String) -> Option<TokenPair>;
    async fn insert(&self, tokens: &TokenPair);
}

#[shaku(interface = TokensRepoIf)]
#[derive(Component, HasLogger)]
pub struct TokensRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
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
    async fn find_by_access(&self, access: String) -> Option<TokenPair> {
        self.db
            .get()
            .collection("tokens")
            .find_one(Some(doc! {"access": access}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|x| deserialize_bson(&x))
    }

    async fn insert(&self, tokens: &TokenPair) {
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
            .log_err_with(self.logger())
            .unwrap();
    }
}
