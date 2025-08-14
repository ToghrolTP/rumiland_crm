use std::fmt::format;

use crate::{
    error::{AppError, AppResult},
    middleware::auth::get_current_user,
    models::{Customer, Transaction, TransactionForm, TransactionType},
    templates::transactions::{AddTransactionTemplate, EditTransactionTemplate},
    utils::localization::persian_to_english_numbers,
};
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

#[axum::debug_handler]
pub async fn show_add_transaction_form(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(customer_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;

    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(customer_id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let template = AddTransactionTemplate {
        customer,
        current_user,
        active_page: "",
        transaction_types: TransactionType::all(),
    };

    Ok(Html(template.render()?))
}

#[axum::debug_handler]
pub async fn show_edit_transaction_form(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path((customer_id, transaction_id)): Path<(i64, i64)>,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;

    let transaction = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = ? AND customer_id = ?")
        .bind(transaction_id)
        .bind(customer_id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;
    // let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
    //     .bind(customer_id)
    //     .fetch_optional(&pool)
    //     .await?
    //     .ok_or(AppError::NotFound)?;
         let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(customer_id)
        .fetch_one(&pool)
        .await?;

    let template = EditTransactionTemplate {
        customer,
        transaction,
        current_user,
        active_page: "",
        transaction_types: TransactionType::all(),
    };

    Ok(Html(template.render()?))
}

#[axum::debug_handler]
pub async fn add_transaction(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path(customer_id): Path<i64>,
    Form(form): Form<TransactionForm>,
) -> AppResult<impl IntoResponse> {
    let shamsi_date = persian_to_english_numbers(&form.transaction_date);

    if shamsi_date.trim().is_empty() {
        return Err(AppError::BadRequest("تاریخ تراکنش الزامی است".to_string()));
    }

    let parts: Vec<&str> = shamsi_date.split('/').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return Err(AppError::BadRequest(
            "فرمت تاریخ تراکنش معتبر نیست. لطفا از فرمت YYYY/MM/DD استفاده کنید".to_string(),
        ));
    }

    sqlx::query(
        "INSERT INTO transactions (customer_id, amount, transaction_type, description, transaction_date)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(customer_id)
    .bind(form.amount)
    .bind(form.transaction_type)
    .bind(form.description)
    .bind(form.transaction_date)
    .execute(&pool)
    .await?;

    let flash_cookie = Cookie::build((
        "flash_message",
        "تراکنش جدید با موفقیت ثبت شد ✅".to_string(),
    ))
    .path(format!("/customer/{}", customer_id))
    .same_site(SameSite::Lax)
    .http_only(true)
    .build();

    let jar = jar.add(flash_cookie);

    Ok((jar, Redirect::to(&format!("/customer/{}", customer_id))))
}

pub async fn delete_transaction(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path((customer_id, transaction_id)): Path<(i64, i64)>,
) -> AppResult<impl IntoResponse> {
    sqlx::query("DELETE FROM transactions WHERE id = ?")
        .bind(transaction_id)
        .execute(&pool)
        .await?;

    println!("🗑️ Deleted transaction ID: {}", transaction_id);

    // Set flash message for deletion
    let flash_cookie = Cookie::build((
        "flash_message",
        format!("تراکنش {} با موفقیت حذف شد 🗑️", transaction_id),
    ))
    .path(format!("/customer/{}", customer_id))
    .same_site(SameSite::Lax)
    .http_only(true)
    .max_age(cookie::time::Duration::seconds(60))
    .build();

    let jar = jar.add(flash_cookie);

    Ok((jar, Redirect::to(&format!("/customer/{}", customer_id))))
}

pub async fn edit_transaction(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Path((customer_id, transaction_id)): Path<(i64, i64)>,
    Form(mut form): Form<TransactionForm>,
) -> AppResult<impl IntoResponse> {
    let shamsi_date = persian_to_english_numbers(&form.transaction_date);

    if shamsi_date.trim().is_empty() {
        return Err(AppError::BadRequest("تاریخ تراکنش الزامی است".to_string()));
    }

    let parts: Vec<&str> = shamsi_date.split('/').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return Err(AppError::BadRequest(
            "فرمت تاریخ تراکنش معتبر نیست. لطفا از فرمت YYYY/MM/DD استفاده کنید".to_string(),
        ));
    }

    let result = sqlx::query(
        "UPDATE transactions
         SET amount = ?, transaction_type = ?, description = ?, transaction_date = ?
         WHERE id = ?",
    )
    .bind(&form.amount)
    .bind(&form.transaction_type)
    .bind(&form.description)
    .bind(&form.transaction_date)
    .bind(transaction_id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    println!("✏️ Updated transaction: (ID: {})", transaction_id);

    // Set flash message for update
    let flash_cookie = Cookie::build(("flash_message", "تراکنش با موفقیت ویرایش شد"))
        .path(format!("/customer/{}", customer_id))
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();

    let jar = jar.add(flash_cookie);

    Ok((jar, Redirect::to(&format!("/customer/{}", customer_id))))
}
