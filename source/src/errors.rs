

use juniper::{graphql_value, FieldError, IntoFieldError};

use AppErrorType::*;

#[derive(Debug, Clone)]
pub enum AppErrorType {
    InternalServerError,
    CheckError,
}

impl ToString for AppErrorType {
    fn to_string(&self) -> String {
        match self {
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
    fn new(message: String, error_type: AppErrorType) -> AppError {
        AppError {
            message,
            error_type,
        }
    }

    pub fn internal_server_error() -> AppError {
        AppError::new(
            "internal server error".to_string(),
            AppErrorType::InternalServerError,
        )
    }

    pub fn check(message: String) -> AppError {
        AppError::new(message, AppErrorType::CheckError)
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
