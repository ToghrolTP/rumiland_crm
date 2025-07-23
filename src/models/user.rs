use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

/// User entity representing system users
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub created_at: String,
}

/// User role enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    /// Parse role from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            _ => None,
        }
    }

    /// Convert role to string
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }

    /// Get Persian display name
    pub fn display_persian(&self) -> &'static str {
        match self {
            UserRole::Admin => "مدیر",
            UserRole::User => "کاربر",
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl User {
    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    /// Get role as enum
    pub fn role_enum(&self) -> UserRole {
        UserRole::from_str(&self.role).unwrap_or(UserRole::User)
    }
}

/// Form data for user login
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

/// Form data for creating a new user
#[derive(Debug, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub role: String,
}