pub mod blocks;
pub mod marks;
pub mod stack;
pub mod tokens;
pub mod users;

use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::utils::OkOrMongoRecordId;
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith};
use bson::oid::ObjectId;
use bson::Document;
use juniper::sa::_core::fmt::Debug;
use juniper::GraphQLScalarValue;
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use slog::Logger;

async fn find_one_by_id<T>(db: &Database, collection: &str, id: &Id, logger: &Logger) -> Option<T>
where
    T: DeserializeOwned,
{
    let id: ObjectId = id.clone().into();

    db.collection(collection)
        .find_one(Some(doc! {"_id": id}), None)
        .await
        .log_err_with(logger)
        .into_app_err()
        .unwrap()
        .map(|u| deserialize_bson(&u))
}

async fn insert_one_into<T>(db: &Database, collection: &str, object: &T, logger: &Logger) -> Id
where
    T: Serialize,
{
    let doc: Document = bson::to_bson(&object)
        .unwrap()
        .as_document()
        .unwrap()
        .clone();

    db.collection(collection)
        .insert_one(doc, None)
        .await
        .map(|ok| ok.inserted_id)
        .log_err_with(logger)
        .unwrap()
        .as_object_id()
        .ok_or_mongo_record_id()
        .log_err_with(logger)
        .unwrap()
        .clone()
        .into()
}

pub async fn link_external_ids(
    db: &Database,
    parent_collection: &str,
    parent_id: &Id,
    foreign_key: &str,
    external_ids: &Vec<Id>,
) {
    let oid: ObjectId = parent_id.clone().into();

    let external_ids: Vec<ObjectId> = external_ids
        .iter()
        .map(|x| x.clone().into())
        .collect::<Vec<ObjectId>>();

    db.collection(parent_collection)
        .update_one(
            doc! {"_id": oid},
            doc! {"$addToSet": {foreign_key: { "$each": external_ids }}},
            None,
        )
        .await
        .unwrap();
}

#[derive(Clone, Debug, GraphQLScalarValue)]
pub struct Id(String);

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
