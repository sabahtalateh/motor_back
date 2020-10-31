pub mod auth;
pub mod groups;
pub mod stack;

use juniper::{EmptySubscription, GraphQLInputObject, GraphQLObject};
use serde::Serialize;
use crate::handlers::groups::UserGroup;

#[derive(GraphQLInputObject)]
pub struct Paging {
    pub offset: i32,
    pub limit: i32,
}

pub struct PagedResponse<T>
{
    pub objects: Vec<T>,
    pub offset: i32,
    pub limit: i32,
}
