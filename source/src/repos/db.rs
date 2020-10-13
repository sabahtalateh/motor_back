use crate::db::DBIf;
use crate::logger::AppLoggerIf;
use crate::repos::Id;
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
use std::fs::read_to_string;
use std::pin::Pin;
use futures::{FutureExt, StreamExt};

pub(crate) async fn find_one_by_id<T>(
    db: &Database,
    collection: &str,
    id: &Id,
    logger: &Logger,
) -> Option<T>
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

pub(crate) async fn find_many_by_ids<T>(
    db: &Database,
    collection: &str,
    ids: &Vec<Id>,
    logger: &Logger,
) -> Vec<T>
where
    T: DeserializeOwned,
{
    let ids: Vec<ObjectId> = ids.into_iter().map(|x| x.clone().into()).collect();

    db.collection(collection)
        .find(Some(doc! {"_id": {"$in": ids}}), None)
        .await
        .log_err_with(logger)
        .into_app_err()
        .unwrap()
        .map(|x| deserialize_bson(&x.unwrap()))
        .collect()
        .await
}

pub(crate) async fn find_many_by<T>(
    db: &Database,
    collection: &str,
    criteria: Document,
    logger: &Logger,
) -> Vec<T>
    where
        T: DeserializeOwned,
{
    db.collection(collection)
        .find(Some(criteria), None)
        .await
        .log_err_with(logger)
        .into_app_err()
        .unwrap()
        .map(|x| deserialize_bson(&x.unwrap()))
        .collect()
        .await
}

pub(crate) async fn insert_one_into<T>(
    db: &Database,
    collection: &str,
    object: &T,
    logger: &Logger,
) -> Id
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

pub(crate) async fn set_by_id(db: &Database, collection: &str, id: &Id, set: Document) -> bool {
    let id: ObjectId = id.clone().into();

    let update_result = db
        .collection(collection)
        .update_one(doc! {"_id": id}, doc! { "$set": set }, None)
        .await
        .unwrap();

    update_result.modified_count > 0
}

// To try futures::join
pub(crate) async fn set_by_id_pin(db: Pin<&Database>, collection: &str, id: &Id, set: Document) -> bool {
    let id: ObjectId = id.clone().into();

    let update_result = db
        .collection(collection)
        .update_one(doc! {"_id": id}, doc! { "$set": set }, None)
        .await
        .unwrap();

    update_result.modified_count > 0
}

pub(crate) async fn inc_version(db: &Database, collection: &str, id: &Id) -> bool {
    let id: ObjectId = id.clone().into();

    let update_result = db
        .collection(collection)
        .update_many(doc! { "_id": id }, doc! { "$inc": { "version": 1 } }, None)
        .await
        .unwrap();

    update_result.modified_count > 0
}

pub(crate) async fn delete_by_id(db: &Database, collection: &str, id: &Id) -> bool {
    let id: ObjectId = id.clone().into();

    let delete_result = db
        .collection(collection)
        .delete_one(doc! {"_id": id}, None)
        .await
        .unwrap();

    delete_result.deleted_count > 0
}

pub (crate) async fn delete_by(db: &Database, collection: &str, criteria: Document) -> bool {
    let delete_result = db
        .collection(collection)
        .delete_many(criteria, None)
        .await
        .unwrap();

    delete_result.deleted_count > 0
}

pub(crate) async fn link_external_ids(
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