use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use sqlx::{Pool, Sqlite};

use crate::{
    error::{AppError, AppResult},
    middleware::auth::{get_current_user, require_admin},
    models::{User, UserForm},
    templates::users::{AddUserTemplate, UsersTemplate},
    utils::password::hash_password,
};

/// List all users (admin only)
pub async fn list_users(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    // Require admin role
    let _ = require_admin(&pool, &jar).await?;
    
    let current_user = get_current_user(&pool, &jar).await;
    
    // Check for flash message
    let flash_message = jar.get("flash_message").map(|c| c.value().to_string());
    
    // Remove flash message cookie after reading
    let jar = if flash_message.is_some() {
        jar.remove(Cookie::from("flash_message"))
    } else {
        jar
    };
    
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error fetching users: {:?}", e);
            AppError::Internal("خطا در دریافت لیست کاربران".to_string())
        })?;
    
    let template = UsersTemplate {
        users,
        current_user,
        active_page: "users",
        flash_message,
    };
    
    Ok((jar, Html(template.render()?)))
}

/// Show add user form (admin only)
pub async fn show_add_user_form(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let _ = require_admin(&pool, &jar).await?;
    
    let current_user = get_current_user(&pool, &jar).await;
    
    let template = AddUserTemplate {
        current_user,
        active_page: "users",
    };
    
    Ok(Html(template.render()?))
}

/// Add new user (admin only)
pub async fn add_user(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Form(form): Form<UserForm>,
) -> AppResult<impl IntoResponse> {
    let admin = require_admin(&pool, &jar).await
        .map_err(|_| AppError::Forbidden)?;
    
    // Validate role
    if form.role != "admin" && form.role != "user" {
        return Err(AppError::BadRequest("Invalid role".to_string()));
    }
    
    // Hash password
    let password_hash = hash_password(&form.password)?;
    
    // Insert user
    sqlx::query(
        "INSERT INTO users (username, password_hash, full_name, role) VALUES (?, ?, ?, ?)"
    )
    .bind(&form.username)
    .bind(password_hash)
    .bind(&form.full_name)
    .bind(&form.role)
    .execute(&pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            AppError::BadRequest("Username already exists".to_string())
        } else {
            AppError::from(e)
        }
    })?;
    
    println!("👤 New user created: {} by {}", form.username, admin.username);
    
    // Set flash message
    let role_text = if form.role == "admin" { "مدیر" } else { "کاربر" };
    let flash_cookie = Cookie::build(("flash_message", format!("کاربر «{}» با نقش {} با موفقیت ایجاد شد 👤", form.full_name, role_text)))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();
    
    let jar = jar.add(flash_cookie);
    
    Ok((jar, Redirect::to("/users")))
}

/// Delete user (admin only)
pub async fn delete_user(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let admin = require_admin(&pool, &jar).await
        .map_err(|_| AppError::Forbidden)?;
    
    // Don't allow deleting yourself
    if admin.id == id {
        return Err(AppError::BadRequest("Cannot delete yourself".to_string()));
    }
    
    // Get user name before deletion
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    
    let user_name = user.map(|u| u.full_name).unwrap_or_else(|| "کاربر".to_string());
    
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    
    println!("🗑️ User deleted by admin: {}", admin.username);
    
    // Set flash message
    let flash_cookie = Cookie::build(("flash_message", format!("کاربر «{}» با موفقیت حذف شد 🗑️", user_name)))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();
    
    let jar = jar.add(flash_cookie);
    
    Ok((jar, Redirect::to("/users")))
}