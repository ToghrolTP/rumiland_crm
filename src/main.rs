//! Rumiland CRM - A simple CRM system with Persian/RTL support
//! 
//! This application provides customer relationship management functionality
//! with user authentication and role-based access control.

mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod templates;
mod utils;

use std::env;

use crate::{
    config::Config,
    db::{connection::create_pool, migrations::{run_migrations, create_default_admin}},
};

#[tokio::main]
async fn main() {
    // Check for CLI commands
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "create-admin" {
        create_admin_cli().await;
        return;
    }
    
    // Load configuration
    let config = Config::from_env();
    
    // Initialize database
    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Run migrations
    if let Err(e) = run_migrations(&pool).await {
        eprintln!("❌ Failed to run migrations: {}", e);
        std::process::exit(1);
    }
    
    // Create default admin if needed
    if let Err(e) = create_default_admin(&pool).await {
        eprintln!("⚠️  Warning: Failed to create default admin: {}", e);
    }
    
    // Configure routes
    let app = handlers::configure_routes(pool);
    
    // Start server
    let addr = config.server_address();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind server address");
    
    println!("🚀 Server running at http://{}", addr);
    println!("📊 Database: {}", config.database_url);
    println!("🔐 Authentication enabled");
    println!("📝 Visit http://{}/login to get started", addr);
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

/// CLI command to create admin user interactively
async fn create_admin_cli() {
    use std::io::{self, Write};
    use crate::utils::password::hash_password;
    
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
    let password_hash = match hash_password(password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("❌ Error hashing password: {}", e);
            return;
        }
    };
    
    // Connect to database
    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            return;
        }
    };
    
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