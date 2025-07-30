use crate::error::AppResult;
use sqlx::{Pool, Sqlite};

/// Run all database migrations
pub async fn run_migrations(pool: &Pool<Sqlite>) -> AppResult<()> {
    // Create customers table
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
        "#,
    )
    .execute(pool)
    .await?;

    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            full_name TEXT NOT NULL,
            role TEXT NOT NULL CHECK(role IN ('admin', 'user')),
            created_at TEXT DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            expires_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create products table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            stock INTEGER NOT NULL,
            image_url TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_name ON products(name)")
        .execute(pool)
        .await?;

    println!("✅ Database migrations completed successfully");
    Ok(())
}

/// Create default admin user if no users exist
pub async fn create_default_admin(pool: &Pool<Sqlite>) -> AppResult<()> {
    use crate::utils::password::hash_password;

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if user_count.0 == 0 {
        let password_hash = hash_password("admin123")?;

        sqlx::query(
            "INSERT INTO users (username, password_hash, full_name, role)
             VALUES (?, ?, ?, ?)",
        )
        .bind("admin")
        .bind(password_hash)
        .bind("مدیر سیستم")
        .bind("admin")
        .execute(pool)
        .await?;

        println!("👤 Created default admin user (username: admin, password: admin123)");
        println!("⚠️  IMPORTANT: Change the default password after first login!");
    }

    Ok(())
}
