use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{delete_by, find_one_by, find_one_by_id, insert_many_into, insert_one_into};
use crate::repos::Id;
use crate::utils::{deserialize_bson, AppResult, IntoAppErr, LogErrWith, OkOrMongoRecordId, Refs};

use crate::repos::db::{find_many_by, find_many_by_ids};
use crate::repos::groups::Group;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{Bson, Document};
use juniper::futures::StreamExt;
use juniper::GraphQLObject;
use mongodb::options::{FindOptions, UpdateOptions};
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

pub const COLLECTION: &str = "groups_ordering";

#[derive(Debug, Serialize)]
pub struct InsertGroupOrder {
    pub user_id: Id,
    pub group_id: Id,
    pub order: i32,
}

#[derive(Debug, Deserialize)]
pub struct GroupOrder {
    #[serde(rename = "_id")]
    pub id: Id,
    pub user_id: Id,
    pub group_id: Id,
    pub order: i32,
}

#[async_trait]
pub trait GroupsOrderingRepoIf: Interface {
    async fn insert(&self, ordering: Vec<InsertGroupOrder>);
    async fn get_by_user_id(&self, user_id: &Id) -> Vec<GroupOrder>;
    async fn get_paged_by_user_id(&self, user_id: &Id, offset: i32, limit: i32) -> Vec<GroupOrder>;
    async fn delete_by_user_id(&self, user_id: &Id);
}

#[shaku(interface = GroupsOrderingRepoIf)]
#[derive(Component, HasLogger)]
pub struct GroupsOrderingRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl GroupsOrderingRepoIf for GroupsOrderingRepo {
    async fn insert(&self, ordering: Vec<InsertGroupOrder>) {
        insert_many_into(&self.db.get(), COLLECTION, ordering.refs(), self.logger()).await;
    }

    async fn get_by_user_id(&self, user_id: &Id) -> Vec<GroupOrder> {
        let user_id: ObjectId = user_id.clone().into();

        find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"user_id": user_id},
            self.logger(),
            None,
        )
        .await
    }

    async fn get_paged_by_user_id(&self, user_id: &Id, offset: i32, limit: i32) -> Vec<GroupOrder> {
        let user_id: ObjectId = user_id.clone().into();

        find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"user_id": user_id},
            self.logger(),
            Some(FindOptions::builder().skip(offset as i64).limit(limit as i64).build()),
        )
        .await
    }

    async fn delete_by_user_id(&self, user_id: &Id) {
        let user_id: ObjectId = user_id.clone().into();
        delete_by(&self.db.get(), COLLECTION, doc! {"user_id": user_id}).await;
    }
}
