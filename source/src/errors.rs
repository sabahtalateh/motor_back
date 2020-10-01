use juniper::{graphql_value, FieldError, IntoFieldError};

use juniper::sa::_core::fmt::Formatter;
use std::fmt;
use AppErrorType::*;

#[derive(Debug, Clone)]
pub enum AppErrorType {
    NotFound,
    Unauthorized,
    AccessExpired,
    InternalServerError,
    CheckError,
    OtherError,
}

impl ToString for AppErrorType {
    fn to_string(&self) -> String {
        match self {
            NotFound => "not_found",
            Unauthorized => "unauthorized",
            AccessExpired => "access_expired",
            InternalServerError => "internal_server_error",
            CheckError => "check_error",
            OtherError => "other_error",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct AppError {
    message: String,
    error_type: AppErrorType,
}

impl AppError {
    fn new(message: &str, error_type: AppErrorType) -> AppError {
        AppError {
            message: message.to_string(),
            error_type,
        }
    }

    pub fn unauthorized() -> AppError {
        AppError::new("Unauthorized", AppErrorType::Unauthorized)
    }

    pub fn access_expire() -> AppError {
        AppError::new("Access Expired", AppErrorType::AccessExpired)
    }

    pub fn login_failed() -> AppError {
        AppError::new("Login Failed", AppErrorType::Unauthorized)
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

    pub fn other_error(message: &str) -> AppError {
        AppError::new(message, AppErrorType::OtherError)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
