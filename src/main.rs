use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use axum_extra::{
    extract::{
        cookie::{Cookie, SameSite},
        CookieJar,
    },
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use tower_http::services::ServeDir;
use uuid::Uuid;

// This struct defines what a Customer looks like in our system
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Customer {
    id: i64,                    // Unique identifier (automatic)
    full_name: String,          // نام کامل
    company: String,            // شرکت
    email: String,              // ایمیل
    phone_number: String,       // شماره تلفن
    notes: String,              // یادداشت‌ها
}

// User model for authentication
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: i64,
    username: String,
    password_hash: String,
    full_name: String,
    role: String, // "admin" or "user"
    created_at: String, // SQLite stores as TEXT
}

// Session model
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Session {
    id: String,
    user_id: i64,
    expires_at: String, // SQLite stores as TEXT
}

// Template for the customer list page
#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    customers: Vec<Customer>,
    active_page: &'static str,
    current_user: Option<User>,
}

// Template for the add customer page - no fields needed
#[derive(Template)]
#[template(path = "add.html")]
struct AddTemplate {
    active_page: &'static str,
    current_user: Option<User>,
}

// Template for the customer detail page
#[derive(Template)]
#[template(path = "detail.html")]
struct DetailTemplate {
    customer: Customer,
    active_page: &'static str,
    current_user: Option<User>,
}

// Template for the edit customer page
#[derive(Template)]
#[template(path = "edit.html")]
struct EditTemplate {
    customer: Customer,
    active_page: &'static str,
    current_user: Option<User>,
}

// Template for login page
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
}

// Template for user management page (admin only)
#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<User>,
    current_user: Option<User>,
    active_page: &'static str,
}

// Template for add user page
#[derive(Template)]
#[template(path = "add_user.html")]
struct AddUserTemplate {
    current_user: Option<User>,
    active_page: &'static str,
}

// Form data structure - matches the HTML form fields
#[derive(Deserialize)]
struct CustomerForm {
    full_name: String,
    company: String,
    email: String,
    phone_number: String,
    notes: String,
}

// Login form data
#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

// User creation form
#[derive(Deserialize)]
struct UserForm {
    username: String,
    password: String,
    full_name: String,
    role: String,
}

#[tokio::main]
async fn main() {
    // Check for CLI arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "create-admin" {
        create_admin_cli().await;
        return;
    }
    
    // Normal application startup
    // Create the database connection
    let pool = setup_database().await;
    
    // Create app with public routes (no auth required)
    let public_routes = Router::new()
        .route("/login", get(show_login).post(do_login))
        .nest_service("/static", ServeDir::new("static"));
    
    // Create protected routes (auth required)
    let protected_routes = Router::new()
        .route("/", get(list_customers))
        .route("/add", get(show_add_form).post(add_customer))
        .route("/customer/:id", get(view_customer))
        .route("/delete/:id", post(delete_customer))
        .route("/edit/:id", get(show_edit_form).post(update_customer))
        .route("/logout", post(logout))
        // Admin only routes
        .route("/users", get(list_users))
        .route("/users/add", get(show_add_user_form).post(add_user))
        .route("/users/delete/:id", post(delete_user))
        .layer(middleware::from_fn_with_state(pool.clone(), auth_middleware));
    
    // Combine routes
    let app = public_routes
        .merge(protected_routes)
        .with_state(pool);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running at http://0.0.0.0:3000");
    println!("📊 Database initialized at rumiland.db");
    println!("🔐 Authentication enabled");
    
    axum::serve(listener, app).await.unwrap();
}

// Initialize database and create tables
async fn setup_database() -> SqlitePool {
    // Create or connect to the SQLite database file
    let pool = SqlitePool::connect("sqlite:rumiland.db?mode=rwc")
        .await
        .expect("Failed to create database pool");
    
    // Create the customers table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS customers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            full_name TEXT NOT NULL,
            company TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT NOT NULL,
            notes TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create customers table");
    
    println!("✅ Database setup complete!");
    
    pool
}

