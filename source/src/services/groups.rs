use crate::errors::AppError;
use crate::handlers::groups::{RemovedGroup, UserGroup, UserSet};
use crate::handlers::Paging;
use crate::logger::AppLoggerIf;
use crate::repos::default_group_sets::{DefaultGroupSetsRepoIf, InsertDefaultGroupSetItem};
use crate::repos::group_sets::{GroupSetItem, GroupSetsRepoIf, InsertGroupSetItem};
use crate::repos::groups::{Group, GroupsRepoIf, InsertGroup};
use crate::repos::groups_ordering::GroupsOrderingRepoIf;
use crate::repos::recent_sets::{InsertRecentSet, RecentSet, RecentSetsRepoIf};
use crate::repos::sets::{InsertSet, SetsRepoIf};
use crate::repos::users::User;
use crate::repos::Id;
use crate::services::groups::IntoSet::{Default, Named};
use crate::services::{PageInfo, Paged};
use crate::utils::{AppResult, Refs};
use async_graphql::SimpleObject;
use async_trait::async_trait;
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

pub const PAGING_MAX_LIMIT: i32 = 1000;

pub enum IntoSet {
    Default,
    Named(String),
}

impl From<Option<String>> for IntoSet {
    fn from(o: Option<String>) -> Self {
        match o {
            Some(x) => Named(x),
            None => Default,
        }
    }
}

#[derive(SimpleObject)]
pub struct Set {
    pub id: Id,
    pub name: String,
}

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
    async fn create_set(&self, user: User, name: String) -> AppResult<UserSet>;

    async fn recent_sets(&self, user: User) -> Vec<UserSet>;

    // async fn create_group(
    //     &self,
    //     user: User,
    //     name: String,
    //     set: IntoSet,
    //     after_group: Option<Id>,
    // ) -> AppResult<UserGroup>;
    //
    // async fn remove_group(&self, user: User, id: Id) -> AppResult<RemovedGroup>;
    //
    // async fn list_groups(
    //     &self,
    //     user: User,
    //     set: IntoSet,
    //     paging: Option<Paging>,
    // ) -> AppResult<Paged<UserGroup>>;
}

#[derive(Component, HasLogger)]
#[shaku(interface = GroupsServiceIf)]
pub struct GroupsService {
    #[shaku(inject)]
    sets_repo: Arc<dyn SetsRepoIf>,

    #[shaku(inject)]
    recent_sets_repo: Arc<dyn RecentSetsRepoIf>,

    #[shaku(inject)]
    group_sets_repo: Arc<dyn GroupSetsRepoIf>,

    #[shaku(inject)]
    default_group_sets_repo: Arc<dyn DefaultGroupSetsRepoIf>,

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
    async fn create_set(&self, user: User, name: String) -> AppResult<UserSet> {
        if let Some(_) = self
            .sets_repo
            .find_one_by_creator_id_and_name(&user.id, &name)
            .await
        {
            return Err(AppError::validation("set with same name exists"));
        }

        let set: crate::repos::sets::Set = self
            .sets_repo
            .insert(InsertSet {
                creator_id: user.id.clone(),
                name,
            })
            .await;

        self.update_recents(
            &user,
            &Set {
                id: set.id.clone(),
                name: set.name.clone(),
            },
        )
        .await;

        Ok(UserSet {
            id: set.id,
            name: set.name,
        })
    }

    async fn recent_sets(&self, user: User) -> Vec<UserSet> {
        let recents_ids: Vec<Id> = self
            .recent_sets_repo
            .find_by_user_id(&user.id)
            .await
            .into_iter()
            .map(|x| x.set_id)
            .collect();

        self.sets_repo
            .find_many(recents_ids.refs())
            .await
            .into_iter()
            .map(|s| UserSet {
                id: s.id,
                name: s.name,
            })
            .collect()
    }

    // async fn create_group(
    //     &self,
    //     user: User,
    //     name: String,
    //     set: IntoSet,
    //     after_group: Option<Id>,
    // ) -> AppResult<UserGroup> {
    //     if let Named(set_name) = &set {
    //         if let Some(_) = self
    //             .group_sets_repo
    //             .find_by_user_id_set_name_and_group_name(&user.id, &set_name, &name)
    //             .await
    //         {
    //             return Err(AppError::validation(&format!(
    //                 "Group `{}` already exists",
    //                 name
    //             )));
    //         }
    //     }
    //
    //     let group_entity = self
    //         .groups_repo
    //         .insert(InsertGroup {
    //             creator_id: (&user.id).clone(),
    //             name: name.to_string(),
    //             removed: false,
    //         })
    //         .await;
    //
    //     let mut inserted_group_position = self
    //         .insert_group_into_default_set(&user, &group_entity, &after_group)
    //         .await;
    //
    //     let default_set_item = self
    //         .default_group_sets_repo
    //         .find_by_group_id(&group_entity.id)
    //         .await
    //         .unwrap();
    //
    //     let mut set_item: Option<GroupSetItem> = None;
    //     if let Named(name) = &set {
    //         inserted_group_position = self
    //             .insert_group_into_set(&user, &group_entity, name, &after_group)
    //             .await;
    //
    //         set_item = self
    //             .group_sets_repo
    //             .find_by_group_id(&group_entity.id)
    //             .await
    //     }
    //
    //     match set_item {
    //         Some(set_item) => Ok(UserGroup {
    //             id: set_item.group_id,
    //             id_in_set: set_item.id,
    //             name: set_item.group_name,
    //             order: set_item.order,
    //         }),
    //         None => Ok(UserGroup {
    //             id: default_set_item.group_id,
    //             id_in_set: default_set_item.id,
    //             name: default_set_item.group_name,
    //             order: default_set_item.order,
    //         }),
    //     }
    // }

