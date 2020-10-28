use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{find_one_by, find_one_by_id, insert_many_into, insert_one_into};
use crate::repos::Id;
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId, Refs};

use crate::repos::db::{find_many_by, find_many_by_ids};
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

pub const COLLECTION: &str = "groups";

#[derive(Serialize)]
pub struct InsertGroup {
    pub creator_id: Id,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename="_id")]
    pub id: Id,
    pub creator_id: Id,
    pub name: String,
}

#[async_trait]
pub trait GroupsRepoIf: Interface {
    async fn find_by_creator_id_and_name(&self, creator_id: &Id, name: &str) -> Option<Group>;
    async fn insert(&self, group: InsertGroup) -> Group;
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
    async fn find_by_creator_id_and_name(&self, creator_id: &Id, name: &str) -> Option<Group> {
        let creator_id: ObjectId = creator_id.clone().into();
        find_one_by(
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
        }
    }
}
