use axum::{
    http::StatusCode,
    response::IntoResponse,
};

/// Health check endpoint for client-side network detection
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}