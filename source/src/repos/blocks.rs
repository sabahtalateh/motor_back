use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{
    delete_by_id, find_many_by_ids, find_one_by_id, inc_version, insert_one_into,
    link_external_ids, set_by_id,
};
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId, ToDocsVec};

use crate::repos::Id;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{Bson, Document};
use juniper::futures::StreamExt;
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    #[serde(rename = "_id")]
    pub id: Id,
    pub stack_id: Id,
    pub text: String,
    pub marks_ids: Vec<Id>,
    pub current_version: i32,
    pub initial_version: i32,
}

#[derive(Serialize, Debug)]
struct InsertBlock {
    stack_id: Id,
    text: String,
    marks_ids: Vec<Id>,
    current_version: i32,
    initial_version: i32,
}

#[async_trait]
pub trait BlocksRepoIf: Interface {
    async fn insert(&self, stack_id: &Id, text: &str) -> Block;

    async fn delete(&self, id: &Id) -> bool;

    /// returns (old_block, new_block)
    async fn update(&self, old: &Block, new_text: &str) -> (Block, Block);

    async fn link_marks(&self, block: &Block, marks_ids: &Vec<Id>) -> Block;

    async fn find_by_ids(&self, ids: &Vec<Id>) -> Vec<Block>;
}

#[shaku(interface = BlocksRepoIf)]
#[derive(Component, HasLogger)]
pub struct BlocksRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl BlocksRepoIf for BlocksRepo {
    async fn insert(&self, stack_id: &Id, text: &str) -> Block {
        let id = insert_one_into(
            &self.db.get(),
            "blocks",
            &InsertBlock {
                stack_id: stack_id.clone(),
                text: text.to_owned(),
                marks_ids: vec![],
                current_version: 0,
                initial_version: 0,
            },
            self.logger(),
        )
        .await;

        Block {
            id: id.into(),
            stack_id: stack_id.clone(),
            text: text.to_owned(),
            marks_ids: vec![],
            current_version: 0,
            initial_version: 0,
        }
    }

    async fn delete(&self, id: &Id) -> bool {
        set_by_id(&self.db.get(), "blocks", id, doc! {"removed": true}).await
    }

    async fn update(&self, old: &Block, new_text: &str) -> (Block, Block) {
        let old = old.clone();

        let inserted_id = insert_one_into(
            &self.db.get(),
            "blocks",
            &InsertBlock {
                stack_id: old.stack_id,
                text: old.text,
                marks_ids: old.marks_ids,
                current_version: 0,
                initial_version: 0,
            },
            self.logger(),
        )
        .await;

        set_by_id(&self.db.get(), "blocks", &old.id, doc! { "text": new_text }).await;
        inc_version(&self.db.get(), "blocks", &old.id).await;

        let old_block = find_one_by_id(&self.db.get(), "blocks", &inserted_id, self.logger())
            .await
            .unwrap();
        let new_block = find_one_by_id(&self.db.get(), "blocks", &old.id, self.logger())
            .await
            .unwrap();

        (old_block, new_block)
    }

    async fn link_marks(&self, block: &Block, marks_ids: &Vec<Id>) -> Block {
        link_external_ids(&self.db.get(), "blocks", &block.id, "marks_ids", marks_ids).await;

        find_one_by_id(&self.db.get(), "blocks", &block.id, self.logger())
            .await
            .unwrap()
    }

    async fn find_by_ids(&self, ids: &Vec<Id>) -> Vec<Block> {
        find_many_by_ids(&self.db.get(), "blocks", ids, self.logger()).await
    }
}
