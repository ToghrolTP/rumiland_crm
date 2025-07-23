//! Integration tests for Rumiland CRM

use sqlx::SqlitePool;

#[tokio::test]
async fn test_database_connection() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create test database");
    
    // Run migrations
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
    .expect("Failed to create table");
    
    // Test insert
    let result = sqlx::query(
        "INSERT INTO customers (full_name, company, email, phone_number, notes) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind("Test User")
    .bind("Test Company")
    .bind("test@example.com")
    .bind("1234567890")
    .bind("Test notes")
    .execute(&pool)
    .await;
    
    assert!(result.is_ok());
}

#[test]
fn test_password_hashing() {
    use bcrypt::{hash, verify, DEFAULT_COST};
    
    let password = "test_password";
    let hashed = hash(password, DEFAULT_COST).unwrap();
    
    assert!(verify(password, &hashed).unwrap());
    assert!(!verify("wrong_password", &hashed).unwrap());
}