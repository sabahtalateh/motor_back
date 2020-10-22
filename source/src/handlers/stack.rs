use crate::repos::Id;
use crate::services::stack::{
    Block as ServiceBlock, Mark as ServiceMark, StackItem as ServiceStackItem,
};
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, GraphQLObject)]
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

#[derive(Debug, Clone, PartialEq, Eq, GraphQLObject)]
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

#[derive(Debug, Clone, PartialEq, Eq, GraphQLObject)]
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

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewStackItem {
    pub blocks: Vec<NewBlock>,
}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct NewBlock {
    pub order: i32,
    pub text: String,
    pub marks: Vec<NewMark>,
}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct NewMark {
    pub from: i32,
    pub to: i32,
}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct UpdateStackItem {
    pub id: Id,
    pub blocks: Vec<UpdateBlock>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, GraphQLInputObject)]
pub struct UpdateBlock {
    pub id: Option<Id>,
    pub text: String,
    pub marks: Vec<UpdateMark>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, GraphQLInputObject)]
pub struct UpdateMark {
    pub id: Option<Id>,
    pub from: i32,
    pub to: i32,
}
