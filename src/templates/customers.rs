use crate::models::{Customer, Transaction, User};
use askama::Template;

/// Customer list page template
#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    pub customers: Vec<Customer>,
    pub active_page: &'static str,
    pub current_user: Option<User>,
    pub flash_message: Option<String>,
}

/// Add customer page template
#[derive(Template)]
#[template(path = "add.html")]
pub struct AddTemplate {
    pub active_page: &'static str,
    pub current_user: Option<User>,
    pub cities: Vec<crate::models::City>,
    pub methods: Vec<crate::models::SettlementMethod>,
    // pub batch_count: i32,
}

/// Customer detail page template
#[derive(Template)]
#[template(path = "detail.html")]
pub struct DetailTemplate {
    pub customer: Customer,
    pub transactions: Vec<Transaction>,
    pub active_page: &'static str,
    pub current_user: Option<User>,
    pub flash_message: Option<String>,
}

/// Edit customer page template
#[derive(Template)]
#[template(path = "edit.html")]
pub struct EditTemplate {
    pub customer: Customer,
    pub active_page: &'static str,
    pub current_user: Option<User>,
    pub cities: Vec<crate::models::City>,
    pub methods: Vec<crate::models::SettlementMethod>,
}
