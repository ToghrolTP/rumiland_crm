use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

/// Flash message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FlashType {
    Success,
    Error,
    Warning,
    Info,
}

/// Flash message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashMessage {
    pub message: String,
    pub flash_type: FlashType,
}

impl FlashMessage {
    /// Create a success message
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            flash_type: FlashType::Success,
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            flash_type: FlashType::Error,
        }
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            flash_type: FlashType::Warning,
        }
    }

    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            flash_type: FlashType::Info,
        }
    }
}

/// Extension trait for CookieJar to handle flash messages
pub trait FlashExt {
    /// Set a flash message
    fn with_flash(self, flash: FlashMessage) -> Self;
    
    /// Get and consume flash message
    fn get_flash(&self) -> Option<FlashMessage>;
}

impl FlashExt for CookieJar {
    fn with_flash(self, flash: FlashMessage) -> Self {
        // Serialize the flash message to JSON
        let flash_json = serde_json::to_string(&flash).unwrap_or_default();
        
        // Create a cookie that expires quickly
        let cookie = Cookie::build(("flash_message", flash_json))
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(cookie::time::Duration::seconds(60)) // 1 minute expiry
            .build();
        
        self.add(cookie)
    }
    
    fn get_flash(&self) -> Option<FlashMessage> {
        self.get("flash_message")
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
    }
}