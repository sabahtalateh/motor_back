use crate::errors::AppError;
use crate::handlers::groups::UserGroup;
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
    async fn create(&self, user: &User, name: &str, after: Option<&Id>) -> AppResult<UserGroup>;
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
    async fn create(&self, user: &User, name: &str, after: Option<&Id>) -> AppResult<UserGroup> {
        if let Some(_) = self
            .groups_repo
            .find_by_creator_id_and_name(&user.id, name)
            .await
        {
            return Err(AppError::validation(&format!(
                "Group `{}` already exists",
                name
            )));
        }

        let group_entity = self
            .groups_repo
            .insert(InsertGroup {
                creator_id: (&user.id).clone(),
                name: name.to_string(),
            })
            .await;

        let order_of_new_group = match after {
            Some(group_id) => match self
                .groups_ordering_repo
                .find_by_user_id_and_group_id(&user.id, group_id)
                .await
            {
                Some(group) => group.order + 1,
                None => 0,
            },
            None => 0,
        };

        self.groups_ordering_repo
            .increment_orders_by_user_id(&user.id, order_of_new_group)
            .await;

        let new_group_id = self
            .groups_ordering_repo
            .insert(InsertGroupOrder {
                user_id: (&user.id).clone(),
                group_id: group_entity.id,
                order: order_of_new_group,
            })
            .await;

        Ok(UserGroup {
            id: new_group_id,
            name: name.to_string(),
            order: order_of_new_group,
        })
    }
}
