use juniper::{graphql_value, FieldError, IntoFieldError};

use AppErrorType::*;

#[derive(Debug, Clone)]
pub enum AppErrorType {
    NotFound,
    Unauthorized,
    InternalServerError,
    CheckError,
}

impl ToString for AppErrorType {
    fn to_string(&self) -> String {
        match self {
            NotFound => "not_found",
            Unauthorized => "unauthorized",
            InternalServerError => "internal_server_error",
            CheckError => "check_error",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct AppError {
    message: String,
    error_type: AppErrorType,
}

#[derive(Debug, Clone)]
struct ApiError {
    message: String,
    error_type: String,
}

impl AppError {
    fn new(message: &str, error_type: AppErrorType) -> AppError {
        AppError {
            message: message.to_string(),
            error_type,
        }
    }

    pub fn unauthorized_default() -> AppError {
        AppError::new("unauthorized", AppErrorType::Unauthorized)
    }

    pub fn unauthorized(message: &str) -> AppError {
        AppError::new(message, AppErrorType::Unauthorized)
    }

    pub fn not_found(message: &str) -> AppError {
        AppError::new(message, AppErrorType::NotFound)
    }

    pub fn check(message: &str) -> AppError {
        AppError::new(message, AppErrorType::CheckError)
    }

    pub fn internal_server_error() -> AppError {
        AppError::new("internal server error", AppErrorType::InternalServerError)
    }
}

// Ето будет написано в ответе графкуэля
impl IntoFieldError for AppError {
    fn into_field_error(self) -> FieldError {
        FieldError::new(
            self.message,
            graphql_value!({ "type": (self.error_type.to_string()) }),
        )
    }
}
