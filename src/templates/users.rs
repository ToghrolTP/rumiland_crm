use askama::Template;
use crate::models::User;

/// User list page template
#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>,
    pub current_user: Option<User>,
    pub active_page: &'static str,
}

/// Add user page template
#[derive(Template)]
#[template(path = "add_user.html")]
pub struct AddUserTemplate {
    pub current_user: Option<User>,
    pub active_page: &'static str,
}