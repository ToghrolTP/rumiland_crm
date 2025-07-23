use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

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
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Authentication required"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Access forbidden"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Database(_)
            | AppError::Internal(_)
            | AppError::Template(_)
            | AppError::Bcrypt(_) => {
                // Log the actual error here in production
                eprintln!("Internal error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, message).into_response()
    }
}

// Implement conversions from other error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
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
