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
    pub sales_count: i64,
    pub job_title: String,
    pub city: String,
    pub address: String,
    pub notes: String,
    pub coordinates: String,
}

/// Form data for creating/updating a customer
#[derive(Debug, Deserialize)]
pub struct CustomerForm {
    pub full_name: String,
    pub company: String,
    pub email: String,
    pub phone_number: String,
    pub sales_count: i64,
    pub job_title: String,
    pub city: String,
    pub address: String,
    pub notes: String,
    pub coordinates: String,
}

impl Customer {
    /// Format phone number for display using the phone utils
    pub fn formatted_phone(&self) -> String {
        crate::utils::phone::format_phone_for_display(&self.phone_number)
    }

    pub fn city_display_name(&self) -> String {
        City::from_str(&self.city).display_name().to_string()
    }

    
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum City {
    Hidaj,
    Khorramdarreh,
    Abhar,
    Zanjan,
    Qazvin,
    None,
}

impl City {
    pub fn all_cities() -> Vec<City> {
        vec![
            City::Hidaj,
            City::Khorramdarreh,
            City::Abhar,
            City::Zanjan,
            City::Qazvin,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            City::Hidaj => "Hidaj",
            City::Khorramdarreh => "Khorramdarreh",
            City::Abhar => "Abhar",
            City::Zanjan => "Zanjan",
            City::Qazvin => "Qazvin",
            City::None => "",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Hidaj" => City::Hidaj,
            "Khorramdarreh" => City::Khorramdarreh,
            "Abhar" => City::Abhar,
            "Zanjan" => City::Zanjan,
            "Qazvin" => City::Qazvin,
            _ => City::None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            City::Hidaj => "هیدج",
            City::Khorramdarreh => "خرمدره",
            City::Abhar => "ابهر",
            City::Zanjan => "زنجان",
            City::Qazvin => "قزوین",
            City::None => "انتخاب کنید",
        }
    }
}
