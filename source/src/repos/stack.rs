use crate::db::DBIf;
use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::Document;
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
    app_logger: Arc<dyn AppLoggerIf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StackItem {
    short: String,
}

#[async_trait]
impl StackRepoIf for StackRepo {
    async fn insert(&self, short: &str) -> AppResult<()> {
        let inserting_doc: Document = bson::to_bson(&StackItem {
            short: short.to_string(),
        })
        .unwrap()
        .as_document()
        .unwrap()
        .clone();

        &self
            .db
            .get()
            .collection("stack")
            .insert_one(inserting_doc, None)
            .await
            .log_on_err(self.logger())
            .into_app_err();

        Ok(())
    }
}
