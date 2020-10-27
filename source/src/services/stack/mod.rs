pub mod diff;

use crate::errors::AppError;
use crate::handlers::stack::{ChangeMark, NewStackItem, StackItemChangeSet, UpdateBlock};
use crate::logger::AppLoggerIf;
use crate::repos::blocks::BlocksRepoIf;
use crate::repos::blocks::{Block as BlockEntity, InsertBlock};
use crate::repos::db::find_many_by_ids;
use crate::repos::marks::{InsertMark, Mark as MarkEntity, MarksRepoIf};
use crate::repos::stack::{NewStackItem as NewStackItemEntity, StackRepoIf};
use crate::repos::stack_history::{InsertHistoryBlock, InsertHistoryMark, StackHistoryRepoIf};
use crate::repos::users::User;
use crate::repos::Id;
use crate::utils::{AppResult, OkOrNotFound, Refs};
use async_trait::async_trait;
use juniper::{GraphQLInputObject, GraphQLObject};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::sync::Arc;
use crate::repos::groups::GroupsRepoIf;
use crate::repos::groups_ordering::GroupsOrderingRepoIf;

#[derive(Debug, Clone)]
pub struct StackItem {
    pub id: Id,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub id: Id,
    pub stack_id: Id,
    pub order: i32,
    pub text: String,
    pub marks: Vec<Mark>,
    pub current_version: i32,
    pub initial_version: i32,
}

// impl PartialEq<UpdateBlock> for Block {
//     fn eq(&self, other: &UpdateBlock) -> bool {
//         // updated block must have id
//         if other.id.is_none() {
//             return false;
//         }
//
//         if &self.id != other.id.as_ref().unwrap() {
//             return false;
//         }
//
//         if self.text != other.text {
//             return false;
//         }
//
//         // updated marks must have ids and amount of marks equals to Block amount of marks
//         let updated_marks = &other.marks;
//         if updated_marks.len() != self.marks.len() {
//             return false;
//         }
//         let has_updated_mark_without_id = updated_marks.iter().any(|u| u.id.is_none());
//         if has_updated_mark_without_id {
//             return false;
//         }
//
//         // updated marks ids must be same as existent marks ids
//         let mut marks_ids: Vec<Id> = self.marks.iter().map(|m| m.id.clone()).collect();
//         let mut updated_marks_ids: Vec<Id> = other
//             .marks
//             .iter()
//             .map(|m| m.id.as_ref().unwrap().clone())
//             .collect();
//         marks_ids.sort();
//         updated_marks_ids.sort();
//         if marks_ids != updated_marks_ids {
//             return false;
//         }
//
//         for existent_mark in &self.marks {
//             // can not be None, checked previously so can unwrap
//             let updated_mark = updated_marks
//                 .iter()
//                 .find(|u| match &u.id {
//                     None => false,
//                     Some(id) => id == &existent_mark.id,
//                 })
//                 .unwrap();
//
//             if existent_mark != updated_mark {
//                 return false;
//             }
//         }
//
//         return true;
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    pub id: Id,
    pub from: i32,
    pub to: i32,
}

// impl PartialEq<InsertMark> for Mark {
//     fn eq(&self, other: &InsertMark) -> bool {
//         if other.id.is_none() {
//             return false;
//         }
//
//         return &self.id == other.id.as_ref().unwrap()
//             && self.from == other.from
//             && self.to == other.to;
//     }
// }

#[async_trait]
pub trait StackServiceIf: Interface {
    async fn add_to_my_stack(&self, user: User, stack_item: NewStackItem) -> AppResult<StackItem>;
    async fn update_stack_item(
        &self,
        user: User,
        changes: StackItemChangeSet,
    ) -> AppResult<StackItem>;
    async fn my_stack(&self, user: User) -> Vec<StackItem>;
}

#[shaku(interface = StackServiceIf)]
#[derive(Component, HasLogger)]
pub struct StackService {
    #[shaku(inject)]
    groups_repo: Arc<dyn GroupsRepoIf>,

    #[shaku(inject)]
    groups_ordering_repo: Arc<dyn GroupsOrderingRepoIf>,

    #[shaku(inject)]
    stack_repo: Arc<dyn StackRepoIf>,

