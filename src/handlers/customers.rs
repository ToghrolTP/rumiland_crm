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
    middleware::auth::get_current_user,
    models::{Customer, CustomerForm},
    templates::customers::{AddTemplate, DetailTemplate, EditTemplate, ListTemplate},
    utils::{email::{validate_email, normalize_email}, phone::normalize_phone_number},
};

/// List all customers
pub async fn list_customers(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;
    
    // Check for flash message
    let flash_message = jar.get("flash_message").map(|c| c.value().to_string());
    
    // Remove flash message cookie after reading
    let jar = if flash_message.is_some() {
        jar.remove(Cookie::from("flash_message"))
    } else {
        jar
    };
    
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY id DESC")
        .fetch_all(&pool)
        .await?;
    
    let template = ListTemplate {
        customers,
        active_page: "list",
        current_user,
        flash_message,
    };
    
    Ok((jar, Html(template.render()?)))
}

/// Show add customer form
pub async fn show_add_form(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;
    
    let template = AddTemplate {
        active_page: "add",
        current_user,
        flash_message: None,
    };
    
    Ok(Html(template.render()?))
}

/// Add new customer
pub async fn add_customer(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Form(mut form): Form<CustomerForm>,
) -> AppResult<impl IntoResponse> {
    // Validate required fields
    if form.full_name.trim().is_empty() {
        return Err(AppError::BadRequest("نام کامل نمی‌تواند خالی باشد".to_string()));
    }
    
    if form.company.trim().is_empty() {
        return Err(AppError::BadRequest("نام شرکت نمی‌تواند خالی باشد".to_string()));
    }
    
    // Validate and normalize phone number
    form.phone_number = normalize_phone_number(&form.phone_number)
        .map_err(|e| match e {
            AppError::BadRequest(msg) => AppError::Validation(vec![
                ("phone_number".to_string(), msg)
            ]),
            _ => e
        })?;
    
    // Validate and normalize email
    form.email = validate_email(&form.email)
        .map_err(|e| match e {
            AppError::BadRequest(msg) => AppError::Validation(vec![
                ("email".to_string(), msg)
            ]),
            _ => e
        })?;
    form.email = normalize_email(&form.email);
    
    // Trim all fields
    form.full_name = form.full_name.trim().to_string();
    form.company = form.company.trim().to_string();
    form.notes = form.notes.trim().to_string();
    
    // Check for duplicate email
    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM customers WHERE email = ?")
        .bind(&form.email)
        .fetch_optional(&pool)
        .await?;
        
    if existing.is_some() {
        return Err(AppError::DuplicateEntry("email".to_string()));
    }
    
    sqlx::query(
        "INSERT INTO customers (full_name, company, email, phone_number, notes) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&form.full_name)
    .bind(&form.company)
    .bind(&form.email)
    .bind(&form.phone_number)
    .bind(&form.notes)
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error while adding customer: {:?}", e);
        AppError::from(e)
    })?;
    
    println!("✅ New customer added: {}", form.full_name);
    
    // Set flash message cookie
    let flash_cookie = Cookie::build(("flash_message", format!("مشتری «{}» با موفقیت اضافه شد ✅", form.full_name)))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();
    
    let jar = jar.add(flash_cookie);
    
    Ok((jar, Redirect::to("/")))
}

/// View customer details
pub async fn view_customer(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;
    
    // Check for flash message
    let flash_message = jar.get("flash_message").map(|c| c.value().to_string());
    
    // Remove flash message cookie after reading
    let jar = if flash_message.is_some() {
        jar.remove(Cookie::from("flash_message"))
    } else {
        jar
    };
    
    let customer = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    let template = DetailTemplate {
        customer,
        active_page: "",
        current_user,
        flash_message,
    };
    
    Ok((jar, Html(template.render()?)))
}

/// Show edit form
pub async fn show_edit_form(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;
    
    let customer = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    let template = EditTemplate {
        customer,
        active_page: "",
        current_user,
    };
    
    Ok(Html(template.render()?))
}

/// Update customer
pub async fn update_customer(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Form(mut form): Form<CustomerForm>,
) -> AppResult<impl IntoResponse> {
    // Validate and normalize phone number
    form.phone_number = normalize_phone_number(&form.phone_number)?;
    
    // Validate and normalize email
    form.email = validate_email(&form.email)?;
    form.email = normalize_email(&form.email);
    
    let result = sqlx::query(
        "UPDATE customers 
         SET full_name = ?, company = ?, email = ?, phone_number = ?, notes = ?
         WHERE id = ?"
    )
    .bind(&form.full_name)
    .bind(&form.company)
    .bind(&form.email)
    .bind(&form.phone_number)
    .bind(&form.notes)
    .bind(id)
    .execute(&pool)
    .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    println!("✏️ Updated customer: {} (ID: {})", form.full_name, id);
    
    // Set flash message for update
    let flash_cookie = Cookie::build(("flash_message", format!("اطلاعات «{}» با موفقیت به‌روزرسانی شد ✏️", form.full_name)))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();
    
    let jar = jar.add(flash_cookie);
    
    Ok((jar, Redirect::to(&format!("/customer/{}", id))))
}

/// Delete customer
pub async fn delete_customer(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    // Get customer name before deletion for flash message
    let customer = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;
    
    let customer_name = customer.map(|c| c.full_name).unwrap_or_else(|| "مشتری".to_string());
    
    sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    
    println!("🗑️ Deleted customer ID: {}", id);
    
    // Set flash message for deletion
    let flash_cookie = Cookie::build(("flash_message", format!("مشتری «{}» با موفقیت حذف شد 🗑️", customer_name)))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();
    
    let jar = jar.add(flash_cookie);
    
    Ok((jar, Redirect::to("/")))
}