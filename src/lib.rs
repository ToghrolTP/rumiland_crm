//! Rumiland CRM Library
//! 
//! This library provides the core functionality for the Rumiland CRM system.

pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod templates;
pub mod utils;

// Re-export commonly used types
pub use config::Config;
pub use error::{AppError, AppResult};
pub use models::{Customer, CustomerForm, LoginForm, User, UserForm, UserRole};