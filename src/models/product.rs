use serde::{Deserialize, Serialize};
use sqlx::FromRow; // Import FromRow

/// Represents a product in the digital catalog
#[derive(Debug, Clone, Serialize, FromRow)] // Add FromRow
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: Option<String>, // Change to Option<String> for flexibility
    pub stock: i32,
    pub created_at: String,
}

/// Form data for creating a new product
#[derive(Debug, Deserialize)]
pub struct ProductForm {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}


impl Product {
    // ... (existing functions remain the same)
    /// Helper function to convert English digits to Persian
    fn to_persian_digits(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '0' => '۰', '1' => '۱', '2' => '۲', '3' => '۳', '4' => '۴',
                '5' => '۵', '6' => '۶', '7' => '۷', '8' => '۸', '9' => '۹',
                _ => c,
            })
            .collect()
    }

    /// Formats the price for display in a Persian format (e.g., "۴۲,۰۰۰ تومان")
    pub fn formatted_price(&self) -> String {
        // For simplicity, we'll format without comma separators for now.
        let price_str = format!("{:.0}", self.price);
        format!("{} تومان", Self::to_persian_digits(&price_str))
    }

    /// Provides a color class based on stock level
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