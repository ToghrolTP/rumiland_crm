use axum::{
    extract::State,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use sqlx::{Pool, Sqlite};

use crate::{error::AppError, models::{Session, User}};

/// Middleware to check if user is authenticated
pub async fn auth_middleware(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, AppError> {
    // Check for session cookie
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();
        
        // Verify session in database
        let session = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE id = ?"
        )
        .bind(session_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error in auth middleware: {:?}", e);
            AppError::Internal("خطا در بررسی احراز هویت".to_string())
        })?;
        
        if let Some(session) = session {
            // Check if session is expired
            let now = chrono::Utc::now();
            let expires_at = chrono::DateTime::parse_from_rfc3339(&session.expires_at)
                .map_err(|_| AppError::Internal("خطا در پردازش زمان نشست".to_string()))?;
            
            if expires_at > now {
                // Valid session, allow request to proceed
                return Ok(next.run(request).await);
            } else {
                // Session expired, clean it up
                sqlx::query("DELETE FROM sessions WHERE id = ?")
                    .bind(session_id)
                    .execute(&pool)
                    .await
                    .ok();
                
                return Err(AppError::SessionExpired);
            }
        }
    }
    
    // No valid session
    Err(AppError::Unauthorized)
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
pub async fn require_admin(pool: &Pool<Sqlite>, jar: &CookieJar) -> Result<User, AppError> {
    let user = get_current_user(pool, jar)
        .await
        .ok_or(AppError::Unauthorized)?;
    
    if !user.is_admin() {
        return Err(AppError::Forbidden);
    }
    
    Ok(user)
}