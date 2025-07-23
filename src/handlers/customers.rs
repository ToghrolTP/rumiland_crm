use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::CookieJar;
use sqlx::{Pool, Sqlite};

use crate::{
    error::{AppError, AppResult},
    middleware::auth::get_current_user,
    models::{Customer, CustomerForm},
    templates::customers::{AddTemplate, DetailTemplate, EditTemplate, ListTemplate},
};

/// List all customers
pub async fn list_customers(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;
    
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY id DESC")
        .fetch_all(&pool)
        .await?;
    
    let template = ListTemplate {
        customers,
        active_page: "list",
        current_user,
    };
    
    Ok(Html(template.render()?))
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
    };
    
    Ok(Html(template.render()?))
}

/// Add new customer
pub async fn add_customer(
    State(pool): State<Pool<Sqlite>>,
    Form(form): Form<CustomerForm>,
) -> AppResult<Redirect> {
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
    .await?;
    
    println!("✅ New customer added: {}", form.full_name);
    Ok(Redirect::to("/"))
}

/// View customer details
pub async fn view_customer(
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
    
    let template = DetailTemplate {
        customer,
        active_page: "",
        current_user,
    };
    
    Ok(Html(template.render()?))
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
    Path(id): Path<i64>,
    Form(form): Form<CustomerForm>,
) -> AppResult<Redirect> {
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
    Ok(Redirect::to(&format!("/customer/{}", id)))
}

/// Delete customer
pub async fn delete_customer(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> AppResult<Redirect> {
    sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    
    println!("🗑️ Deleted customer ID: {}", id);
    Ok(Redirect::to("/"))
}