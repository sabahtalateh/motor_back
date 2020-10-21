use crate::repos::Id;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct AddStackItem {
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
