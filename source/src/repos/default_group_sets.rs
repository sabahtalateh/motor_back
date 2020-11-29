use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{
    delete_many_by, find_many_by, find_one_by, find_one_by_id, insert_many_into, paged_find_many_by,
    PaginationOptions,
};
use crate::repos::Id;
use async_trait::async_trait;
use bson::oid::ObjectId;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

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
    async fn find(&self, id: &Id) -> Option<DefaultGroupSetItem>;

    async fn find_by_group_id(&self, group_id: &Id) -> Option<DefaultGroupSetItem>;

    async fn insert_many(&self, items: Vec<&InsertDefaultGroupSetItem>);

    async fn find_by_user_id(&self, user_id: &Id) -> Option<DefaultGroupSetItem>;

    async fn get_paged_by_user_id(
        &self,
        user_id: &Id,
        offset: i32,
        limit: i32,
    ) -> Vec<DefaultGroupSetItem>;

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
    async fn find(&self, id: &Id) -> Option<DefaultGroupSetItem> {
        find_one_by_id(&self.db.get(), COLLECTION, id, self.logger()).await
    }

    async fn find_by_group_id(&self, group_id: &Id) -> Option<DefaultGroupSetItem> {
        let group_id: ObjectId = group_id.clone().into();

        find_one_by(
            &self.db.get(),
            COLLECTION,
            doc! {"group_id": group_id},
            self.logger(),
        )
        .await
    }

    async fn insert_many(&self, items: Vec<&InsertDefaultGroupSetItem>) {
        insert_many_into(&self.db.get(), COLLECTION, items, self.logger()).await;
    }

    async fn find_by_user_id(&self, user_id: &Id) -> Option<DefaultGroupSetItem> {
        let user_id: ObjectId = user_id.clone().into();

        find_one_by(
            &self.db.get(),
            COLLECTION,
            doc! {"user_id": user_id},
            self.logger(),
        )
        .await
    }

    async fn get_paged_by_user_id(
        &self,
        user_id: &Id,
        offset: i32,
        limit: i32,
    ) -> Vec<DefaultGroupSetItem> {
        let user_id: ObjectId = user_id.clone().into();

        paged_find_many_by(
            &self.db.get(),
            COLLECTION,
            doc! {"user_id": user_id},
            self.logger(),
            PaginationOptions {
                offset: offset as i64,
                limit: limit as i64,
            },
        )
        .await
    }

    async fn remove_by_user_id(&self, user_id: &Id) {
        let user_id: ObjectId = user_id.clone().into();

        delete_many_by(&self.db.get(), COLLECTION, doc! {"user_id": user_id}).await;
    }
}
