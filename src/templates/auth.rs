use askama::Template;

/// Login page template
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
    pub flash_message: Option<String>,
}
