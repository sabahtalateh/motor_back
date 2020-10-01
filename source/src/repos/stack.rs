use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
use crate::utils::{LogErrWith, OkOrMongoRecordId, IntoAppErr, deserialize_bson};
use async_trait::async_trait;
use bson::{Document};
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use bson::oid::ObjectId;
use juniper::futures::StreamExt;

#[async_trait]
pub trait StackRepoIf: Interface {
    async fn insert(&self, stack_item: NewStackItem) -> StackItem;
    async fn find_by_user_id(&self, user_id: Id) -> Vec<StackItem>;
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

#[derive(Serialize, Debug)]
pub struct NewStackItem {
    pub short: String,
    pub user_id: Id,
}

#[derive(Deserialize, Debug, GraphQLObject)]
pub struct StackItem {
    #[serde(rename = "_id")]
    pub id: Id,
    pub short: String,
}

#[async_trait]
impl StackRepoIf for StackRepo {
    async fn insert(&self, stack_item: NewStackItem) -> StackItem {
        let doc: Document = bson::to_bson(&stack_item)
            .unwrap()
            .as_document()
            .unwrap()
            .clone();

        let id = self
            .db
            .get()
            .collection("stack")
            .insert_one(doc, None)
            .await
            .map(|ok| ok.inserted_id)
            .log_err_with(self.logger())
            .unwrap()
            .as_object_id()
            .ok_or_mongo_record_id()
            .log_err_with(self.logger())
            .unwrap()
            .clone();

        StackItem {
            id: id.into(),
            short: stack_item.short,
        }
    }

    async fn find_by_user_id(&self, user_id: Id) -> Vec<StackItem> {
        let user_id: ObjectId = user_id.into();

        self.db
            .get()
            .collection("stack")
            .find(Some(doc! {"user_id": user_id}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|x| deserialize_bson(&x.unwrap()))
            .collect()
            .await
            // .unwrap()
    }
}