    // async fn remove_group(&self, _user: User, _id: Id) -> AppResult<RemovedGroup> {
    //     unimplemented!()

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
    // }

    // async fn list_groups(
    //     &self,
    //     user: User,
    //     set: IntoSet,
    //     paging: Option<Paging>,
    // ) -> AppResult<Paged<UserGroup>> {
    //     let paging: crate::services::Paging = paging.into();
    //
    //     if paging.limit > PAGING_MAX_LIMIT {
    //         return Err(AppError::validation(&format!(
    //             "Paging limit can not be more then {}",
    //             PAGING_MAX_LIMIT
    //         )));
    //     }
    //
    //     let user_groups: Vec<UserGroup> = match &set {
    //         Named(name) => self
    //             .group_sets_repo
    //             .get_paged_by_user_id_and_set_name(&user.id, name, paging.offset, paging.limit)
    //             .await
    //             .into_iter()
    //             .map(|x| UserGroup {
    //                 id: x.group_id,
    //                 id_in_set: x.id,
    //                 name: x.group_name,
    //                 order: x.order,
    //             })
    //             .collect(),
    //
    //         Default => self
    //             .default_group_sets_repo
    //             .get_paged_by_user_id(&user.id, paging.offset, paging.limit)
    //             .await
    //             .into_iter()
    //             .map(|x| UserGroup {
    //                 id: x.group_id,
    //                 id_in_set: x.id,
    //                 name: x.group_name,
    //                 order: x.order,
    //             })
    //             .collect(),
    //     };
    //
    //     Ok(Paged {
    //         objects: user_groups,
    //         page_info: PageInfo {
    //             offset: paging.offset,
    //             limit: paging.limit,
    //             total: None,
    //         },
    //     })
    // }
}

impl GroupsService {
    async fn update_recents(&self, user: &User, set: &Set) {
        let mut recents = self.recent_sets_repo.find_by_user_id(&user.id).await;
        self.recent_sets_repo
            .delete_many(recents.iter().map(|r| &r.id).collect())
            .await;

        match recents
            .iter()
            .position(|recent_set| recent_set.id == set.id)
        {
            Some(position) => {
                recents.remove(position);
            }
            None => (),
        };

        let mut recents_to_insert: Vec<InsertRecentSet> = recents
            .into_iter()
            .map(|r| InsertRecentSet {
                user_id: r.user_id,
                set_id: r.set_id,
            })
            .collect();

        recents_to_insert.insert(
            0,
            InsertRecentSet {
                user_id: user.id.clone(),
                set_id: set.id.clone(),
            },
        );

        self.recent_sets_repo
            .insert_many(recents_to_insert.refs())
            .await;
    }

    // async fn insert_group_into_set(
    //     &self,
    //     user: &User,
    //     group: &Group,
    //     set_name: &str,
    //     after_group: &Option<Id>,
    // ) {
    //     let mut group_set: Vec<InsertGroupSetItem> = self
    //         .group_sets_repo
    //         .get_by_user_id_and_set_name(&user.id, set_name)
    //         .await
    //         .iter()
    //         .enumerate()
    //         .map(|(i, group)| InsertGroupSetItem {
    //             user_id: group.user_id.clone(),
    //             group_id: group.group_id.clone(),
    //             group_name: group.group_name.clone(),
    //             set_name: set_name.to_string(),
    //             order: i as i32,
    //         })
    //         .collect();
    //
    //     let new_group_position = match after_group {
    //         None => 0,
    //         Some(id) => match group_set.iter().position(|o| o.group_id == *id) {
    //             Some(pos) => pos + 1,
    //             None => 0,
    //         },
    //     };
    //
    //     group_set.insert(
    //         new_group_position,
    //         InsertGroupSetItem {
    //             user_id: user.id.clone(),
    //             group_id: group.id.clone(),
    //             group_name: (&group.name).clone(),
    //             set_name: set_name.to_string(),
    //             order: new_group_position as i32,
    //         },
    //     );
    //
    //     recount_ordering(&mut group_set);
    //
    //     self.group_sets_repo
    //         .remove_by_set_name_and_user_id(set_name, &user.id)
    //         .await;
    //     self.group_sets_repo.insert(group_set.refs()).await;
    // }
    //
    // async fn insert_group_into_default_set(
    //     &self,
    //     user: &User,
    //     group: &Group,
    //     after_group: &Option<Id>,
    // ) {
    //     let mut default_set: Vec<InsertDefaultGroupSetItem> = self
    //         .default_group_sets_repo
    //         .find_by_user_id(&user.id)
    //         .await
    //         .iter()
    //         .enumerate()
    //         .map(|(i, group)| InsertDefaultGroupSetItem {
    //             user_id: group.user_id.clone(),
    //             group_id: group.group_id.clone(),
    //             group_name: group.group_name.clone(),
    //             order: i as i32,
    //         })
    //         .collect();
    //
    //     let new_group_position = match after_group {
    //         None => 0,
    //         Some(id) => match default_set.iter().position(|o| o.group_id == *id) {
    //             Some(pos) => pos + 1,
    //             None => 0,
    //         },
    //     };
    //
    //     default_set.insert(
    //         new_group_position,
    //         InsertDefaultGroupSetItem {
    //             user_id: user.id.clone(),
    //             group_id: group.id.clone(),
    //             group_name: (&group.name).clone(),
    //             order: new_group_position as i32,
    //         },
    //     );
    //
    //     recount_default_ordering(&mut default_set);
    //
    //     self.default_group_sets_repo
    //         .remove_by_user_id(&user.id)
    //         .await;
    //     self.default_group_sets_repo
    //         .insert(default_set.refs())
    //         .await;
    // }
}
