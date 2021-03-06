use async_graphql::SimpleObject;
use serde::Serialize;

use crate::repos::Id;
use crate::services::stack::{
    Block as ServiceBlock, Mark as ServiceMark, StackItem as ServiceStackItem,
};

#[derive(Debug, Clone, SimpleObject)]
pub struct StackItem {
    pub id: Id,
    pub blocks: Vec<Block>,
}

impl From<ServiceStackItem> for StackItem {
    fn from(item: ServiceStackItem) -> Self {
        StackItem {
            id: item.id,
            blocks: item.blocks.into_iter().map(|b| b.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct Block {
    pub id: Id,
    pub text: String,
    pub marks: Vec<Mark>,
}

impl From<ServiceBlock> for Block {
    fn from(block: ServiceBlock) -> Self {
        Block {
            id: block.id,
            text: block.text,
            marks: block.marks.into_iter().map(|m| m.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct Mark {
    pub id: Id,
    pub from: i32,
    pub to: i32,
}

impl From<ServiceMark> for Mark {
    fn from(mark: ServiceMark) -> Self {
        Mark {
            id: mark.id,
            from: mark.from,
            to: mark.to,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewStackItem {
    pub blocks: Vec<NewBlock>,
}

#[derive(Serialize, Debug, Clone)]
pub struct NewBlock {
    pub text: String,
    pub marks: Vec<NewMark>,
}

#[derive(Serialize, Debug, Clone)]
pub struct NewMark {
    pub from: i32,
    pub to: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct StackItemChangeSet {
    pub stack_id: Id,
    pub inserted: Option<InsertChangeSet>,
    pub removed: Vec<Id>,
    pub updated: Vec<UpdateBlock>,
}

#[derive(Serialize, Debug, Clone)]
pub struct InsertChangeSet {
    pub insert_after_id: Option<Id>,
    pub blocks: Vec<InsertBlock>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct InsertBlock {
    pub text: String,
    pub marks: Vec<ChangeMark>,
}

#[derive(Serialize, Debug, Clone)]
pub struct UpdateBlock {
    pub id: Id,
    pub text: String,
    pub marks: Vec<ChangeMark>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ChangeMark {
    pub id: Option<Id>,
    pub from: i32,
    pub to: i32,
}
