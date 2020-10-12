use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use crate::repos::Id;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewStackItem {
    pub title: Option<String>,
    pub blocks: Vec<NewBlock>,
}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct NewBlock {
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
    pub title: Option<String>,
    pub blocks: Vec<UpdateBlock>
}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct UpdateBlock {
    pub id: Option<Id>,
    pub text: String,
    pub marks: Vec<UpdateMark>,
}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct UpdateMark {
    pub id: Option<Id>,
    pub from: i32,
    pub to: i32,
}
