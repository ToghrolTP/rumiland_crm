use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Customer entity representing a CRM customer
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: i64,
    pub full_name: String,
    pub company: String,
    pub email: String,
    pub phone_number: String,
    pub notes: String,
}

/// Form data for creating/updating a customer
#[derive(Debug, Deserialize)]
pub struct CustomerForm {
    pub full_name: String,
    pub company: String,
    pub email: String,
    pub phone_number: String,
    pub notes: String,
}

impl Customer {
    /// Check if notes field is empty
    pub fn has_notes(&self) -> bool {
        !self.notes.trim().is_empty()
    }
}