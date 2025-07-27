use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use std::fmt;

use crate::templates::errors::ErrorTemplate;

/// Application-wide error type
#[derive(Debug)]
pub enum AppError {
    /// Database errors
    Database(sqlx::Error),
    /// Authentication failed
    Unauthorized,
    /// Access forbidden
    Forbidden,
    /// Resource not found
    NotFound,
    /// Bad request
    BadRequest(String),
    /// Internal server error
    Internal(String),
    /// Template rendering error
    Template(askama::Error),
    /// Password hashing error
    Bcrypt(bcrypt::BcryptError),
    /// Validation error with field details
    Validation(Vec<(String, String)>),
    /// Duplicate entry error
    DuplicateEntry(String),
    /// Session expired
    SessionExpired,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Unauthorized => write!(f, "Authentication required"),
            AppError::Forbidden => write!(f, "Access forbidden"),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::Template(e) => write!(f, "Template error: {}", e),
            AppError::Bcrypt(e) => write!(f, "Password error: {}", e),
            AppError::Validation(errors) => {
                write!(f, "Validation errors: ")?;
                for (field, msg) in errors {
                    write!(f, "{}: {}, ", field, msg)?;
                }
                Ok(())
            }
            AppError::DuplicateEntry(field) => write!(f, "Duplicate entry: {}", field),
            AppError::SessionExpired => write!(f, "Session expired"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, template) = match self {
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorTemplate::for_unauthorized()
            ),
            AppError::SessionExpired => (
                StatusCode::UNAUTHORIZED,
                ErrorTemplate {
                    title: "نشست منقضی شده".to_string(),
                    message: "نشست کاربری شما منقضی شده است.".to_string(),
                    details: Some("برای ادامه کار باید دوباره وارد سیستم شوید.".to_string()),
                    suggestions: vec![
                        "روی دکمه ورود کلیک کنید".to_string(),
                        "نام کاربری و رمز عبور خود را وارد کنید".to_string(),
                    ],
                    error_code: Some("401".to_string()),
                    back_url: "/login".to_string(),
                    home_url: "/login".to_string(),
                }
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                ErrorTemplate::for_forbidden()
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorTemplate::for_not_found()
            ),
            AppError::BadRequest(ref msg) => (
                StatusCode::BAD_REQUEST,
                ErrorTemplate::for_validation_error(msg.clone())
            ),
            AppError::DuplicateEntry(ref field) => (
                StatusCode::CONFLICT,
                ErrorTemplate::for_duplicate_entry(field)
            ),
            AppError::Validation(ref errors) => {
                let details = errors.iter()
                    .map(|(field, msg)| format!("• {}: {}", field, msg))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                (
                    StatusCode::BAD_REQUEST,
                    ErrorTemplate::for_validation_error(details)
                )
            },
            AppError::Database(ref e) => {
                // Log the actual database error
                eprintln!("Database error: {:?}", e);
                
                // Check for specific database errors
                let template = if e.to_string().contains("UNIQUE constraint failed") {
                    ErrorTemplate::for_duplicate_entry("unknown")
                } else {
                    ErrorTemplate::for_database_error()
                };
                
                (StatusCode::INTERNAL_SERVER_ERROR, template)
            },
            AppError::Internal(ref msg) => {
                eprintln!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorTemplate {
                        title: "خطای سیستمی".to_string(),
                        message: "متأسفانه مشکلی در سیستم رخ داده است.".to_string(),
                        details: None,
                        suggestions: vec![
                            "صفحه را رفرش کنید (F5)".to_string(),
                            "چند دقیقه صبر کرده و دوباره امتحان کنید".to_string(),
                            "در صورت ادامه مشکل با پشتیبانی تماس بگیرید".to_string(),
                        ],
                        error_code: Some("500".to_string()),
                        back_url: "javascript:location.reload()".to_string(),
                        home_url: "/".to_string(),
                    }
                )
            },
            AppError::Template(_) | AppError::Bcrypt(_) => {
                eprintln!("Internal error: {}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorTemplate::for_database_error()
                )
            }
        };

        match template.render() {
            Ok(html) => (status, Html(html)).into_response(),
            Err(_) => (
                status, 
                Html("<h1>خطا در نمایش صفحه خطا</h1><p>لطفاً با پشتیبانی تماس بگیرید.</p>")
            ).into_response(),
        }
    }
}

// Implement conversions from other error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        // Check for specific database errors
        match &err {
            sqlx::Error::Database(db_err) => {
                let msg = db_err.message();
                if msg.contains("UNIQUE constraint failed") {
                    if msg.contains("username") {
                        return AppError::DuplicateEntry("username".to_string());
                    } else if msg.contains("email") {
                        return AppError::DuplicateEntry("email".to_string());
                    }
                }
            }
            _ => {}
        }
        AppError::Database(err)
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::Template(err)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Bcrypt(err)
    }
}

/// Type alias for Results in our application
pub type AppResult<T> = Result<T, AppError>;