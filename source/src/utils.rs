use slog::Logger;

use crate::errors::AppError;

pub type AppResult<T> = Result<T, AppError>;

///
/// Трэйт чтобы логировать ошибки
///
pub trait LogOnErr<T, E> {
    fn log_on_err(self, logger: &Logger) -> Result<T, E>;
}

impl<T, E> LogOnErr<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn log_on_err(self, logger: &Logger) -> Result<T, E> {
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
