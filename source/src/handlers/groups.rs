use crate::repos::Id;
use crate::services::stack::{
    Block as ServiceBlock, Mark as ServiceMark, StackItem as ServiceStackItem,
};
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, GraphQLObject)]
pub struct UserGroup {
    pub id: Id,
    pub name: String,
    pub order: i32,
}
