use std::fs;

use askama::Template;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, method},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};

use xlsxwriter::{Workbook, XlsxError};

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
    utils::{
        email::{normalize_email, validate_email},
        localization::persian_to_english_numbers,
        phone::normalize_phone_number,
    },
};

use parsidate::ParsiDate;

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
        cities: crate::models::City::all_cities(),
        methods: crate::models::SettlementMethod::all_methods(),
    };

    Ok(Html(template.render()?))
}

/// Add new customer
pub async fn add_customer(
    State(pool): State<Pool<Sqlite>>,
    jar: CookieJar,
    Form(mut form): Form<CustomerForm>,
) -> AppResult<impl IntoResponse> {
    // Trim all fields
    form.full_name = form.full_name.trim().to_string();
    form.company = form.company.trim().to_string();
    form.notes = form.notes.trim().to_string();
    form.job_title = form.job_title.trim().to_string();
    form.address = form.address.trim().to_string();
    form.city = form.city.trim().to_string();
    form.settlement_method = form.settlement_method.trim().to_string();

    // Validate required fields
    if form.full_name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "نام کامل نمی‌تواند خالی باشد".to_string(),
        ));
    }

    if form.company.trim().is_empty() {
        return Err(AppError::BadRequest(
            "نام شرکت نمی‌تواند خالی باشد".to_string(),
        ));
    }

    // Validate and normalize phone number
    form.phone_number = normalize_phone_number(&form.phone_number).map_err(|e| match e {
        AppError::BadRequest(msg) => AppError::Validation(vec![("phone_number".to_string(), msg)]),
        _ => e,
    })?;

    // Validate and normalize email
    form.email = validate_email(&form.email).map_err(|e| match e {
        AppError::BadRequest(msg) => AppError::Validation(vec![("email".to_string(), msg)]),
        _ => e,
    })?;
    form.email = normalize_email(&form.email);

    // Validate sales_count
    if form.sales_count < 0 {
        return Err(AppError::BadRequest(
            "تعداد فروش نمیتواند منفی باشد".to_string(),
        ));
    }

    // Validate settlement_method
    let valid_methods: Vec<String> = crate::models::SettlementMethod::all_methods()
        .into_iter()
        .map(|method| method.as_str().to_string())
        .collect();

    let method_str = form.settlement_method;
    if !valid_methods.contains(&method_str.to_string()) && !method_str.is_empty() {
        return Err(AppError::BadRequest(
            "نحوه تسویه انتخاب شده معتبر نیست".to_string(),
        ));
    }

    // Validate purchase_date
    let purchase_date_gregorian = if form.purchase_date.is_empty() {
        "".to_string()
    } else {
        let normalized_date = persian_to_english_numbers(&form.purchase_date);

        match ParsiDate::parse(&normalized_date, "%Y/%m/%d") {
            Ok(parsi_date) => match parsi_date.to_gregorian() {
                Ok(gregorian_date) => gregorian_date.format("%Y-%m-%d").to_string(),
                Err(_) => {
                    return Err(AppError::BadRequest("خطا در تبدیل تاریخ".to_string()));
                }
            },
            Err(_) => {
                return Err(AppError::BadRequest(
                    "فرمت تاریخ خرید معتبر نیست".to_string(),
                ));
            }
        }
    };

    // Validate city
    let city_str = form.city;

    let valid_cities: Vec<String> = crate::models::City::all_cities()
        .into_iter()
        .map(|city| city.as_str().to_string())
        .collect();

    if !valid_cities.contains(&city_str.to_string()) && !city_str.is_empty() {
        return Err(AppError::BadRequest(
            "شهر انتخاب شده معتبر نیست".to_string(),
        ));
    }

    // Check for duplicate email
    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM customers WHERE email = ?")
        .bind(&form.email)
        .fetch_optional(&pool)
        .await?;

    if existing.is_some() {
        return Err(AppError::DuplicateEntry("email".to_string()));
    }

    sqlx::query(
        "INSERT INTO customers (full_name, company, email, phone_number, sales_count, settlement_method, purchase_date, job_title, city, address, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&form.full_name)
    .bind(&form.company)
    .bind(&form.email)
    .bind(&form.phone_number)
    .bind(&form.sales_count)
    .bind(method_str)
    .bind(&purchase_date_gregorian)
    .bind(&form.job_title)
    .bind(city_str)
    .bind(&form.address)
    .bind(&form.notes)
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error while adding customer: {:?}", e);
        AppError::from(e)
    })?;

    println!("✅ New customer added: {}", form.full_name);

    // Set flash message cookie
    let flash_cookie = Cookie::build((
        "flash_message",
        format!("مشتری «{}» با موفقیت اضافه شد ✅", form.full_name),
    ))
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

    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
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

    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let template = EditTemplate {
        customer,
        active_page: "",
        current_user,
        cities: crate::models::City::all_cities(),
        methods: crate::models::SettlementMethod::all_methods(),
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

    // Validate cities
    let valid_cities: Vec<String> = crate::models::City::all_cities()
        .into_iter()
        .map(|city| city.as_str().to_string())
        .collect();

    let city_str = form.city.trim();

    if !valid_cities.contains(&city_str.to_string()) && !city_str.is_empty() {
        return Err(AppError::BadRequest(
            "شهر انتخاب شده معتبر نیست".to_string(),
        ));
    }

    // Validate sales_count
    if form.sales_count < 0 {
        return Err(AppError::BadRequest(
            "تعداد فروش نمیتواند منفی باشد".to_string(),
        ));
    }

    // Validate settlement_method
    let valid_methods: Vec<String> = crate::models::SettlementMethod::all_methods()
        .into_iter()
        .map(|method| method.as_str().to_string())
        .collect();

    let method_str = form.settlement_method;
    if !valid_methods.contains(&method_str.to_string()) && !method_str.is_empty() {
        return Err(AppError::BadRequest(
            "نحوه تسویه انتخاب شده معتبر نیست".to_string(),
        ));
    }

    // Validate purchase_date
    let purchase_date_gregorian = if form.purchase_date.is_empty() {
        "".to_string()
    } else {
        let normalized_date = persian_to_english_numbers(&form.purchase_date);
        match ParsiDate::parse(&normalized_date, "%Y/%m/%d") {
            Ok(parsi_date) => match parsi_date.to_gregorian() {
                Ok(gregorian_date) => gregorian_date.format("%Y-%m-%d").to_string(),
                Err(_) => {
                    return Err(AppError::BadRequest("خطا در تبدیل تاریخ".to_string()));
                }
            },
            Err(_) => {
                return Err(AppError::BadRequest(
                    "فرمت تاریخ خرید معتبر نیست".to_string(),
                ));
            }
        }
    };

    // Validate and normalize email
    form.email = validate_email(&form.email)?;
    form.email = normalize_email(&form.email);

    let result = sqlx::query(
        "UPDATE customers
         SET full_name = ?, company = ?, email = ?, phone_number = ?, sales_count = ?, settlement_method = ?, purchase_date = ?, job_title = ?, city = ?, address = ?, notes = ?
         WHERE id = ?"
    )
    .bind(&form.full_name)
    .bind(&form.company)
    .bind(&form.email)
    .bind(&form.phone_number)
    .bind(&form.sales_count)
    .bind(method_str)
    .bind(&purchase_date_gregorian)
    .bind(&form.job_title)
    .bind(city_str)
    .bind(&form.address)
    .bind(&form.notes)
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    println!("✏️ Updated customer: {} (ID: {})", form.full_name, id);

    // Set flash message for update
    let flash_cookie = Cookie::build((
        "flash_message",
        format!("اطلاعات «{}» با موفقیت به‌روزرسانی شد ✏️", form.full_name),
    ))
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
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    let customer_name = customer
        .map(|c| c.full_name)
        .unwrap_or_else(|| "مشتری".to_string());

    sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    println!("🗑️ Deleted customer ID: {}", id);

    // Set flash message for deletion
    let flash_cookie = Cookie::build((
        "flash_message",
        format!("مشتری «{}» با موفقیت حذف شد 🗑️", customer_name),
    ))
    .path("/")
    .same_site(SameSite::Lax)
    .http_only(true)
    .max_age(cookie::time::Duration::seconds(60))
    .build();

    let jar = jar.add(flash_cookie);

    Ok((jar, Redirect::to("/")))
}

