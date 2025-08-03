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

    // Check for flash message
    let flash_message = jar.get("flash_message").map(|c| c.value().to_string());

    // Check if we should show login help
    let show_help = jar.get("login_help").is_some();

    // Remove cookies after reading
    let jar = if flash_message.is_some() || show_help {
        jar.remove(Cookie::from("flash_message"))
            .remove(Cookie::from("login_help"))
    } else {
        jar
    };

    let mut template = LoginTemplate {
        error: None,
        flash_message,
    };

    // Add help message if login failed
    if show_help {
        template.error = Some("نام کاربری یا رمز عبور اشتباه است".to_string());
    }

    (jar, Html(template.render().unwrap())).into_response()
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

            sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")
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

            // Set welcome flash message
            let flash_cookie =
                Cookie::build(("flash_message", format!("خوش آمدید، {} 👋", user.full_name)))
                    .path("/")
                    .same_site(SameSite::Lax)
                    .http_only(true)
                    .max_age(cookie::time::Duration::seconds(60))
                    .build();

            let jar = jar.add(cookie).add(flash_cookie);

            println!("✅ User logged in: {}", user.username);
            return Ok((jar, Redirect::to("/")));
        }
    }

    // Login failed
    let template = LoginTemplate {
        error: Some("نام کاربری یا رمز عبور اشتباه است".to_string()),
        flash_message: None,
    };

    // Add helpful cookie with suggestions
    let help_cookie = Cookie::build(("login_help", "true"))
        .path("/login")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(30))
        .build();

    let jar = jar.add(help_cookie);

    Err((jar, Html(template.render().unwrap())))
}

/// Handle logout
pub async fn logout(State(pool): State<Pool<Sqlite>>, jar: CookieJar) -> (CookieJar, Redirect) {
    // Remove session from database
    if let Some(session_cookie) = jar.get("session_id") {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_cookie.value())
            .execute(&pool)
            .await
            .ok();
    }

    // Set logout flash message
    let flash_cookie = Cookie::build(("flash_message", "با موفقیت از سیستم خارج شدید 👋"))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();

    // Remove session cookie and add flash message
    let jar = jar.remove(Cookie::from("session_id")).add(flash_cookie);

    (jar, Redirect::to("/login"))
}
