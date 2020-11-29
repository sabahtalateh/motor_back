use async_graphql::ErrorExtensions;
use bson::Document;
use serde::de::DeserializeOwned;
use serde::Serialize;
use slog::Logger;

use crate::errors::AppError;

pub type AppResult<T> = Result<T, AppError>;

// impl From<AppError> for async_graphql::Error {
//     fn from(_: AppError) -> Self {
//         unimplemented!()
//     }
// }

// impl <T> Into<Result<T, async_graphql::Error>> for AppResult<T> {
//     fn into(self) -> Result<T, Error> {
//         unimplemented!()
//     }
// }

pub fn deserialize_bson<T>(bson: &Document) -> T
where
    T: DeserializeOwned,
{
    bson::from_bson(bson::to_bson(&bson).unwrap()).unwrap()
}

///
/// Трэйт чтобы логировать ошибки в Result
///
pub trait LogErrWith<T, E> {
    fn log_err_with(self, logger: &Logger) -> Result<T, E>;
}

impl<T, E> LogErrWith<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn log_err_with(self, logger: &Logger) -> Result<T, E> {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => {
                slog_error!(logger, "{}", e);
                Err(e)
            }
        }
    }
}

///
/// Трэйт чтобы логировать ошибки в Option
///
pub trait OkOrMongoRecordId<T> {
    fn ok_or_mongo_record_id(self) -> AppResult<T>;
}

impl<T> OkOrMongoRecordId<T> for Option<T> {
    fn ok_or_mongo_record_id(self) -> AppResult<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(AppError::general(
                "Mongo inserted record id is not of `ObjectId` type",
            )),
        }
    }
}

///
/// Переделываем ошибку во внутренню ошибку
///
pub trait IntoAppErr<T> {
    fn into_app_err(self) -> AppResult<T>;
}

impl<T, E> IntoAppErr<T> for Result<T, E> {
    fn into_app_err(self) -> Result<T, AppError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(AppError::internal()),
        }
    }
}

///
/// Пишем тип в графкуэльную ошибку
///
pub trait ExtendType<T> {
    fn extend_type(self) -> async_graphql::Result<T>;
}

impl<T> ExtendType<T> for AppResult<T> {
    fn extend_type(self) -> async_graphql::Result<T> {
        self.map_err(|ee| ee.extend_with(|_, e| e.set("type", ee.get_type())))
    }
}

///
/// Если пользователь не найден
/// Обычно в контроллере первой строчкой
///
pub trait OkOrUnauthorized<T> {
    fn ok_or_unauthorized(self) -> AppResult<T>;
}

impl<T> OkOrUnauthorized<T> for Option<T> {
    fn ok_or_unauthorized(self) -> AppResult<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(AppError::unauthorized()),
        }
    }
}

///
/// Если пользователь не найден
/// Обычно в контроллере первой строчкой
///
pub trait OkOrNotFound<T> {
    fn ok_or_not_found(self) -> AppResult<T>;
}

impl<T> OkOrNotFound<T> for Option<T> {
    fn ok_or_not_found(self) -> AppResult<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(AppError::not_found("Not Found")),
        }
    }
}

///
/// Для переделывания массива каких то штук в массив документов.
/// Штуки должны быть сериализуемые
///
pub trait ToDocsVec<T> {
    fn to_documents_vec(&self) -> Vec<Document>;
}

impl<T> ToDocsVec<T> for Vec<T>
where
    T: Serialize,
{
    fn to_documents_vec(&self) -> Vec<Document> {
        self.iter()
            .map(|x| bson::to_bson(x).unwrap().as_document().unwrap().clone())
            .collect()
    }
}

pub trait Refs<T> {
    fn refs(&self) -> Vec<&T>;
}

impl<T> Refs<T> for Vec<T> {
    fn refs(&self) -> Vec<&T> {
        self.iter().map(|x| x).collect()
    }
}