// Handler to list all customers
async fn list_customers(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
) -> impl IntoResponse {
    // Get current user
    let current_user = get_current_user(&pool, &jar).await;
    
    // Fetch all customers from the database
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    
    // Create the template with our data
    let template = ListTemplate { 
        customers,
        active_page: "list",
        current_user,
    };
    
    // Render the template to HTML
    Html(template.render().unwrap())
}

// Handler to show the add customer form
async fn show_add_form(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
) -> impl IntoResponse {
    let current_user = get_current_user(&pool, &jar).await;
    
    let template = AddTemplate {
        active_page: "add",
        current_user,
    };
    Html(template.render().unwrap())
}

// Handler to process the form and add a new customer
async fn add_customer(
    State(pool): State<SqlitePool>,
    Form(form): Form<CustomerForm>,
) -> impl IntoResponse {
    // Insert the new customer into the database
    let result = sqlx::query(
        "INSERT INTO customers (full_name, company, email, phone_number, notes) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&form.full_name)
    .bind(&form.company)
    .bind(&form.email)
    .bind(&form.phone_number)
    .bind(&form.notes)
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => {
            println!("✅ New customer added: {}", form.full_name);
            // Redirect to the home page after successful addition
            Redirect::to("/")
        }
        Err(e) => {
            println!("❌ Error adding customer: {}", e);
            // In a real app, you'd show an error page
            Redirect::to("/")
        }
    }
}

// Handler to view a single customer's details
async fn view_customer(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let current_user = get_current_user(&pool, &jar).await;
    
    // Fetch the customer with the given ID
    let customer = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await;
    
    match customer {
        Ok(customer) => {
            // Render the detail template with the customer data
            let template = DetailTemplate { 
                customer,
                active_page: "",
                current_user,
            };
            Html(template.render().unwrap()).into_response()
        }
        Err(_) => {
            // Customer not found, redirect to home page
            // In a real app, you might show a 404 page
            Redirect::to("/").into_response()
        }
    }
}

