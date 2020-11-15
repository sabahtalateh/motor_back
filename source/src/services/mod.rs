use async_graphql::*;

pub mod auth;
pub mod groups;
pub mod stack;

#[derive(InputObject)]
pub struct Paging {
    pub offset: i32,
    pub limit: i32,
}

#[derive(Debug)]
pub struct PagedResponse<T>
{
    pub total: i32,
    pub offset: i32,
    pub limit: i32,
    pub objects: Vec<T>,
}
