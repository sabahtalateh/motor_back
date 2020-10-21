pub mod diff;

use crate::errors::AppError;
use crate::handlers::stack::{AddStackItem, UpdateBlock, UpdateMark, UpdateStackItem};
use crate::logger::AppLoggerIf;
use crate::repos::blocks::Block as BlockEntity;
use crate::repos::blocks::BlocksRepoIf;
use crate::repos::marks::{InsertMark, Mark as MarkEntity, MarksRepoIf};
use crate::repos::stack::{NewStackItem as NewStackItemEntity, StackRepoIf};
use crate::repos::stack_history::StackHistoryRepoIf;
use crate::repos::users::User;
use crate::repos::Id;
use crate::utils::{AppResult, OkOrNotFound};
use async_trait::async_trait;
use juniper::{GraphQLInputObject, GraphQLObject};
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::collections::HashMap;
use std::iter::Map;
use std::sync::Arc;

#[derive(Debug, Clone, GraphQLObject)]
pub struct StackItem {
    pub id: Id,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, GraphQLObject)]
pub struct Block {
    pub id: Id,
    pub text: String,
    pub marks: Vec<Mark>,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.text == other.text && self.marks == other.marks
    }
}

impl Eq for Block {}

#[derive(Debug, Clone, GraphQLObject)]
pub struct Mark {
    pub id: Id,
    pub from: i32,
    pub to: i32,
}

impl PartialEq for Mark {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.from == other.from && self.to == other.to
    }
}

impl Eq for Mark {}

#[async_trait]
pub trait StackServiceIf: Interface {
    async fn add_to_my_stack(&self, user: User, stack_item: AddStackItem) -> AppResult<StackItem>;
    async fn update_stack_item(
        &self,
        user: User,
        stack_item: UpdateStackItem,
    ) -> AppResult<StackItem>;
    async fn my_stack(&self, user: User) -> Vec<StackItem>;
}

#[shaku(interface = StackServiceIf)]
#[derive(Component, HasLogger)]
pub struct StackService {
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

