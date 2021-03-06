// use juniper::{graphql_value, FieldError, IntoFieldError};

use std::fmt;
use AppErrorType::*;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppErrorType {
    NotFound,
    Unauthorized,
    AccessExpired,
    InternalServerError,
    ValidationError,
    General,
}

impl ToString for AppErrorType {
    fn to_string(&self) -> String {
        match self {
            NotFound => "not_found",
            Unauthorized => "unauthorized",
            AccessExpired => "access_expired",
            InternalServerError => "internal_server_error",
            ValidationError => "validation_error",
            General => "general_error",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn validation(message: &str) -> AppError {
        AppError::new(message, AppErrorType::ValidationError)
    }

    pub fn internal() -> AppError {
        AppError::new("internal server error", AppErrorType::InternalServerError)
    }

    pub fn general(message: &str) -> AppError {
        AppError::new(message, AppErrorType::General)
    }

    pub fn get_type(&self) -> String {
        self.error_type.to_string()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
