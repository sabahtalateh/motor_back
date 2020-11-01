use crate::repos::Id;
use crate::services::stack::{
    Block as ServiceBlock, Mark as ServiceMark, StackItem as ServiceStackItem,
};
use crate::services::PagedResponse;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, GraphQLObject)]
pub struct UserGroup {
    pub id: Id,
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, GraphQLObject)]
pub struct RemovedGroup {
    pub id: Id,
    pub name: String,
}

#[juniper::graphql_object(name = "PagedUserGroups")]
impl PagedResponse<UserGroup> {
    pub fn objects(&self) -> &Vec<UserGroup> {
        &self.objects
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn total(&self) -> i32 {
        (&self.objects).len() as i32
    }
}
