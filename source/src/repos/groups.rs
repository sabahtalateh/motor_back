use std::sync::Arc;

use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{find_many_by, find_many_by_ids};
use crate::repos::db::{find_one_by, find_one_by_id, insert_one_into, set_by_id};
use crate::repos::Id;

pub const COLLECTION: &str = "groups";

#[derive(Serialize)]
pub struct InsertGroup {
    pub creator_id: Id,
    pub name: String,
    pub removed: bool,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: Id,
    pub creator_id: Id,
    pub name: String,
    pub removed: bool,
}

#[async_trait]
pub trait GroupsRepoIf: Interface {
    async fn find(&self, id: &Id) -> Option<Group>;
    async fn find_by_ids(&self, ids: Vec<&Id>) -> Vec<Group>;
    async fn get_by_creator_id_and_name(&self, creator_id: &Id, name: &str) -> Vec<Group>;
    async fn insert(&self, group: InsertGroup) -> Group;
    async fn mark_removed(&self, group_id: &Id) -> bool;
}

#[shaku(interface = GroupsRepoIf)]
#[derive(Component, HasLogger)]
pub struct GroupsRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl GroupsRepoIf for GroupsRepo {
    async fn find(&self, id: &Id) -> Option<Group> {
        find_one_by_id(&self.db.get(), COLLECTION, id, self.logger()).await
    }

    async fn find_by_ids(&self, ids: Vec<&Id>) -> Vec<Group> {
        find_many_by_ids(&self.db.get(), COLLECTION, ids, self.logger()).await
    }

    async fn get_by_creator_id_and_name(&self, creator_id: &Id, name: &str) -> Vec<Group> {
        let creator_id: ObjectId = creator_id.clone().into();
        find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"creator_id": creator_id, "name": name},
            self.logger(),
        )
        .await
    }

    async fn insert(&self, group: InsertGroup) -> Group {
        let id = insert_one_into(&self.db.get(), COLLECTION, &group, self.logger()).await;
        Group {
            id,
            creator_id: group.creator_id,
            name: group.name,
            removed: false,
        }
    }

    async fn mark_removed(&self, group_id: &Id) -> bool {
        set_by_id(
            &self.db.get(),
            COLLECTION,
            &group_id,
            doc! { "removed": true },
        )
        .await
    }
}
