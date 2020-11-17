use std::sync::Arc;

use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{delete_by, find_many_by, find_one_by, insert_many_into};
use crate::repos::Id;

pub const COLLECTION: &str = "group_sets";

#[derive(Serialize)]
pub struct InsertGroupSetItem {
    pub user_id: Id,
    pub group_id: Id,
    pub group_name: String,
    pub set_name: String,
    pub order: i32,
}

#[derive(Debug, Deserialize)]
pub struct GroupSetItem {
    #[serde(rename = "_id")]
    pub id: Id,
    pub user_id: Id,
    pub group_id: Id,
    pub group_name: String,
    pub set_name: String,
    pub order: i32,
}

#[async_trait]
pub trait GroupSetsRepoIf: Interface {
    async fn insert(&self, items: Vec<&InsertGroupSetItem>);
    async fn find_by_user_id_set_name_and_group_name(
        &self,
        user_id: &Id,
        set_name: &str,
        group_name: &str,
    ) -> Option<GroupSetItem>;
    async fn get_by_user_id_and_set_name(&self, user_id: &Id, set_name: &str) -> Vec<GroupSetItem>;
    async fn remove_by_set_name_and_user_id(&self, set_name: &str, user_id: &Id);
}

#[shaku(interface = GroupSetsRepoIf)]
#[derive(Component, HasLogger)]
pub struct GroupSetsRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl GroupSetsRepoIf for GroupSetsRepo {
    async fn insert(&self, items: Vec<&InsertGroupSetItem>) {
        insert_many_into(&self.db.get(), COLLECTION, items, self.logger()).await;
    }

    async fn find_by_user_id_set_name_and_group_name(
        &self,
        user_id: &Id,
        set_name: &str,
        group_name: &str,
    ) -> Option<GroupSetItem> {
        let user_id: ObjectId = user_id.clone().into();

        find_one_by(
            &self.db.get(),
            COLLECTION,
            doc! {
                "user_id": user_id,
                "set_name": set_name,
                "group_name": group_name,
            },
            self.logger(),
        ).await
    }

    async fn get_by_user_id_and_set_name(&self, user_id: &Id, set_name: &str) -> Vec<GroupSetItem> {
        let user_id: ObjectId = user_id.clone().into();

        find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"set_name": set_name, "user_id": user_id},
            self.logger(),
        )
        .await
    }

    async fn remove_by_set_name_and_user_id(&self, set_name: &str, user_id: &Id) {
        let user_id: ObjectId = user_id.clone().into();

        delete_by(
            &self.db.get(),
            COLLECTION,
            doc! {"set_name": set_name, "user_id": user_id},
        )
        .await;
    }
}