pub async fn export_customer(State(pool): State<Pool<Sqlite>>) -> AppResult<Response> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY id DESC")
        .fetch_all(&pool)
        .await?;

    
    let temp_file_path = format!("/tmp/{}.xlsx", uuid::Uuid::new_v4());
    
    
    let workbook = Workbook::new(&temp_file_path)?;
    let mut sheet = workbook.add_worksheet(None)?;

    let headers = [
        "ID",
        "نام کامل",
        "شرکت",
        "ایمیل",
        "شماره تلفن",
        "تعداد فروش",
        "نحوه تسویه",
        "تاریخ خرید",
        "سمت شغلی",
        "شهر",
        "آدرس",
        "یادداشت‌ها",
    ];
    
    for (i, header) in headers.iter().enumerate() {
        sheet.write_string(0, i as u16, header, None)?;
    }

    for (row_num, customer) in customers.iter().enumerate() {
        let row = (row_num + 1) as u32;
        sheet.write_number(row, 0, customer.id as f64, None)?;
        sheet.write_string(row, 1, &customer.full_name, None)?;
        sheet.write_string(row, 2, &customer.company, None)?;
        sheet.write_string(row, 3, &customer.email, None)?;
        sheet.write_string(row, 4, &customer.phone_number, None)?;
        sheet.write_number(row, 5, customer.sales_count as f64, None)?;
        sheet.write_string(row, 6, &customer.settlement_method_display_name(), None)?;
        sheet.write_string(row, 7, &customer.purchase_date_shamsi(), None)?;
        sheet.write_string(row, 8, &customer.job_title, None)?;
        sheet.write_string(row, 9, &customer.city_display_name(), None)?;
        sheet.write_string(row, 10, &customer.address, None)?;
        sheet.write_string(row, 11, &customer.notes, None)?;
    }
    
    workbook.close()?;
    
    let buffer = fs::read(&temp_file_path).map_err(|e| AppError::Internal(e.to_string()))?;
    let _ = fs::remove_file(&temp_file_path);

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()),
        (header::CONTENT_DISPOSITION, "attachment; filename=\"customers.xlsx\"".to_string()),
    ];

    Ok((headers, Body::from(buffer)).into_response())
}
