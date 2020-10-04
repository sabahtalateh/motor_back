use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::{insert_one_into, Id};
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId, ToDocsVec};

use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{Bson, Document};
use juniper::futures::StreamExt;
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Block {
    pub id: Id,
    pub stack_id: Id,
    pub text: String,
}

#[derive(Serialize, Debug)]
pub struct NewBlock {
    pub stack_id: Id,
    pub text: String,
}

#[async_trait]
pub trait BlocksRepoIf: Interface {
    async fn insert(&self, new_block: &NewBlock) -> Block;
    async fn insert_many(&self, new_blocks: &Vec<NewBlock>) -> Vec<Block>;
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
    async fn insert(&self, new_block: &NewBlock) -> Block {
        let id = insert_one_into(&self.db.get(), "blocks", new_block, self.logger()).await;

        Block {
            id: id.into(),
            stack_id: new_block.stack_id.clone(),
            text: new_block.text.clone(),
        }
    }

    async fn insert_many(&self, new_blocks: &Vec<NewBlock>) -> Vec<Block> {
        let docs_vec: Vec<Document> = new_blocks.to_documents_vec();

        println!("{:?}", docs_vec);

        let inserted_result = self
            .db
            .get()
            .collection("blocks")
            .insert_many(docs_vec, None)
            .await
            .log_err_with(self.logger())
            .unwrap();

        let mut out = vec![];
        for i in 0..new_blocks.len() {
            let block = new_blocks.get(i).unwrap();
            let inserted_id = match inserted_result.inserted_ids.get(&i).unwrap() {
                Bson::ObjectId(oid) => oid,
                _ => unreachable!(),
            };

            out.push(Block {
                id: inserted_id.clone().into(),
                stack_id: block.stack_id.clone(),
                text: block.text.clone(),
            })
        }

        out
    }
}
