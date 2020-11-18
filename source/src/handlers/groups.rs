use async_graphql::{Object, SimpleObject};
use serde::Serialize;

use crate::repos::Id;
use crate::services::Paged;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, SimpleObject)]
pub struct UserGroup {
    pub id: Id,
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemovedGroup {
    pub id: Id,
    pub name: String,
}

// #[Object]
// impl Paged<UserGroup> {
//     pub async fn objects(&self) -> &Vec<UserGroup> {
//         &self.objects
//     }
//
//     pub async fn offset(&self) -> i32 {
//         self.offset
//     }
//
//     pub async fn limit(&self) -> i32 {
//         self.limit
//     }
//
//     pub async fn total(&self) -> i32 {
//         (&self.objects).len() as i32
//     }
// }

// #[Object]
// impl Paged<i32> {
//     pub async fn objects(&self) -> &Vec<i32> {
//         &self.objects
//     }
//
//     pub async fn offset(&self) -> i32 {
//         self.offset
//     }
//
//     pub async fn limit(&self) -> i32 {
//         self.limit
//     }
//
//     pub async fn total(&self) -> i32 {
//         (&self.objects).len() as i32
//     }
// }
