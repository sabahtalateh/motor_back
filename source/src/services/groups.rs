use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::errors::AppError;
use crate::handlers::groups::{RemovedGroup, UserGroup};
use crate::handlers::Paging;
use crate::logger::AppLoggerIf;
use crate::repos::default_group_sets::{DefaultGroupSetsRepoIf, InsertDefaultGroupSetItem};
use crate::repos::group_sets::{GroupSetsRepoIf, InsertGroupSetItem};
use crate::repos::groups::{Group, GroupsRepoIf, InsertGroup};
use crate::repos::groups_ordering::GroupsOrderingRepoIf;
use crate::repos::Id;
use crate::repos::users::User;
use crate::services::{Paged, PageInfo};
use crate::utils::{AppResult, Refs};

pub const PAGING_MAX_LIMIT: i32 = 1000;

fn recount_default_ordering(group_set: &mut Vec<InsertDefaultGroupSetItem>) {
    for (i, set_item) in group_set.iter_mut().enumerate() {
        set_item.order = i as i32
    }
}

fn recount_ordering(group_set: &mut Vec<InsertGroupSetItem>) {
    for (i, set_item) in group_set.iter_mut().enumerate() {
        set_item.order = i as i32
    }
}

#[async_trait]
pub trait GroupsServiceIf: Interface {
    async fn create_group(
        &self,
        user: &User,
        name: &str,
        into_set: Option<&str>,
        after_group: Option<&Id>,
    ) -> AppResult<UserGroup>;
    async fn remove(&self, user: &User, id: &Id) -> AppResult<RemovedGroup>;
    async fn list(
        &self,
        user: &User,
        set_name: Option<&str>,
        paging: Option<Paging>,
    ) -> AppResult<Paged<UserGroup>>;
    // ) -> Vec<UserGroup>;
}

#[derive(Component, HasLogger)]
#[shaku(interface = GroupsServiceIf)]
pub struct GroupsService {
    #[shaku(inject)]
    groups_repo: Arc<dyn GroupsRepoIf>,

    #[shaku(inject)]
    groups_ordering_repo: Arc<dyn GroupsOrderingRepoIf>,

    #[shaku(inject)]
    default_group_sets_repo: Arc<dyn DefaultGroupSetsRepoIf>,

    #[shaku(inject)]
    group_sets_repo: Arc<dyn GroupSetsRepoIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

impl GroupsService {
    async fn insert_group_into_set(
        &self,
        user: &User,
        group: &Group,
        set_name: &str,
        after_group: &Option<&Id>,
    ) -> usize {
        let mut group_set: Vec<InsertGroupSetItem> = self
            .group_sets_repo
            .get_by_user_id_and_set_name(&user.id, set_name)
            .await
            .iter()
            .enumerate()
            .map(|(i, group)| InsertGroupSetItem {
                user_id: group.user_id.clone(),
                group_id: group.group_id.clone(),
                group_name: group.group_name.clone(),
                set_name: set_name.to_string(),
                order: i as i32,
            })
            .collect();

        let new_group_position = match after_group {
            None => 0,
            Some(id) => match group_set.iter().position(|o| &o.group_id == *id) {
                Some(pos) => pos + 1,
                None => 0,
            },
        };

        group_set.insert(
            new_group_position,
            InsertGroupSetItem {
                user_id: user.id.clone(),
                group_id: group.id.clone(),
                group_name: (&group.name).clone(),
                set_name: set_name.to_string(),
                order: new_group_position as i32,
            },
        );

        recount_ordering(&mut group_set);

        self.group_sets_repo
            .remove_by_set_name_and_user_id(set_name, &user.id)
            .await;
        self.group_sets_repo.insert(group_set.refs()).await;

        new_group_position
    }

