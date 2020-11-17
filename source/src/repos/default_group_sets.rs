use std::sync::Arc;

use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{delete_by, find_many_by, insert_many_into};
use crate::repos::Id;
use crate::utils::Refs;

pub const COLLECTION: &str = "default_group_sets";

#[derive(Serialize)]
pub struct InsertDefaultGroupSetItem {
    pub user_id: Id,
    pub group_id: Id,
    pub group_name: String,
    pub order: i32,
}

#[derive(Debug, Deserialize)]
pub struct DefaultGroupSetItem {
    #[serde(rename = "_id")]
    pub id: Id,
    pub user_id: Id,
    pub group_id: Id,
    pub group_name: String,
    pub order: i32,
}

#[async_trait]
pub trait DefaultGroupSetsRepoIf: Interface {
    async fn insert(&self, items: Vec<&InsertDefaultGroupSetItem>);
    async fn get_by_user_id(&self, user_id: &Id) -> Vec<DefaultGroupSetItem>;
    async fn remove_by_user_id(&self, user_id: &Id);
}

#[shaku(interface = DefaultGroupSetsRepoIf)]
#[derive(Component, HasLogger)]
pub struct DefaultGroupSetsRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl DefaultGroupSetsRepoIf for DefaultGroupSetsRepo {
    async fn insert(&self, items: Vec<&InsertDefaultGroupSetItem>) {
        insert_many_into(&self.db.get(), COLLECTION, items, self.logger()).await;
    }

    async fn get_by_user_id(&self, user_id: &Id) -> Vec<DefaultGroupSetItem> {
        let user_id: ObjectId = user_id.clone().into();

        find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"user_id": user_id},
            self.logger(),
        )
        .await
    }

    async fn remove_by_user_id(&self, user_id: &Id) {
        let user_id: ObjectId = user_id.clone().into();

        delete_by(&self.db.get(), COLLECTION, doc! {"user_id": user_id}).await;
    }
}
