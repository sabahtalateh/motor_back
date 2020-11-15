use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::errors::AppError;
use crate::handlers::groups::{RemovedGroup, UserGroup};
use crate::logger::AppLoggerIf;
use crate::repos::groups::{GroupsRepoIf, InsertGroup};
use crate::repos::groups_ordering::{GroupsOrderingRepoIf, InsertGroupOrder};
use crate::repos::Id;
use crate::repos::users::User;
use crate::services::{PagedResponse, Paging};
use crate::utils::AppResult;

pub const PAGING_MAX_LIMIT: i32 = 1000;

fn recount_ordering(ordering: &mut Vec<InsertGroupOrder>) {
    for (i, ordering) in ordering.iter_mut().enumerate() {
        ordering.order = i as i32
    }
}

#[async_trait]
pub trait GroupsServiceIf: Interface {
    async fn add(&self, user: &User, name: &str, after: Option<&Id>) -> AppResult<UserGroup>;
    async fn remove(&self, user: &User, id: &Id) -> AppResult<RemovedGroup>;
    async fn list(&self, user: &User, paging: &Paging) -> AppResult<PagedResponse<UserGroup>>;
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
    async fn add(&self, user: &User, name: &str, after: Option<&Id>) -> AppResult<UserGroup> {
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
                removed: false,
            })
            .await;

        let mut ordering: Vec<InsertGroupOrder> = self
            .groups_ordering_repo
            .get_by_user_id(&user.id)
            .await
            .iter()
            .enumerate()
            .map(|(i, o)| InsertGroupOrder {
                user_id: o.user_id.clone(),
                group_id: o.group_id.clone(),
                order: i as i32,
            })
            .collect();

        let new_group_position = match after {
            None => 0,
            Some(id) => match ordering.iter().position(|o| &o.group_id == id) {
                Some(pos) => pos + 1,
                None => 0,
            },
        };

        ordering.insert(
            new_group_position,
            InsertGroupOrder {
                user_id: user.id.clone(),
                group_id: group_entity.id.clone(),
                order: new_group_position as i32,
            },
        );

        recount_ordering(&mut ordering);

        self.groups_ordering_repo.delete_by_user_id(&user.id).await;
        self.groups_ordering_repo.insert(ordering).await;

        Ok(UserGroup {
            id: group_entity.id,
            name: group_entity.name,
            order: new_group_position as i32,
        })
    }

    async fn remove(&self, user: &User, id: &Id) -> AppResult<RemovedGroup> {
        let mut ordering = self.groups_ordering_repo.get_by_user_id(&user.id).await;

        let remove_group_position =
            ordering
                .iter()
                .position(|g| &g.group_id == id)
                .ok_or(AppError::validation(&format!(
                    "Group `{}` you are trying to remove not exists",
                    id
                )))?;

        let removed_order = ordering.remove(remove_group_position);
        self.groups_repo.mark_removed(&removed_order.group_id).await;

        let mut new_ordering: Vec<InsertGroupOrder> = ordering
            .iter()
            .map(|o| InsertGroupOrder {
                user_id: o.user_id.clone(),
                group_id: o.group_id.clone(),
                order: o.order,
            })
            .collect();

        recount_ordering(&mut new_ordering);
        self.groups_ordering_repo.delete_by_user_id(&user.id).await;
        self.groups_ordering_repo.insert(new_ordering).await;

        let group_entity = self
            .groups_repo
            .find(&removed_order.group_id)
            .await
            .unwrap();

        Ok(RemovedGroup {
            id: removed_order.group_id,
            name: group_entity.name,
        })
    }

    async fn list(&self, user: &User, paging: &Paging) -> AppResult<PagedResponse<UserGroup>> {
        if paging.limit > PAGING_MAX_LIMIT {
            return Err(AppError::validation(&format!(
                "Paging limit can not be more then {}",
                PAGING_MAX_LIMIT
            )));
        }

        let groups_ordering = self
            .groups_ordering_repo
            .get_paged_by_user_id(&user.id, paging.offset, paging.limit)
            .await;

        let groups_ids: Vec<&Id> = groups_ordering.iter().map(|g| &g.group_id).collect();
        let groups = self.groups_repo.find_by_ids(groups_ids).await;

        let mut res: Vec<UserGroup> = vec![];
        for order in groups_ordering {
            if let Some(group) = groups.iter().find(|g| g.id == order.group_id) {
                res.push(UserGroup {
                    id: group.id.clone(),
                    name: group.name.clone(),
                    order: order.order,
                })
            }
        }

        Ok(PagedResponse {
            total: res.len() as i32,
            offset: paging.offset,
            limit: paging.limit,
            objects: res,
        })
    }
}