    #[shaku(inject)]
    blocks_repo: Arc<dyn BlocksRepoIf>,

    #[shaku(inject)]
    marks_repo: Arc<dyn MarksRepoIf>,

    #[shaku(inject)]
    stack_history_repo: Arc<dyn StackHistoryRepoIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

impl StackService {
    async fn find_stack_item_by_user_id_and_stack_item_id(
        &self,
        user_id: &Id,
        stack_item_id: &Id,
    ) -> Option<StackItem> {
        let stack_item_entity = self
            .stack_repo
            .find_by_user_id_and_stack_item_id(user_id.clone(), stack_item_id.clone())
            .await?;

        let blocks_ids: Vec<Id> = stack_item_entity.blocks_ids;
        let marks_ids: Vec<Id> = stack_item_entity.marks_ids;

        let blocks = self.blocks_repo.find_by_ids(blocks_ids.refs()).await;
        let marks = self
            .marks_repo
            .find_by_ids(marks_ids.refs())
            .await;

        let mut stack_item_blocks = vec![];
        for block_id in blocks_ids {
            let block_entity = blocks.iter().find(|b| b.id == block_id).unwrap().clone();

            let mut block_item_marks = vec![];
            for mark_id in block_entity.marks_ids {
                let mark_entity = marks.iter().find(|m| m.id == mark_id).unwrap().clone();
                block_item_marks.push(Mark {
                    id: mark_entity.id,
                    from: mark_entity.from,
                    to: mark_entity.to,
                })
            }

            stack_item_blocks.push(Block {
                id: block_entity.id,
                stack_id: block_entity.stack_id,
                order: block_entity.order,
                text: block_entity.text,
                marks: block_item_marks,
                current_version: block_entity.current_version,
                initial_version: block_entity.initial_version,
            })
        }
        Some(StackItem {
            id: stack_item_entity.id,
            blocks: stack_item_blocks,
        })
    }
}

#[async_trait]
impl StackServiceIf for StackService {
    async fn add_to_my_stack(
        &self,
        user: User,
        mut new_stack_item: NewStackItem,
    ) -> AppResult<StackItem> {
        if new_stack_item.blocks.len() == 0 {
            return Err(AppError::validation("Can not add empty stack item"));
        }

        // TODO это убарть
        // let ids = vec![Id("123".to_string())];
        // let ids2 = vec![Id("456".to_string())];
        //
        // let a = self.marks_repo.find_by_ids(&ids);
        // let b = self.marks_repo.find_by_ids(&ids2);
        // // let b = self.marks_repo.find_by_ids(&vec![Id("456".to_string())]);
        // let c = futures::join!(a, b);
        // TODO вот досюда

        let stack_item_entity = self
            .stack_repo
            .insert(&NewStackItemEntity {
                user_id: user.id,
                blocks_ids: vec![],
                marks_ids: vec![],
                version: 0,
            })
            .await;

        let mut blocks = vec![];
        let mut blocks_ids = vec![];

        let mut marks = vec![];
        let mut marks_ids = vec![];
        for (i, b) in new_stack_item.blocks.into_iter().enumerate() {
            let inserted_block = self
                .blocks_repo
                .insert(InsertBlock {
                    stack_id: stack_item_entity.id.clone(),
                    order: i as i32,
                    text: b.text,
                    marks_ids: vec![],
                    current_version: 0,
                    initial_version: 0,
                })
                .await;

            blocks_ids.push(inserted_block.id.clone());

            let new_marks: Vec<InsertMark> = b
                .marks
                .iter()
                .map(|x| InsertMark {
                    block_id: inserted_block.id.clone(),
                    from: x.from,
                    to: x.to,
                })
                .collect();
            let inserted_marks = self.marks_repo.insert_many(new_marks.refs()).await;
            inserted_marks.iter().for_each(|m| {
                marks.push(m.clone());
                marks_ids.push(m.id.clone())
            });

            self.blocks_repo
                .link_marks(
                    &inserted_block,
                    &inserted_marks.iter().map(|m| m.id.clone()).collect(),
                )
                .await;

            blocks.push(Block {
                id: inserted_block.id,
                stack_id: inserted_block.stack_id,
                order: inserted_block.order,
                text: inserted_block.text,
                marks: inserted_marks
                    .into_iter()
                    .map(|m| Mark {
                        id: m.id,
                        from: m.from,
                        to: m.to,
                    })
                    .collect(),
                current_version: 0,
                initial_version: 0,
            })
        }

        let stack_item_entity = self
            .stack_repo
            .link_blocks(&stack_item_entity, &blocks_ids)
            .await;

        let stack_item_entity = self
            .stack_repo
            .link_marks(&stack_item_entity, &marks_ids)
            .await;

        Ok(StackItem {
            id: stack_item_entity.id,
            blocks,
        })
    }

