use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{
    delete_by_id, find_many_by_ids, find_one_by_id, inc_version, insert_one_into,
    link_external_ids, set_by_id,
};
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith, OkOrMongoRecordId, ToDocsVec};

use crate::repos::Id;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{Bson, Document};
use juniper::futures::StreamExt;
use juniper::GraphQLObject;
use proc_macro::HasLogger;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Serialize, Debug)]
struct InsertHistoryBlock {
    stack_id: Id,
    block_id: Id,
    version: i32,
    text: String,
    marks: Vec<InsertHistoryMark>,
}

#[derive(Serialize, Debug)]
struct InsertHistoryMark {
    from: i32,
    to: i32,
}

#[async_trait]
pub trait StackHistoryRepoIf: Interface {}

#[shaku(interface = StackHistoryRepoIf)]
#[derive(Component, HasLogger)]
pub struct StackHistoryRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

impl StackHistoryRepoIf for StackHistoryRepo {}
