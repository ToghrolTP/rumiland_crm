use crate::models::{Product, User};
use askama::Template;

/// Product catalog page template
#[derive(Template)]
#[template(path = "catalog.html")]
pub struct CatalogTemplate {
    pub products: Vec<Product>,
    pub current_user: Option<User>,
    pub active_page: &'static str,
    pub flash_message: Option<String>, // Add this field
}

/// Add Product page template
#[derive(Template)]
#[template(path = "add_product.html")]
pub struct AddProductTemplate {
    pub current_user: Option<User>,
    pub active_page: &'static str,
}