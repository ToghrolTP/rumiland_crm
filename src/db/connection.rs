use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;

/// Create a new database connection pool
pub async fn create_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:rumiland.db?mode=rwc".to_string());
    
    SqlitePool::connect(&database_url).await
}