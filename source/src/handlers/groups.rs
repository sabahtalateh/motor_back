use async_graphql::{Object, SimpleObject};
use serde::Serialize;

use crate::repos::Id;
use crate::services::Paged;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, SimpleObject)]
pub struct UserSet {
    pub id: Id,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, SimpleObject)]
pub struct UserGroup {
    pub id: Id,
    pub id_in_set: Id,
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemovedGroup {
    pub id: Id,
    pub name: String,
}
