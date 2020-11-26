use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{find_many_by, find_many_by_ids};
use crate::repos::db::{find_one_by_id, insert_one_into, set_by_id};
use crate::repos::{Id, Repo};
use async_trait::async_trait;
use bson::oid::ObjectId;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

pub const COLLECTION: &str = "sets";

#[derive(Serialize)]
pub struct InsertSet {
    pub creator_id: Id,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Set {
    #[serde(rename = "_id")]
    pub id: Id,
    pub name: String,
    pub creator_id: Id,
}

#[async_trait]
pub trait SetsRepoIf: Interface + Repo<Set, InsertSet> {}

#[shaku(interface = SetsRepoIf)]
#[derive(Component, HasLogger)]
pub struct SetsRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl Repo<Set, InsertSet> for SetsRepo {
    async fn find(&self, id: &Id) -> Option<Set> {
        find_one_by_id(&self.db.get(), COLLECTION, id, self.logger()).await
    }

    async fn find_many(&self, ids: Vec<&Id>) -> Vec<Set> {
        unimplemented!()
    }

    async fn insert(&self, insert: InsertSet) -> Set {
        let id = insert_one_into(&self.db.get(), COLLECTION, &insert, self.logger()).await;
        Set {
            id,
            creator_id: insert.creator_id,
            name: insert.name,
        }
    }
}

#[async_trait]
impl SetsRepoIf for SetsRepo {}
