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

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct UpdateBlock {
    pub id: Option<Id>,
    pub text: String,
    pub marks: Vec<UpdateMark>,
}

impl PartialEq for UpdateBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.text == other.text && self.marks == other.marks
    }
}

impl Eq for UpdateBlock {}

#[derive(Serialize, Debug, Clone, GraphQLInputObject)]
pub struct UpdateMark {
    pub id: Option<Id>,
    pub from: i32,
    pub to: i32,
}

impl PartialEq for UpdateMark {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.from == other.from && self.to == other.to
    }
}

impl Eq for UpdateMark {}
