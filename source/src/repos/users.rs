use crate::db::DBIf;

use crate::logger::{AppLoggerIf, WithLogger};
use crate::utils::{AppResult, LogOnErr, IntoAppErr};
use async_trait::async_trait;
use bson::Document;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait UserRepoIf: Interface {
    async fn username_exists(&self, username: &str) -> AppResult<bool>;
    async fn insert(&self, username: String, password: String) -> AppResult<()>;
}

#[derive(Component)]
#[shaku(interface = UserRepoIf)]
pub struct UserRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

impl WithLogger for UserRepo {
    fn logger(&self) -> &Logger {
        self.app_logger.logger()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewUser {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl UserRepoIf for UserRepo {
    async fn username_exists(&self, username: &str) -> AppResult<bool> {
        match &self
            .db
            .get()
            .collection("users")
            .find_one(Some(doc! {"username": username}), None)
            .await
            .log_on_err(self.logger())
            .into_app_err()?
        {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn insert(&self, username: String, encrypted_password: String) -> AppResult<()> {
        let inserting_doc: Document = bson::to_bson(&NewUser {
            username,
            password: encrypted_password,
        })
        .unwrap()
        .as_document()
        .unwrap()
        .clone();

        self.db
            .get()
            .collection("users")
            .insert_one(inserting_doc, None)
            .await
            .map(|_| Ok(()))
            .log_on_err(self.logger())
            .into_app_err()?
    }
}
