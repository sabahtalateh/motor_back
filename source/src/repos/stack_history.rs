use std::sync::Arc;

use async_trait::async_trait;
use serde::Serialize;
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::insert_many_into;
use crate::repos::Id;
use crate::services::stack::{Block, Mark};

pub const COLLECTION: &str = "stack_history";

#[derive(Serialize, Debug)]
pub struct InsertHistoryBlock {
    pub stack_id: Id,
    pub block_id: Id,
    pub version: i32,
    pub text: String,
    pub marks: Vec<InsertHistoryMark>,
}

impl From<Block> for InsertHistoryBlock {
    fn from(block: Block) -> Self {
        InsertHistoryBlock {
            stack_id: block.stack_id,
            block_id: block.id,
            version: block.current_version,
            text: block.text,
            marks: block.marks.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct InsertHistoryMark {
    pub mark_id: Id,
    pub from: i32,
    pub to: i32,
}

impl From<Mark> for InsertHistoryMark {
    fn from(mark: Mark) -> Self {
        InsertHistoryMark {
            mark_id: mark.id,
            from: mark.from,
            to: mark.to,
        }
    }
}

#[async_trait]
pub trait StackHistoryRepoIf: Interface {
    async fn insert_many(&self, blocks: Vec<&InsertHistoryBlock>);
}

#[shaku(interface = StackHistoryRepoIf)]
#[derive(Component, HasLogger)]
pub struct StackHistoryRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl StackHistoryRepoIf for StackHistoryRepo {
    async fn insert_many(&self, blocks: Vec<&InsertHistoryBlock>) {
        insert_many_into(&self.db.get(), COLLECTION, blocks, &self.logger()).await;
    }
}
