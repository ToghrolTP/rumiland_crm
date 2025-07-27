use crate::models::User;
use askama::Template;

/// User list page template
#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>,
    pub current_user: Option<User>,
    pub active_page: &'static str,
    pub flash_message: Option<String>,
}

/// Add user page template
#[derive(Template)]
#[template(path = "add_user.html")]
pub struct AddUserTemplate {
    pub current_user: Option<User>,
    pub active_page: &'static str,
}
