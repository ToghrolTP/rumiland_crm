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
    error::AppResult,
    middleware::auth::get_current_user,
    models::{Product, ProductForm},
    templates::catalog::{AddProductTemplate, CatalogTemplate},
};

/// Show the product catalog
pub async fn show_catalog(
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

    // Fetch products from the database
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;

    let template = CatalogTemplate {
        products,
        current_user,
        active_page: "catalog",
        flash_message,
    };

    Ok((jar, Html(template.render()?)))
}


/// Show the form to add a new product
pub async fn show_add_product_form(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let current_user = get_current_user(&pool, &jar).await;
    let template = AddProductTemplate {
        current_user,
        active_page: "catalog",
    };
    Ok(Html(template.render()?))
}

/// Handle the submission of the new product form
pub async fn add_product(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Form(form): Form<ProductForm>,
) -> AppResult<impl IntoResponse> {
    // Insert the new product into the database
    sqlx::query("INSERT INTO products (name, description, price, stock) VALUES (?, ?, ?, ?)")
        .bind(&form.name)
        .bind(&form.description)
        .bind(form.price)
        .bind(form.stock)
        .execute(&pool)
        .await?;

    println!("📦 New product added to database: {}", form.name);

    // Set a flash message to confirm the action
    let flash_cookie = Cookie::build(("flash_message", format!("محصول «{}» با موفقیت اضافه شد ✅", form.name)))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(60))
        .build();
    
    let jar = jar.add(flash_cookie);

    Ok((jar, Redirect::to("/catalog")))
}