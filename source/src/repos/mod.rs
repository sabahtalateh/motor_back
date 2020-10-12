pub mod blocks;
pub mod marks;
pub mod stack;
pub mod tokens;
pub mod users;
pub mod db;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::utils::OkOrMongoRecordId;
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith};
use bson::oid::ObjectId;
use bson::Document;
use juniper::futures::{FutureExt, StreamExt};
use juniper::sa::_core::fmt::Debug;
use juniper::GraphQLScalarValue;
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use slog::Logger;
use std::fs::read_to_string;

#[derive(Clone, Debug, GraphQLScalarValue, Hash)]
pub struct Id(String);

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Id {}

impl Id {
    pub fn new(val: String) -> Self {
        Id(val)
    }
}

impl From<Id> for ObjectId {
    fn from(id: Id) -> Self {
        bson::oid::ObjectId::with_string(id.0.as_str()).expect("fail to convert `Id` to `ObjectId`")
    }
}

impl From<ObjectId> for Id {
    fn from(oid: ObjectId) -> Self {
        Id::new(oid.to_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Id::new(ObjectId::deserialize(deserializer)?.to_string()))
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let oid: ObjectId = self.clone().into();
        oid.serialize(serializer)
    }
}
