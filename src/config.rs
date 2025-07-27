use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub session_duration_hours: i64,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:rumiland.db?mode=rwc".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            session_duration_hours: env::var("SESSION_DURATION_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
        }
    }

    /// Get server address for binding
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}
