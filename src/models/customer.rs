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
    pub job_title: String,
    pub city: String,
    pub address: String,
    pub notes: String,
}

/// Form data for creating/updating a customer
#[derive(Debug, Deserialize)]
pub struct CustomerForm {
    pub full_name: String,
    pub company: String,
    pub email: String,
    pub phone_number: String,
    pub job_title: String,
    pub city: String,
    pub address: String,
    pub notes: String,
}

impl Customer {
    /// Format phone number for display using the phone utils
    pub fn formatted_phone(&self) -> String {
        crate::utils::phone::format_phone_for_display(&self.phone_number)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum City {
    Hidaj,
    Khorramdarreh,
    Abhar,
    None,
}

impl City {
    pub fn as_str(&self) -> &'static str {
        match self {
            City::Hidaj => "Hidaj",
            City::Khorramdarreh => "Khorramdarreh",
            City::Abhar => "Abhar",
            City::None => "",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "Hidaj" => City::Hidaj,
            "Khorramdarreh" => City::Khorramdarreh,
            "Abhar" => City::Abhar,
            _ => City::None,
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            City::Hidaj => "هیدج",
            City::Khorramdarreh => "خرمدره",
            City::Abhar => "ابهر",
            City::None => "انتخواب کنید",
        }
    }
}