pub mod tokens;
pub mod users;
pub mod stack;

use bson::oid::ObjectId;
use juniper::ID;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Serialize, Debug)]
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

impl From<Id> for ID {
    fn from(id: Id) -> Self {
        ID::new(id.0)
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
