use bson::Document;
use bson::oid::ObjectId;
use futures::StreamExt;
use mongodb::Database;
use mongodb::options::FindOptions;
use serde::de::DeserializeOwned;
use serde::Serialize;
use slog::Logger;

use crate::repos::Id;
use crate::utils::{deserialize_bson, IntoAppErr, LogErrWith};
use crate::utils::OkOrMongoRecordId;

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
        .map(|x| deserialize_bson(&x))
}

pub(crate) async fn find_one_by<T>(
    db: &Database,
    collection: &str,
    criteria: Document,
    logger: &Logger,
) -> Option<T>
where
    T: DeserializeOwned,
{
    db.collection(collection)
        .find_one(Some(criteria), None)
        .await
        .log_err_with(logger)
        .into_app_err()
        .unwrap()
        .map(|x| deserialize_bson(&x))
}

pub(crate) async fn find_many_by_ids<T>(
    db: &Database,
    collection: &str,
    ids: Vec<&Id>,
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
    find_options: Option<FindOptions>,
) -> Vec<T>
where
    T: DeserializeOwned,
{
    db.collection(collection)
        .find(Some(criteria), find_options)
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

pub async fn insert_many_into<T>(
    db: &Database,
    collection: &str,
    many: Vec<&T>,
    logger: &Logger,
) -> Vec<Id>
where
    T: Serialize,
{
    if many.is_empty() {
        return vec![];
    }

    let docs_vec: Vec<Document> = many
        .iter()
        .map(|x| bson::to_bson(x).unwrap().as_document().unwrap().clone())
        .collect();

    let insert_many_result = db
        .collection(collection)
        .insert_many(docs_vec, None)
        .await
        .log_err_with(logger)
        .unwrap();

    insert_many_result
        .inserted_ids
        .iter()
        .map(|x| {
            // as inserted id is always bson::Bson::ObjectId, second branch unreachable
            match x.1 {
                bson::Bson::ObjectId(oid) => oid,
                _ => unreachable!(),
            }
            .clone()
            .into()
        })
        .collect()
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

pub(crate) async fn delete_by_id(db: &Database, collection: &str, id: &Id) -> bool {
    let id: ObjectId = id.clone().into();

    let delete_result = db
        .collection(collection)
        .delete_one(doc! {"_id": id}, None)
        .await
        .unwrap();

    delete_result.deleted_count > 0
}

pub(crate) async fn delete_by(db: &Database, collection: &str, criteria: Document) -> bool {
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
