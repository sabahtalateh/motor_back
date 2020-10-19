use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId};

use crate::repos::db::{find_many_by_ids, find_many_by};
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

#[derive(Clone, Deserialize, Debug)]
pub struct Mark {
    #[serde(rename = "_id")]
    pub id: Id,
    pub block_id: Id,
    pub from: i32,
    pub to: i32,
}

#[derive(Serialize)]
pub struct InsertMark {
    pub block_id: Id,
    pub from: i32,
    pub to: i32,
}

#[async_trait]
pub trait MarksRepoIf: Interface {
    async fn insert_many(&self, new_marks: &Vec<InsertMark>) -> Vec<Mark>;
    async fn find_by_ids(&self, ids: &Vec<Id>) -> Vec<Mark>;
    async fn find_by_block_id(&self, block_id: &Id) -> Vec<Mark>;
}

#[shaku(interface = MarksRepoIf)]
#[derive(Component, HasLogger)]
pub struct MarksRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl MarksRepoIf for MarksRepo {
    async fn insert_many(&self, new_marks: &Vec<InsertMark>) -> Vec<Mark> {
        if new_marks.len() == 0 {
            return vec![];
        }

        let insert_marks: Vec<InsertMark> = new_marks
            .iter()
            .map(|m| InsertMark {
                block_id: m.block_id.clone(),
                from: m.from,
                to: m.to,
            })
            .collect();

        let docs_vec: Vec<Document> = insert_marks
            .iter()
            .map(|x| bson::to_bson(x).unwrap().as_document().unwrap().clone())
            .collect();

        let inserted_result = self
            .db
            .get()
            .collection("marks")
            .insert_many(docs_vec, None)
            .await
            .log_err_with(self.logger())
            .unwrap();

        let mut out = vec![];
        for i in 0..insert_marks.len() {
            let mark = insert_marks.get(i).unwrap();
            let inserted_id = match inserted_result.inserted_ids.get(&i).unwrap() {
                Bson::ObjectId(oid) => oid,
                _ => unreachable!(),
            };

            out.push(Mark {
                id: inserted_id.clone().into(),
                block_id: mark.block_id.clone(),
                from: mark.from,
                to: mark.to,
            })
        }

        out
    }

    async fn find_by_ids(&self, ids: &Vec<Id>) -> Vec<Mark> {
        find_many_by_ids(&self.db.get(), "marks", ids, self.logger()).await
    }

    async fn find_by_block_id(&self, block_id: &Id) -> Vec<Mark> {
        let block_id: ObjectId = block_id.clone().into();
        find_many_by(&self.db.get(), "marks", doc!{ "block_id": block_id }, self.logger()).await
    }
}
