use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{
    delete_many_by, find_many_by, find_one_by, find_one_by_id, insert_many_into,
    paged_find_many_by, PaginationOptions,
};
use crate::repos::{Id, Repo};
use async_trait::async_trait;
use bson::oid::ObjectId;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

pub const COLLECTION: &str = "recent_sets";

#[derive(Debug, Serialize)]
pub struct InsertRecentSet {
    pub user_id: Id,
    pub set_id: Id,
}

#[derive(Deserialize)]
pub struct RecentSet {
    #[serde(rename = "_id")]
    pub id: Id,
    pub user_id: Id,
    pub set_id: Id,
}

#[async_trait]
pub trait RecentSetsRepoIf: Interface + Repo<RecentSet, InsertRecentSet> {
    async fn find_by_user_id(&self, id: &Id) -> Vec<RecentSet>;
}

#[shaku(interface = RecentSetsRepoIf)]
#[derive(Component, HasLogger)]
pub struct RecentSetsRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl RecentSetsRepoIf for RecentSetsRepo {
    async fn find_by_user_id(&self, id: &Id) -> Vec<RecentSet> {
        println!("{:#?}", doc! {"user_id": id.oid()});

        find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"user_id": id.oid()},
            &self.logger(),
        )
        .await
    }
}

#[async_trait]
impl Repo<RecentSet, InsertRecentSet> for RecentSetsRepo {
    async fn insert(&self, insert: InsertRecentSet) -> RecentSet {
        unimplemented!()
    }

    async fn insert_many(&self, insert: Vec<&InsertRecentSet>) {
        insert_many_into(&self.db.get(), COLLECTION, insert, &self.logger()).await;
    }

    async fn find(&self, id: &Id) -> Option<RecentSet> {
        unimplemented!()
    }

    async fn find_many(&self, ids: Vec<&Id>) -> Vec<RecentSet> {
        unimplemented!()
    }

    async fn delete(&self, id: &Id) {
        unimplemented!()
    }

    async fn delete_many(&self, ids: Vec<&Id>) {
        let ids: Vec<ObjectId> = ids.iter().map(|x| x.oid()).collect();
        delete_many_by(&self.db.get(), COLLECTION, doc! {"_id": {"$in": ids}}).await;
    }
}
