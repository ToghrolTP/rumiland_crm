use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Session entity for user authentication
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub expires_at: String,
}

impl Session {
    /// Create a new session ID
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Create expiry timestamp (24 hours from now)
    pub fn generate_expiry() -> String {
        use chrono::{Duration, Utc};
        (Utc::now() + Duration::hours(24)).to_rfc3339()
    }
}