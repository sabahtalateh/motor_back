use std::sync::Arc;

use async_graphql::SimpleObject;
use async_trait::async_trait;
use bson::Document;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith};

pub const COLLECTION: &str = "tokens";

#[async_trait]
pub trait TokensRepoIf: Interface {
    async fn find_by_access(&self, access: &str) -> Option<TokenPair>;
    async fn find_by_refresh(&self, refresh: &str) -> Option<TokenPair>;
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

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct TokenPair {
    pub access: String,
    pub refresh: String,
    pub access_lifetime: DateTime<Utc>,
    pub refresh_lifetime: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub user_id: Id,
}

#[async_trait]
impl TokensRepoIf for TokensRepo {
    async fn find_by_access(&self, access: &str) -> Option<TokenPair> {
        self.db
            .get()
            .collection(COLLECTION)
            .find_one(Some(doc! {"access": access}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|x| deserialize_bson(&x))
    }

    async fn find_by_refresh(&self, refresh: &str) -> Option<TokenPair> {
        self.db
            .get()
            .collection(COLLECTION)
            .find_one(Some(doc! {"refresh": refresh}), None)
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
            .collection(COLLECTION)
            .insert_one(inserting_doc, None)
            .await
            .log_err_with(self.logger())
            .unwrap();
    }
}
