use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use tower_http::services::ServeDir;

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

// Template for the customer list page
#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    customers: Vec<Customer>,
    active_page: &'static str,
}

// Template for the add customer page - no fields needed
#[derive(Template)]
#[template(path = "add.html")]
struct AddTemplate {
    active_page: &'static str,
}

// Template for the customer detail page
#[derive(Template)]
#[template(path = "detail.html")]
struct DetailTemplate {
    customer: Customer,
    active_page: &'static str,
}

// Template for the edit customer page
#[derive(Template)]
#[template(path = "edit.html")]
struct EditTemplate {
    customer: Customer,
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

#[tokio::main]
async fn main() {
    // Create the database connection
    let pool = setup_database().await;
    
    // Create our application with routes
    let app = Router::new()
        .route("/", get(list_customers))
        .route("/add", get(show_add_form).post(add_customer))
        .route("/customer/:id", get(view_customer))
        .route("/delete/:id", post(delete_customer))
        .route("/edit/:id", get(show_edit_form).post(update_customer))
        .nest_service("/static", ServeDir::new("static"))  // Serve static files
        .with_state(pool);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running at http://0.0.0.0:3000");
    println!("📊 Database initialized at rumiland.db");
    
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
    State(pool): State<SqlitePool>
) -> impl IntoResponse {
    // Fetch all customers from the database
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    
    // Create the template with our data
    let template = ListTemplate { 
        customers,
        active_page: "list",
    };
    
    // Render the template to HTML
    Html(template.render().unwrap())
}

// Handler to show the add customer form
async fn show_add_form() -> impl IntoResponse {
    let template = AddTemplate {
        active_page: "add",
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
    Path(id): Path<i64>,
) -> impl IntoResponse {
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
    Path(id): Path<i64>,
) -> impl IntoResponse {
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