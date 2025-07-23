use askama::Template;
use crate::models::{Customer, User};

/// Customer list page template
#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    pub customers: Vec<Customer>,
    pub active_page: &'static str,
    pub current_user: Option<User>,
}

/// Add customer page template
#[derive(Template)]
#[template(path = "add.html")]
pub struct AddTemplate {
    pub active_page: &'static str,
    pub current_user: Option<User>,
}

/// Customer detail page template
#[derive(Template)]
#[template(path = "detail.html")]
pub struct DetailTemplate {
    pub customer: Customer,
    pub active_page: &'static str,
    pub current_user: Option<User>,
}

/// Edit customer page template
#[derive(Template)]
#[template(path = "edit.html")]
pub struct EditTemplate {
    pub customer: Customer,
    pub active_page: &'static str,
    pub current_user: Option<User>,
}