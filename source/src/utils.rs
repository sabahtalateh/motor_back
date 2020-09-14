use crate::errors::AppError;
use bson::Document;
use serde::de::DeserializeOwned;
use slog::Logger;


pub type AppResult<T> = Result<T, AppError>;

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
            None => Err(AppError::other_error(
                "Mongo inserted record id is not of `ObjectId` type",
            )),
        }
    }
}

///
/// Переделываем ошибку во внутренню ошибку
///
pub trait IntoAppErr<T> {
    fn into_app_err(self) -> Result<T, AppError>;
}

impl<T, E> IntoAppErr<T> for Result<T, E> {
    fn into_app_err(self) -> Result<T, AppError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(AppError::internal_server_error()),
        }
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
