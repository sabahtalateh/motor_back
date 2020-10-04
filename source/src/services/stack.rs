use crate::handlers::NewStackItem;
use crate::logger::AppLoggerIf;
use crate::repos::blocks::{BlocksRepoIf, NewBlock};
use crate::repos::marks::{MarksRepoIf, NewMark};
use crate::repos::stack::{NewStackItem as NewStackItemEntity, StackItem, StackRepoIf};
use crate::repos::users::User;
use async_trait::async_trait;
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait StackServiceIf: Interface {
    async fn add_to_my_stack(&self, user: User, stack_item: NewStackItem) -> StackItem;
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

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl StackServiceIf for StackService {
    async fn add_to_my_stack(&self, user: User, new_stack_item: NewStackItem) -> StackItem {
        let stack_item_entity = self
            .stack_repo
            .insert(&NewStackItemEntity {
                user_id: user.id,
                title: new_stack_item.title,
                block_ids: vec![],
                mark_ids: vec![],
            })
            .await;

        let mut block_ids = vec![];
        let mut marks_ids = vec![];
        for b in new_stack_item.blocks {
            let block = self
                .blocks_repo
                .insert(&NewBlock {
                    stack_id: stack_item_entity.id.clone(),
                    text: b.text.clone(),
                })
                .await;

            block_ids.push(block.id.clone());

            let new_marks = b
                .marks
                .iter()
                .map(|x| NewMark {
                    block_id: block.id.clone(),
                    from: x.from,
                    to: x.to,
                })
                .collect();
            let marks = self.marks_repo.insert_many(&new_marks).await;
            marks.iter().for_each(|m| marks_ids.push(m.id.clone()))
        }

        let stack_item_entity = self
            .stack_repo
            .link_blocks(&stack_item_entity, &block_ids)
            .await;

        let stack_item_entity = self
            .stack_repo
            .link_marks(&stack_item_entity, &marks_ids)
            .await;

        stack_item_entity
    }

    async fn my_stack(&self, user: User) -> Vec<StackItem> {
        self.stack_repo.find_by_user_id(user.id).await
    }
}
