use crate::db::DBIf;
use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::utils::{AppResult, IntoAppErr, LogOnErr};
use async_trait::async_trait;
use bson::Document;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use bson::oid::ObjectId;

#[async_trait]
pub trait UsersRepoIf: Interface {
    async fn insert(&self, new_user: NewUser) -> AppResult<()>;
    async fn username_exists(&self, username: &str) -> AppResult<bool>;
    async fn find_by_username(&self, username: &str) -> AppResult<User>;
}

#[shaku(interface = UsersRepoIf)]
#[derive(Component, HasLogger)]
pub struct UsersRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    pub app_logger: Arc<dyn AppLoggerIf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
}

#[async_trait]
impl UsersRepoIf for UsersRepo {
    async fn insert(&self, new_user: NewUser) -> AppResult<()> {
        let inserting_doc: Document = bson::to_bson(&new_user)
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

    async fn find_by_username(&self, username: &str) -> AppResult<User> {
        match &self
            .db
            .get()
            .collection("users")
            .find_one(Some(doc! {"username": username}), None)
            .await
            .log_on_err(self.logger())
            .into_app_err()?
        {
            Some(doc) => Ok(bson::from_bson(bson::to_bson(doc).unwrap()).unwrap()),
            None => Err(AppError::not_found("No user found")),
        }
    }
}