    async fn insert_group_into_default_set(
        &self,
        user: &User,
        group: &Group,
        after_group: &Option<&Id>,
    ) -> usize {
        let mut default_set: Vec<InsertDefaultGroupSetItem> = self
            .default_group_sets_repo
            .get_by_user_id(&user.id)
            .await
            .iter()
            .enumerate()
            .map(|(i, group)| InsertDefaultGroupSetItem {
                user_id: group.user_id.clone(),
                group_id: group.group_id.clone(),
                group_name: group.group_name.clone(),
                order: i as i32,
            })
            .collect();

        let new_group_position = match after_group {
            None => 0,
            Some(id) => match default_set.iter().position(|o| &o.group_id == *id) {
                Some(pos) => pos + 1,
                None => 0,
            },
        };

        default_set.insert(
            new_group_position,
            InsertDefaultGroupSetItem {
                user_id: user.id.clone(),
                group_id: group.id.clone(),
                group_name: (&group.name).clone(),
                order: new_group_position as i32,
            },
        );

        recount_default_ordering(&mut default_set);

        self.default_group_sets_repo
            .remove_by_user_id(&user.id)
            .await;
        self.default_group_sets_repo
            .insert(default_set.refs())
            .await;

        new_group_position
    }
}

#[async_trait]
impl GroupsServiceIf for GroupsService {
    async fn create_group(
        &self,
        user: &User,
        name: &str,
        into_set: Option<&str>,
        after_group: Option<&Id>,
    ) -> AppResult<UserGroup> {
        if let Some(set_name) = into_set {
            if let Some(_) = self
                .group_sets_repo
                .find_by_user_id_set_name_and_group_name(&user.id, set_name, name)
                .await
            {
                return Err(AppError::validation(&format!(
                    "Group `{}` already exists",
                    name
                )));
            }
        }

        let group_entity = self
            .groups_repo
            .insert(InsertGroup {
                creator_id: (&user.id).clone(),
                name: name.to_string(),
                removed: false,
            })
            .await;

        let mut inserted_group_position = self
            .insert_group_into_default_set(&user, &group_entity, &after_group)
            .await;

        if let Some(set_name) = into_set {
            inserted_group_position = self
                .insert_group_into_set(&user, &group_entity, set_name, &after_group)
                .await;
        }

        Ok(UserGroup {
            id: group_entity.id,
            name: group_entity.name,
            order: inserted_group_position as i32,
        })
    }

    async fn remove(&self, _user: &User, _id: &Id) -> AppResult<RemovedGroup> {
        unimplemented!()

        // let mut ordering = self.groups_ordering_repo.get_by_user_id(&user.id).await;
        //
        // let remove_group_position =
        //     ordering
        //         .iter()
        //         .position(|g| &g.group_id == id)
        //         .ok_or(AppError::validation(&format!(
        //             "Group `{}` you are trying to remove not exists",
        //             id
        //         )))?;
        //
        // let removed_order = ordering.remove(remove_group_position);
        // self.groups_repo.mark_removed(&removed_order.group_id).await;
        //
        // let mut new_ordering: Vec<InsertGroupOrder> = ordering
        //     .iter()
        //     .map(|o| InsertGroupOrder {
        //         user_id: o.user_id.clone(),
        //         group_id: o.group_id.clone(),
        //         order: o.order,
        //     })
        //     .collect();
        //
        // recount_default_ordering(&mut new_ordering);
        // self.groups_ordering_repo.delete_by_user_id(&user.id).await;
        // self.groups_ordering_repo.insert(new_ordering).await;
        //
        // let group_entity = self
        //     .groups_repo
        //     .find(&removed_order.group_id)
        //     .await
        //     .unwrap();
        //
        // Ok(RemovedGroup {
        //     id: removed_order.group_id,
        //     name: group_entity.name,
        // })
    }

    async fn list(
        &self,
        user: &User,
        set_name: Option<&str>,
        paging: Option<Paging>,
    ) -> AppResult<Paged<UserGroup>> {
        let paging: crate::services::Paging = paging.into();

        if paging.limit > PAGING_MAX_LIMIT {
            return Err(AppError::validation(&format!(
                "Paging limit can not be more then {}",
                PAGING_MAX_LIMIT
            )));
        }

        let mut res: Vec<UserGroup> = vec![];

        let nn: Vec<UserGroup> = match set_name {
            _ => self
                .default_group_sets_repo
                .get_paged_by_user_id(&user.id, paging.offset, paging.limit)
                .await
                .into_iter()
                .map(|x| UserGroup {
                    id: x.id,
                    name: x.group_name,
                    order: x.order,
                })
                .collect(),
        };

        Ok(Paged {
            objects: nn,
            page_info: PageInfo {
                offset: paging.offset,
                limit: paging.limit,
                total: None,
            },
        })
    }
}