// Handler to delete a customer
async fn delete_customer(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // First, get the customer name for logging
    let customer_name = sqlx::query_as::<_, (String,)>(
        "SELECT full_name FROM customers WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(|(name,)| name)
    .unwrap_or_else(|_| "Unknown".to_string());
    
    // Delete the customer from the database
    let result = sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;
    
    match result {
        Ok(query_result) => {
            if query_result.rows_affected() > 0 {
                println!("🗑️ Deleted customer: {} (ID: {})", customer_name, id);
            } else {
                println!("⚠️ No customer found with ID: {}", id);
            }
        }
        Err(e) => {
            println!("❌ Error deleting customer: {}", e);
        }
    }
    
    // Always redirect to the home page
    Redirect::to("/")
}

// Handler to show the edit form
async fn show_edit_form(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let current_user = get_current_user(&pool, &jar).await;
    
    // Fetch the customer to edit
    let customer = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await;
    
    match customer {
        Ok(customer) => {
            // Show the edit form with current data
            let template = EditTemplate { 
                customer,
                active_page: "",
                current_user,
            };
            Html(template.render().unwrap()).into_response()
        }
        Err(_) => {
            // Customer not found, redirect to home
            Redirect::to("/").into_response()
        }
    }
}

// Handler to update customer data
async fn update_customer(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Form(form): Form<CustomerForm>,
) -> impl IntoResponse {
    // Update the customer in the database
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
    .await;
    
    match result {
        Ok(query_result) => {
            if query_result.rows_affected() > 0 {
                println!("✏️ Updated customer: {} (ID: {})", form.full_name, id);
                // Redirect to the customer detail page
                Redirect::to(&format!("/customer/{}", id))
            } else {
                println!("⚠️ No customer found with ID: {}", id);
                Redirect::to("/")
            }
        }
        Err(e) => {
            println!("❌ Error updating customer: {}", e);
            // In a real app, you'd show an error page
            Redirect::to(&format!("/customer/{}", id))
        }
    }
}

// Create default admin user if none exists
async fn create_default_admin(pool: &SqlitePool) {
    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    
    if user_count.0 == 0 {
        let password_hash = hash("admin123", DEFAULT_COST).unwrap();
        sqlx::query(
            "INSERT INTO users (username, password_hash, full_name, role) 
             VALUES (?, ?, ?, ?)"
        )
        .bind("admin")
        .bind(password_hash)
        .bind("مدیر سیستم")
        .bind("admin")
        .execute(pool)
        .await
        .expect("Failed to create default admin user");
        
        println!("👤 Created default admin user (username: admin, password: admin123)");
        println!("⚠️  IMPORTANT: Change the default password after first login!");
    }
}

// Authentication middleware
async fn auth_middleware(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
    request: axum::extract::Request,
    next: middleware::Next,
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

// Show login page
async fn show_login(jar: CookieJar) -> impl IntoResponse {
    // If already logged in, redirect to home
    if jar.get("session_id").is_some() {
        return Redirect::to("/").into_response();
    }
    
    let template = LoginTemplate { error: None };
    Html(template.render().unwrap()).into_response()
}

// Handle login
async fn do_login(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), (CookieJar, impl IntoResponse)> {
    // Find user by username
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&form.username)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);
    
    if let Some(user) = user {
        // Verify password
        if verify(&form.password, &user.password_hash).unwrap_or(false) {
            // Create session
            let session_id = Uuid::new_v4().to_string();
            let expires_at = (Utc::now() + chrono::Duration::hours(24)).to_rfc3339();
            
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

// Logout
async fn logout(
    State(pool): State<SqlitePool>,
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

// Get current user from session
async fn get_current_user(pool: &SqlitePool, jar: &CookieJar) -> Option<User> {
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

// List users (admin only)
async fn list_users(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let current_user = get_current_user(&pool, &jar).await
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Check if user is admin
    if current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    
    let template = UsersTemplate {
        users,
        current_user: Some(current_user),
        active_page: "users",
    };
    
    Ok(Html(template.render().unwrap()))
}

// Show add user form (admin only)
async fn show_add_user_form(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let current_user = get_current_user(&pool, &jar).await
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let template = AddUserTemplate {
        current_user: Some(current_user),
        active_page: "users",
    };
    
    Ok(Html(template.render().unwrap()))
}

// Add new user (admin only)
async fn add_user(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
    Form(form): Form<UserForm>,
) -> Result<Redirect, StatusCode> {
    let current_user = get_current_user(&pool, &jar).await
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Hash password
    let password_hash = hash(&form.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
    .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    println!("👤 New user created: {} by {}", form.username, current_user.username);
    
    Ok(Redirect::to("/users"))
}

// CLI function to create admin user
async fn create_admin_cli() {
    use std::io::{self, Write};
    
    println!("=== Rumiland CRM Admin User Creator ===\n");
    
    // Get username
    print!("Enter admin username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();
    
    // Get password
    print!("Enter admin password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    // Get full name
    print!("Enter full name: ");
    io::stdout().flush().unwrap();
    let mut full_name = String::new();
    io::stdin().read_line(&mut full_name).unwrap();
    let full_name = full_name.trim().to_string();
    
    // Generate password hash
    println!("\nGenerating password hash...");
    let password_hash = hash(password, DEFAULT_COST).unwrap();
    
    // Connect to database
    let pool = SqlitePool::connect("sqlite:rumiland.db?mode=rwc")
        .await
        .expect("Failed to connect to database");
    
    // Delete existing user if any
    sqlx::query("DELETE FROM users WHERE username = ?")
        .bind(&username)
        .execute(&pool)
        .await
        .ok();
    
    // Insert new admin user
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, full_name, role, created_at) 
         VALUES (?, ?, ?, 'admin', datetime('now'))"
    )
    .bind(&username)
    .bind(&password_hash)
    .bind(&full_name)
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => {
            println!("\n✅ Admin user created successfully!");
            println!("Username: {}", username);
            println!("Full Name: {}", full_name);
            println!("Role: admin");
            println!("\nYou can now login with these credentials.");
        }
        Err(e) => {
            eprintln!("\n❌ Error creating admin user: {}", e);
        }
    }
}

// Delete user (admin only)
async fn delete_user(
    State(pool): State<SqlitePool>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> Result<Redirect, StatusCode> {
    let current_user = get_current_user(&pool, &jar).await
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Don't allow deleting yourself
    if current_user.id == id {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to("/users"))
}