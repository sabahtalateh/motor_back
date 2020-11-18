use async_graphql::InputObject;
use async_graphql::SimpleObject;

use crate::handlers::Paging as HandlerPaging;
use crate::services::groups::PAGING_MAX_LIMIT;

pub mod auth;
pub mod groups;
pub mod stack;

#[derive(InputObject)]
pub struct Paging {
    pub offset: i32,
    pub limit: i32,
}

impl From<Option<HandlerPaging>> for Paging {
    fn from(p: Option<HandlerPaging>) -> Self {
        let mut offset = 0;
        let mut limit = PAGING_MAX_LIMIT;

        if let Some(paging) = p {
            if let Some(o) = paging.offset {
                offset = o
            }
            if let Some(l) = paging.limit {
                limit = l
            }
        }

        return Paging { offset, limit };
    }
}

#[derive(Debug, SimpleObject)]
pub struct PageInfo {
    pub offset: i32,
    pub limit: i32,
    pub total: Option<i32>,
}

#[derive(Debug)]
pub struct Paged<T> {
    pub objects: Vec<T>,
    pub page_info: PageInfo,
}
