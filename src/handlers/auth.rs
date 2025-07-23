use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use sqlx::{Pool, Sqlite};

use crate::{
    models::{LoginForm, Session, User},
    templates::auth::LoginTemplate,
    utils::password::verify_password,
};

/// Show login page
pub async fn show_login(jar: CookieJar) -> impl IntoResponse {
    // If already logged in, redirect to home
    if jar.get("session_id").is_some() {
        return Redirect::to("/").into_response();
    }
    
    let template = LoginTemplate { error: None };
    Html(template.render().unwrap()).into_response()
}

/// Handle login form submission
pub async fn do_login(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), (CookieJar, impl IntoResponse)> {
    // Find user by username
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(&form.username)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);
    
    if let Some(user) = user {
        // Verify password
        if verify_password(&form.password, &user.password_hash).unwrap_or(false) {
            // Create session
            let session_id = Session::generate_id();
            let expires_at = Session::generate_expiry();
            
            sqlx::query(
                "INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)"
            )
            .bind(&session_id)
            .bind(user.id)
            .bind(&expires_at)
            .execute(&pool)
            .await
            .unwrap();
            
            // Set session cookie
            let cookie = Cookie::build(("session_id", session_id))
                .path("/")
                .same_site(SameSite::Lax)
                .http_only(true)
                .build();
            
            println!("✅ User logged in: {}", user.username);
            return Ok((jar.add(cookie), Redirect::to("/")));
        }
    }
    
    // Login failed
    let template = LoginTemplate {
        error: Some("نام کاربری یا رمز عبور اشتباه است".to_string()),
    };
    Err((jar, Html(template.render().unwrap())))
}

/// Handle logout
pub async fn logout(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> (CookieJar, Redirect) {
    // Remove session from database
    if let Some(session_cookie) = jar.get("session_id") {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_cookie.value())
            .execute(&pool)
            .await
            .ok();
    }
    
    // Remove cookie
    let jar = jar.remove(Cookie::from("session_id"));
    (jar, Redirect::to("/login"))
}