    async fn update_stack_item(
        &self,
        user: User,
        changes: StackItemChangeSet,
    ) -> AppResult<StackItem> {
        let updated_contains_removed =  changes.updated.iter().any(|u| changes.removed.iter().any(|r| &u.id == r));
        let removed_contains_updated =  changes.removed.iter().any(|r| changes.updated.iter().any(|u| &u.id == r));

        if updated_contains_removed || removed_contains_updated {
            return Err(AppError::validation("updated and removed changes intersects"));
        }

        let old_stack_item = self
            .find_stack_item_by_user_id_and_stack_item_id(&user.id, &changes.stack_id)
            .await
            .ok_or(AppError::not_found("Stack item not found"))?;

        let removed_history_blocks: Vec<InsertHistoryBlock> = old_stack_item
            .blocks
            .iter()
            .filter(|b| changes.removed.contains(&b.id))
            .map(|b| InsertHistoryBlock {
                stack_id: changes.stack_id.clone(),
                block_id: b.id.clone(),
                version: b.current_version,
                text: b.text.clone(),
                marks: b
                    .marks
                    .iter()
                    .map(|m| InsertHistoryMark {
                        mark_id: m.id.clone(),
                        from: m.from,
                        to: m.to,
                    })
                    .collect(),
            })
            .collect();

        let updated_history_blocks: Vec<InsertHistoryBlock> = old_stack_item
            .blocks
            .iter()
            .filter(|b| changes.updated.iter().any(|u| u.id == b.id))
            .map(|b| InsertHistoryBlock {
                stack_id: changes.stack_id.clone(),
                block_id: b.id.clone(),
                version: b.current_version,
                text: b.text.clone(),
                marks: b
                    .marks
                    .iter()
                    .map(|m| InsertHistoryMark {
                        mark_id: m.id.clone(),
                        from: m.from,
                        to: m.to,
                    })
                    .collect(),
            })
            .collect();

        futures::join!(
            self.stack_history_repo.insert_many(removed_history_blocks.refs()),
            self.stack_history_repo.insert_many(updated_history_blocks.refs()),
        );

        // потом дропнуть удалённое
        // инкрементить текущую версию
        // потом обновить с новой версией
        // потом добавить с инициальной = новой

        unimplemented!()

        // rebuilt orders
        //  orders [0, 2, 5] will be [0, 1, 2]
        // changes.blocks.sort_by(|a, b| a.order.cmp(&b.order));
        // for (i, block) in changes.blocks.iter_mut().enumerate() {
        //     block.order = i as i32;
        // }
        //
        // // First create history copy of blocks

        //
        // let deleted_blocks =
        //     diff::get_deleted_blocks(&old_stack_item.blocks, &changes.blocks);
        //
        // let deleted_history_blocks: Vec<InsertHistoryBlock> = deleted_blocks
        //     .iter()
        //     .map(|x| x.clone().clone().into())
        //     .collect();
        //
        // let updated_blocks =
        //     diff::get_updated_blocks(&old_stack_item.blocks, &changes.blocks);
        //
        // let updated_history_blocks: Vec<InsertHistoryBlock> = updated_blocks
        //     .iter()
        //     .map(|(x, _)| x.clone().clone().into())
        //     .collect();
        //
        // futures::join!(
        //     self.stack_history_repo
        //         .insert_many(deleted_history_blocks.refs()),
        //     self.stack_history_repo
        //         .insert_many(updated_history_blocks.refs())
        // );
        //
        // // self.stack_history_repo
        // //     .insert_many(deleted_history_blocks.refs())
        // //     .await;
        // //
        // // self.stack_history_repo
        // //     .insert_many(updated_history_blocks.refs())
        // //     .await;
        //
        // let added_blocks = diff::get_new_blocks(&old_stack_item.blocks, &changes.blocks);
        //
        // // let
        // println!("{:#?}", deleted_blocks);
        //
        // unimplemented!();
        //
        // // return Ok(old_stack_item);
        // // println!("{:#?}", old_stack_item);
        // // unimplemented!();
        //
        // let stack_item = self
        //     .stack_repo
        //     .find_by_user_id_and_stack_item_id(user.id, changes.id)
        //     .await
        //     .ok_or_not_found()?;
        //
        // let updated_blocks = changes.blocks;
        // let old_blocks = self.blocks_repo.find_by_ids(&stack_item.blocks_ids).await;
        //
        // // println!("UU");
        // // println!("{:?}", updated_blocks);
        // // println!("PP");
        // // println!("{:?}", old_blocks);
        //
        // let added_new_blocks: Vec<&UpdateBlock> = updated_blocks
        //     .iter()
        //     .filter(|u| match &u.id {
        //         None => true,
        //         Some(id) => old_blocks.iter().any(|p| p.id != *id),
        //     })
        //     .collect();
        //
        // // let mut marks_appeared_in_added_blocks = HashMap::new();
        // for added in added_new_blocks {
        //     // let inserted = self.blocks_repo.insert(&stack_item.id, &added.text).await;
        //     // self.stack_repo
        //     //     .link_blocks(&stack_item, &vec![inserted.id.clone()])
        //     //     .await;
        //     // marks_appeared_in_added_blocks.insert(inserted.id, &added.marks);
        // }
        //
        // // println!("MARKS_IN_NEW_BLOCKS");
        // // println!("{:?}", marks_appeared_in_added_blocks);
        //
        // let removed_old_blocks: Vec<&BlockEntity> = old_blocks
        //     .iter()
        //     .filter(|prev| {
        //         !updated_blocks.iter().any(|u| match &u.id {
        //             None => false,
        //             Some(id) => id == &prev.id,
        //         })
        //     })
        //     .collect();
        //
        // // println!("REMBLK");
        // // println!("{:?}", removed_old_blocks);
        //
        // // let mut marks_exists_in_removed_blocks = HashMap::new();
        // // for removed in removed_old_blocks {
        // // self.blocks_repo.delete(&removed.id).await;
        // // marks_exists_in_removed_blocks.insert(removed.id.clone(), removed.clone().marks_ids);
        // // }
        //
        // let updated_old_blocks: Vec<&BlockEntity> = old_blocks
        //     .iter()
        //     .filter(|prev| {
        //         updated_blocks.iter().any(|u| match &u.id {
        //             None => false,
        //             Some(id) => id == &prev.id,
        //         })
        //     })
        //     .collect();
        //
        // println!("KK");
        // println!("{:?}", updated_blocks);
        //
        // let mut old_marks_removed_from_updated_blocks = HashMap::new();
        // let mut marks_modified_in_updated_blocks = HashMap::new();
        // let mut new_marks_added_into_updated_blocks = HashMap::new();
        //
        // for old in updated_old_blocks {
        //     if let Some(new) = updated_blocks.iter().find(|u| match &u.id {
        //         None => false,
        //         Some(id) => id == &old.id,
        //     }) {
        //         let old_block_marks = self.marks_repo.find_by_block_id(&old.id).await;
        //         let old_marks_removed_from_updated_block: Vec<Id> = old_block_marks
        //             .clone()
        //             .into_iter()
        //             .filter(|m| {
        //                 !new.marks.iter().any(|new_m| match &new_m.id {
        //                     None => false,
        //                     Some(id) => id == &m.id,
        //                 })
        //             })
        //             .map(|m| m.id.clone())
        //             .collect();
        //         old_marks_removed_from_updated_blocks
        //             .insert(&old.id, old_marks_removed_from_updated_block);
        //
        //         let old_marks_modified_in_updated_block: Vec<(MarkEntity, &InsertMark)> =
        //             old_block_marks
        //                 .clone()
        //                 .into_iter()
        //                 .filter(|m| {
        //                     new.marks.iter().any(|new_m| match &new_m.id {
        //                         None => false,
        //                         Some(id) => {
        //                             id == &m.id && (m.from != new_m.from || m.to != new_m.to)
        //                         }
        //                     })
        //                 })
        //                 .map(|old_mark| {
        //                     let new_mark = new
        //                         .marks
        //                         .iter()
        //                         .find(|new_m| match &new_m.id {
        //                             None => false,
        //                             Some(new_m_id) => new_m_id == &old_mark.id,
        //                         })
        //                         .unwrap();
        //                     (old_mark, new_mark)
        //                 })
        //                 .collect();
        //         marks_modified_in_updated_blocks
        //             .insert(&old.id, old_marks_modified_in_updated_block);
        //
        //         let new_marks_added_into_updated_block: Vec<&InsertMark> =
        //             new.marks.iter().filter(|m| m.id.is_none()).collect();
        //         new_marks_added_into_updated_blocks
        //             .insert(&old.id, new_marks_added_into_updated_block);
        //     };
        // }
        //
        // println!("OMREM");
        // println!("{:#?}", old_marks_removed_from_updated_blocks);
        // println!("MUPD");
        // println!("{:#?}", marks_modified_in_updated_blocks);
        // println!("NM");
        // println!("{:#?}", new_marks_added_into_updated_blocks);
        //
        // // let mut updated_blocks_and_marks: Vec<(&BlockEntity, &Vec<UpdateMark>)> = vec![];
        // // for updated in updated_blocks {
        // //     // updated_blocks_and_marks.push((&updated, &updated.))
        // // }
        //
        // // Сначала добавляем все новые блоки
        // // Потом отмечаем удалённые
        // // Потом записываем обновлённые блоки с сохранением предыдущей версии
        // //
        // // flag - fresh, version, deleted
        // // При удалении блока - history - копия, текущий блок обновляется как fresh, deleted, version + 1
        // // При добавлении блока - fresh, version = 0
        // // При изменении блока - history - копия, текущий блок обновляется как fresh, version + 1
        // //
        // // Что с марками
        // // Марки могут
        // //  не измениться
        // //  остаться в том же блоке но сменить расположение
        // //  переместиться в другой блок
        // //  быть удалёнными
        // //
        //
        // println!("UPD");
        // println!("{:?}", updated_blocks);
        //
        // // for prev_b in prev_blocks {
        // //
        // // }
        //
        // unimplemented!()
    }

