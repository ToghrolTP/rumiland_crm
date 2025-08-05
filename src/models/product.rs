use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub stock: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductForm {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}

impl Product {
    pub fn formatted_price(&self) -> String {
        let price_str = format!("{:.0}", self.price);
        format!(
            "{} تومان",
            crate::utils::localization::to_persian_digits(&price_str)
        )
    }

    pub fn stock_status_class(&self) -> &'static str {
        if self.stock > 50 {
            "badge-success"
        } else if self.stock > 0 {
            "badge-warning"
        } else {
            "badge-error"
        }
    }
}
