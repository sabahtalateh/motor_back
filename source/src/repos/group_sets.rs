use std::sync::Arc;

use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use slog::Logger;

use proc_macro::HasLogger;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::db::{find_one_by, find_one_by_id, insert_one_into, set_by_id};
use crate::repos::db::find_many_by_ids;
use crate::repos::Id;

pub const COLLECTION: &str = "group_sets";

#[async_trait]
pub trait GroupSetsRepoIf: Interface {
}

#[shaku(interface = GroupSetsRepoIf)]
#[derive(Component, HasLogger)]
pub struct GroupSetsRepo {
    #[shaku(inject)]
    db: Arc<dyn DBIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl GroupSetsRepoIf for GroupSetsRepo {
}