        let blocks = self.blocks_repo.find_by_ids(&blocks_ids).await;
        let marks = self.marks_repo.find_by_ids(&marks_ids).await;

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
                text: block_entity.text,
                marks: block_item_marks,
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
        new_stack_item: AddStackItem,
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
        for b in new_stack_item.blocks {
            let inserted_block = self
                .blocks_repo
                .insert(&stack_item_entity.id, &b.text)
                .await;

            blocks_ids.push(inserted_block.id.clone());

            let new_marks = b
                .marks
                .iter()
                .map(|x| InsertMark {
                    block_id: inserted_block.id.clone(),
                    from: x.from,
                    to: x.to,
                })
                .collect();
            let inserted_marks = self.marks_repo.insert_many(&new_marks).await;
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
                text: inserted_block.text,
                marks: inserted_marks
                    .into_iter()
                    .map(|m| Mark {
                        id: m.id,
                        from: m.from,
                        to: m.to,
                    })
                    .collect(),
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
        updated_stack_item: UpdateStackItem,
    ) -> AppResult<StackItem> {
        if updated_stack_item.blocks.len() == 0 {
            return Err(AppError::validation("Can not update to empty stack item"));
        }

        // First create history copy of blocks
        let old_stack_item = self
            .find_stack_item_by_user_id_and_stack_item_id(&user.id, &updated_stack_item.id)
            .await
            .ok_or(AppError::not_found("Stack item not found"))?;

        let new_blocks = diff::get_new_blocks(&old_stack_item.blocks, &updated_stack_item.blocks);
        println!("{:#?}", new_blocks);

        unimplemented!();

        // return Ok(old_stack_item);
        // println!("{:#?}", old_stack_item);
        // unimplemented!();

        let stack_item = self
            .stack_repo
            .find_by_user_id_and_stack_item_id(user.id, updated_stack_item.id)
            .await
            .ok_or_not_found()?;

        let updated_blocks = updated_stack_item.blocks;
        let old_blocks = self.blocks_repo.find_by_ids(&stack_item.blocks_ids).await;

        // println!("UU");
        // println!("{:?}", updated_blocks);
        // println!("PP");
        // println!("{:?}", old_blocks);

        let added_new_blocks: Vec<&UpdateBlock> = updated_blocks
            .iter()
            .filter(|u| match &u.id {
                None => true,
                Some(id) => old_blocks.iter().any(|p| p.id != *id),
            })
            .collect();

        // let mut marks_appeared_in_added_blocks = HashMap::new();
        for added in added_new_blocks {
            // let inserted = self.blocks_repo.insert(&stack_item.id, &added.text).await;
            // self.stack_repo
            //     .link_blocks(&stack_item, &vec![inserted.id.clone()])
            //     .await;
            // marks_appeared_in_added_blocks.insert(inserted.id, &added.marks);
        }

        // println!("MARKS_IN_NEW_BLOCKS");
        // println!("{:?}", marks_appeared_in_added_blocks);

        let removed_old_blocks: Vec<&BlockEntity> = old_blocks
            .iter()
            .filter(|prev| {
                !updated_blocks.iter().any(|u| match &u.id {
                    None => false,
                    Some(id) => id == &prev.id,
                })
            })
            .collect();

        // println!("REMBLK");
        // println!("{:?}", removed_old_blocks);

        // let mut marks_exists_in_removed_blocks = HashMap::new();
        // for removed in removed_old_blocks {
        // self.blocks_repo.delete(&removed.id).await;
        // marks_exists_in_removed_blocks.insert(removed.id.clone(), removed.clone().marks_ids);
        // }

        let updated_old_blocks: Vec<&BlockEntity> = old_blocks
            .iter()
            .filter(|prev| {
                updated_blocks.iter().any(|u| match &u.id {
                    None => false,
                    Some(id) => id == &prev.id,
                })
            })
            .collect();

        println!("KK");
        println!("{:?}", updated_blocks);

        let mut old_marks_removed_from_updated_blocks = HashMap::new();
        let mut marks_modified_in_updated_blocks = HashMap::new();
        let mut new_marks_added_into_updated_blocks = HashMap::new();

        for old in updated_old_blocks {
            if let Some(new) = updated_blocks.iter().find(|u| match &u.id {
                None => false,
                Some(id) => id == &old.id,
            }) {
                let old_block_marks = self.marks_repo.find_by_block_id(&old.id).await;
                let old_marks_removed_from_updated_block: Vec<Id> = old_block_marks
                    .clone()
                    .into_iter()
                    .filter(|m| {
                        !new.marks.iter().any(|new_m| match &new_m.id {
                            None => false,
                            Some(id) => id == &m.id,
                        })
                    })
                    .map(|m| m.id.clone())
                    .collect();
                old_marks_removed_from_updated_blocks
                    .insert(&old.id, old_marks_removed_from_updated_block);

                let old_marks_modified_in_updated_block: Vec<(MarkEntity, &UpdateMark)> =
                    old_block_marks
                        .clone()
                        .into_iter()
                        .filter(|m| {
                            new.marks.iter().any(|new_m| match &new_m.id {
                                None => false,
                                Some(id) => {
                                    id == &m.id && (m.from != new_m.from || m.to != new_m.to)
                                }
                            })
                        })
                        .map(|old_mark| {
                            let new_mark = new
                                .marks
                                .iter()
                                .find(|new_m| match &new_m.id {
                                    None => false,
                                    Some(new_m_id) => new_m_id == &old_mark.id,
                                })
                                .unwrap();
                            (old_mark, new_mark)
                        })
                        .collect();
                marks_modified_in_updated_blocks
                    .insert(&old.id, old_marks_modified_in_updated_block);

                let new_marks_added_into_updated_block: Vec<&UpdateMark> =
                    new.marks.iter().filter(|m| m.id.is_none()).collect();
                new_marks_added_into_updated_blocks
                    .insert(&old.id, new_marks_added_into_updated_block);
            };
        }

        println!("OMREM");
        println!("{:#?}", old_marks_removed_from_updated_blocks);
        println!("MUPD");
        println!("{:#?}", marks_modified_in_updated_blocks);
        println!("NM");
        println!("{:#?}", new_marks_added_into_updated_blocks);

        // let mut updated_blocks_and_marks: Vec<(&BlockEntity, &Vec<UpdateMark>)> = vec![];
        // for updated in updated_blocks {
        //     // updated_blocks_and_marks.push((&updated, &updated.))
        // }

        // Сначала добавляем все новые блоки
        // Потом отмечаем удалённые
        // Потом записываем обновлённые блоки с сохранением предыдущей версии
        //
        // flag - fresh, version, deleted
        // При удалении блока - history - копия, текущий блок обновляется как fresh, deleted, version + 1
        // При добавлении блока - fresh, version = 0
        // При изменении блока - history - копия, текущий блок обновляется как fresh, version + 1
        //
        // Что с марками
        // Марки могут
        //  не измениться
        //  остаться в том же блоке но сменить расположение
        //  переместиться в другой блок
        //  быть удалёнными
        //

        println!("UPD");
        println!("{:?}", updated_blocks);

        // for prev_b in prev_blocks {
        //
        // }

        unimplemented!()
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

        let blocks = self.blocks_repo.find_by_ids(&blocks_ids).await;
        let marks = self.marks_repo.find_by_ids(&marks_ids).await;

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
                    text: block_entity.text,
                    marks: block_item_marks,
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