    // TODO переписать чтобы выбирались блоки по stack_id с учётом moment = true
    async fn my_stack(&self, user: User) -> Vec<StackItem> {
        let stack_item_entities = self.stack_repo.find_by_user_id(user.id).await;

        let blocks_ids: Vec<Id> = stack_item_entities
            .iter()
            .map(|s| s.blocks_ids.clone())
            .flatten()
            .collect();
        let marks_ids: Vec<Id> = stack_item_entities
            .iter()
            .map(|s| s.marks_ids.clone())
            .flatten()
            .collect();

        let blocks = self.blocks_repo.find_by_ids(blocks_ids.refs()).await;
        let marks = self.marks_repo.find_by_ids(marks_ids.refs()).await;

        let mut stack = vec![];
        for item in stack_item_entities {
            let mut stack_item_blocks = vec![];
            for block_id in item.blocks_ids {
                let block_entity = blocks.iter().find(|b| b.id == block_id).unwrap().clone();

                let mut block_item_marks = vec![];
                for mark_id in block_entity.marks_ids {
                    let mark_entity = marks.iter().find(|m| m.id == mark_id).unwrap().clone();
                    block_item_marks.push(Mark {
                        id: mark_entity.id,
                        from: mark_entity.from,
                        to: mark_entity.to,
                    })
                }

                stack_item_blocks.push(Block {
                    id: block_entity.id,
                    stack_id: block_entity.stack_id,
                    order: block_entity.order,
                    text: block_entity.text,
                    marks: block_item_marks,
                    current_version: block_entity.current_version,
                    initial_version: block_entity.initial_version,
                })
            }
            stack.push(StackItem {
                id: item.id,
                blocks: stack_item_blocks,
            })
        }

        stack
    }
}
