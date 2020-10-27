use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{insert_many_into, insert_one_into};
use crate::repos::Id;
use crate::utils::{deserialize_bson, AppResult, IntoAppErr, LogErrWith, OkOrMongoRecordId, Refs};

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

pub const COLLECTION: &str = "groups_ordering";

#[derive(Serialize)]
pub struct InsertGroupOrder {
    pub user_id: Id,
    pub group_id: Id,
    pub order: i32,
}

#[async_trait]
pub trait GroupsOrderingRepoIf: Interface {
    async fn insert(&self, order: InsertGroupOrder) -> Id;
    async fn increment_orders(&self, user_id: &Id, start_from_order: i32);
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
    async fn insert(&self, order: InsertGroupOrder) -> Id {
        insert_one_into(&self.db.get(), COLLECTION, &order, self.logger()).await
    }

    async fn increment_orders(&self, user_id: &Id, start_from_order: i32) {
        let user_id: ObjectId = user_id.clone().into();

        let condition = doc! {
            "$and": [
                { "order": { "$gte": start_from_order } },
                { "user_id": user_id }
            ]
        };

        self.db
            .get()
            .collection(COLLECTION)
            .update_many(condition, doc! { "$inc": { "order": 1 } }, None)
            .await;
    }
}
