use axum::{
    extract::State,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use sqlx::{Pool, Sqlite};

use crate::models::{Session, User};

/// Middleware to check if user is authenticated
pub async fn auth_middleware(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check for session cookie
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();
        
        // Verify session in database
        let session = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE id = ? AND expires_at > datetime('now')"
        )
        .bind(session_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);
        
        if session.is_some() {
            // Valid session, allow request to proceed
            return Ok(next.run(request).await);
        }
    }
    
    // No valid session, redirect to login
    Ok(Redirect::to("/login").into_response())
}

/// Get current user from session
pub async fn get_current_user(pool: &Pool<Sqlite>, jar: &CookieJar) -> Option<User> {
    if let Some(session_cookie) = jar.get("session_id") {
        let result = sqlx::query_as::<_, (i64,)>(
            "SELECT user_id FROM sessions WHERE id = ? AND expires_at > datetime('now')"
        )
        .bind(session_cookie.value())
        .fetch_optional(pool)
        .await
        .ok()?;
        
        if let Some((user_id,)) = result {
            return sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_optional(pool)
                .await
                .ok()?;
        }
    }
    None
}

/// Require admin role
pub async fn require_admin(pool: &Pool<Sqlite>, jar: &CookieJar) -> Result<User, StatusCode> {
    let user = get_current_user(pool, jar)
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if !user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(user)
}