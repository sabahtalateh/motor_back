use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::db::find_one_by_id;
use crate::repos::groups::{GroupsRepoIf, InsertGroup};
use crate::repos::groups_ordering::{GroupsOrderingRepoIf, InsertGroupOrder};
use crate::repos::tokens::{TokenPair, TokensRepoIf};
use crate::repos::users::{NewUser, User, UsersRepoIf};
use crate::repos::Id;
use crate::utils::{AppResult, IntoAppErr, LogErrWith, OkOrUnauthorized};
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait GroupsServiceIf: Interface {
    async fn create(&self, user: &User, name: &str, after: Option<&Id>) -> AppResult<Id>;
}

#[derive(Component, HasLogger)]
#[shaku(interface = GroupsServiceIf)]
pub struct GroupsService {
    #[shaku(inject)]
    groups_repo: Arc<dyn GroupsRepoIf>,

    #[shaku(inject)]
    groups_ordering_repo: Arc<dyn GroupsOrderingRepoIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl GroupsServiceIf for GroupsService {
    async fn create(&self, user: &User, name: &str, after: Option<&Id>) -> AppResult<Id> {
        // проверить есть ли уже такая группа у пользователя

        let group_entity = self
            .groups_repo
            .insert(InsertGroup {
                creator_id: (&user.id).clone(),
                name: name.to_string(),
            })
            .await;

        let order = 0;
        // match after {
        // Some(order) =>
        // }

        self.groups_ordering_repo
            .increment_orders(&user.id, order)
            .await;
        self.groups_ordering_repo.insert(InsertGroupOrder {
            user_id: (&user.id).clone(),
            group_id: group_entity.id,
            order,
        }).await;

        unimplemented!()
    }
}
