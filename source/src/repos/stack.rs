use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::{find_one_by_id, insert_one_into, link_external_ids, Id};
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId};

use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::Document;
use juniper::futures::StreamExt;
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait StackRepoIf: Interface {
    async fn insert(&self, stack_item: &NewStackItem) -> StackItem;
    async fn link_blocks(&self, stack_item: &StackItem, block_ids: &Vec<Id>) -> StackItem;
    async fn link_marks(&self, stack_item: &StackItem, mark_ids: &Vec<Id>) -> StackItem;
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

#[derive(Serialize, Debug, Clone)]
pub struct NewStackItem {
    pub user_id: Id,
    pub title: String,
    pub block_ids: Vec<Id>,
    pub mark_ids: Vec<Id>,
}

#[derive(Serialize, Debug)]
pub struct NewBlock {
    pub text: String,
    pub marks: Vec<NewMark>,
}

#[derive(Serialize, Debug)]
pub struct NewMark {
    pub from: i32,
    pub to: i32,
}

#[derive(Deserialize, Debug, GraphQLObject)]
pub struct StackItem {
    #[serde(rename = "_id")]
    pub id: Id,
    pub title: String,
    pub block_ids: Vec<Id>,
    pub mark_ids: Vec<Id>,
}

#[async_trait]
impl StackRepoIf for StackRepo {
    async fn insert(&self, stack_item: &NewStackItem) -> StackItem {
        let id = insert_one_into(&self.db.get(), "stack", &stack_item, self.logger()).await;
        find_one_by_id(&self.db.get(), "stack", &id, self.logger())
            .await
            .unwrap()
    }

    async fn link_blocks(&self, stack_item: &StackItem, block_ids: &Vec<Id>) -> StackItem {
        link_external_ids(
            &self.db.get(),
            "stack",
            &stack_item.id,
            "block_ids",
            block_ids,
        )
        .await;

        find_one_by_id(&self.db.get(), "stack", &stack_item.id, self.logger())
            .await
            .unwrap()
    }

    async fn link_marks(&self, stack_item: &StackItem, mark_ids: &Vec<Id>) -> StackItem {
        link_external_ids(
            &self.db.get(),
            "stack",
            &stack_item.id,
            "mark_ids",
            mark_ids,
        )
        .await;

        find_one_by_id(&self.db.get(), "stack", &stack_item.id, self.logger())
            .await
            .unwrap()
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
    }
}
