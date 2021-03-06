use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Display;

pub mod blocks;
pub mod db;
pub mod default_group_sets;
pub mod group_sets;
pub mod groups;
pub mod groups_ordering;
pub mod marks;
pub mod recent_sets;
pub mod sets;
pub mod stack;
pub mod stack_history;
pub mod tokens;
pub mod users;

#[async_trait]
pub trait Repo<Select, Insert>
where
    Select: DeserializeOwned,
    Insert: Serialize,
{
    async fn insert(&self, insert: Insert) -> Select;
    async fn insert_many(&self, insert: Vec<&Insert>);
    async fn find(&self, id: &Id) -> Option<Select>;
    async fn find_many(&self, ids: Vec<&Id>) -> Vec<Select>;
    async fn delete(&self, id: &Id);
    async fn delete_many(&self, ids: Vec<&Id>);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub String);

#[Scalar]
impl ScalarType for Id {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(Id::from_str(value))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

impl Id {
    pub fn from_str(str: &str) -> Id {
        Id(str.to_string())
    }
}

impl Id {
    pub fn new(val: String) -> Self {
        Id(val)
    }

    pub fn oid(&self) -> ObjectId {
        self.clone().into()
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
