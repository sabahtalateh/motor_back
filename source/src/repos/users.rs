use crate::db::DBIf;
use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
use crate::utils::{deserialize_bson, AppResult, IntoAppErr, LogErrWith};
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::Document;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait UsersRepoIf: Interface {
    async fn find(&self, id: Id) -> Option<User>;
    async fn insert(&self, new_user: &NewUser);
    async fn find_by_username(&self, username: &str) -> Option<User>;
}

#[shaku(interface = UsersRepoIf)]
#[derive(Component, HasLogger)]
pub struct UsersRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Id,
    pub username: String,
    pub password: String,
}

#[async_trait]
impl UsersRepoIf for UsersRepo {
    async fn find(&self, id: Id) -> Option<User> {
        let id: ObjectId = id.into();

        self.db
            .get()
            .collection("users")
            .find_one(Some(doc! {"_id": id}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|u| deserialize_bson(&u))
    }

    async fn insert(&self, new_user: &NewUser) {
        let inserting_doc: Document = bson::to_bson(new_user)
            .unwrap()
            .as_document()
            .unwrap()
            .clone();

        self.db
            .get()
            .collection("users")
            .insert_one(inserting_doc, None)
            .await
            .log_err_with(self.logger())
            .unwrap();
    }

    async fn find_by_username(&self, username: &str) -> Option<User> {
        self.db
            .get()
            .collection("users")
            .find_one(Some(doc! {"username": username}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|u| deserialize_bson(&u))
    }
}
