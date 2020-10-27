use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{find_one_by_id, insert_one_into, link_external_ids};
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId};

use crate::repos::Id;
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

pub const COLLECTION: &str = "stack";

#[async_trait]
pub trait StackRepoIf: Interface {
    async fn insert(&self, stack_item: &NewStackItem) -> StackItem;
    async fn update(&self, stack_item: &StackItem) -> StackItem;
    async fn link_blocks(&self, stack_item: &StackItem, blocks_ids: &Vec<Id>) -> StackItem;
    async fn link_marks(&self, stack_item: &StackItem, marks_ids: &Vec<Id>) -> StackItem;
    async fn find_by_user_id(&self, user_id: Id) -> Vec<StackItem>;
    async fn find_by_user_id_and_stack_item_id(
        &self,
        user_id: Id,
        stack_item_id: Id,
    ) -> Option<StackItem>;
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
    pub blocks_ids: Vec<Id>,
    pub marks_ids: Vec<Id>,
    pub version: i32,
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

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct StackItem {
    #[serde(rename = "_id")]
    pub id: Id,
    pub blocks_ids: Vec<Id>,
    pub marks_ids: Vec<Id>,
}

#[async_trait]
impl StackRepoIf for StackRepo {
    async fn insert(&self, stack_item: &NewStackItem) -> StackItem {
        let id = insert_one_into(&self.db.get(), COLLECTION, &stack_item, self.logger()).await;
        find_one_by_id(&self.db.get(), COLLECTION, &id, self.logger())
            .await
            .unwrap()
    }

    async fn update(&self, stack_item: &StackItem) -> StackItem {
        let id: ObjectId = stack_item.id.clone().into();

        let doc: Document = bson::to_bson(&stack_item)
            .unwrap()
            .as_document()
            .unwrap()
            .clone();

        // self.db
        //     .get()
        //     .collection("stack")
        //     .update_one(doc! {"_id": id}, doc, None);

        unimplemented!()
    }

    async fn link_blocks(&self, stack_item: &StackItem, blocks_ids: &Vec<Id>) -> StackItem {
        link_external_ids(
            &self.db.get(),
            COLLECTION,
            &stack_item.id,
            "blocks_ids",
            blocks_ids,
        )
        .await;

        find_one_by_id(&self.db.get(), "stack", &stack_item.id, self.logger())
            .await
            .unwrap()
    }

    async fn link_marks(&self, stack_item: &StackItem, marks_ids: &Vec<Id>) -> StackItem {
        link_external_ids(
            &self.db.get(),
            COLLECTION,
            &stack_item.id,
            "marks_ids",
            marks_ids,
        )
        .await;

        find_one_by_id(&self.db.get(), COLLECTION, &stack_item.id, self.logger())
            .await
            .unwrap()
    }

    async fn find_by_user_id(&self, user_id: Id) -> Vec<StackItem> {
        let user_id: ObjectId = user_id.into();

        self.db
            .get()
            .collection(COLLECTION)
            .find(Some(doc! {"user_id": user_id}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|x| deserialize_bson(&x.unwrap()))
            .collect()
            .await
    }

    async fn find_by_user_id_and_stack_item_id(
        &self,
        user_id: Id,
        stack_item_id: Id,
    ) -> Option<StackItem> {
        let user_id: ObjectId = user_id.into();
        let stack_item_id: ObjectId = stack_item_id.into();

        self.db
            .get()
            .collection(COLLECTION)
            .find_one(Some(doc! {"user_id": user_id, "_id": stack_item_id}), None)
            .await
            .log_err_with(self.logger())
            .into_app_err()
            .unwrap()
            .map(|u| deserialize_bson(&u))
    }
}
