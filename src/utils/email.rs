use crate::error::{AppError, AppResult};

/// Validate email format
pub fn validate_email(email: &str) -> AppResult<String> {
    let email = email.trim().to_lowercase();

    // Check if email is empty
    if email.is_empty() {
        return Err(AppError::BadRequest(
            "آدرس ایمیل نمی‌تواند خالی باشد".to_string()
        ));
    }

    // Check for @ symbol
    if !email.contains('@') {
        return Err(AppError::BadRequest(
            "آدرس ایمیل باید شامل @ باشد".to_string()
        ));
    }

    // Check for domain
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(AppError::BadRequest(
            "فرمت ایمیل نامعتبر است".to_string()
        ));
    }

    let local = parts[0];
    let domain = parts[1];

    // Basic validation
    if local.is_empty() {
        return Err(AppError::BadRequest(
            "نام کاربری ایمیل نمی‌تواند خالی باشد".to_string()
        ));
    }

    if !domain.contains('.') {
        return Err(AppError::BadRequest(
            "آدرس ایمیل باید شامل نام دامنه معتبر باشد".to_string()
        ));
    }

    // Check for spaces
    if email.contains(' ') {
        return Err(AppError::BadRequest(
            "آدرس ایمیل نباید شامل فاصله باشد".to_string()
        ));
    }

    // Check minimum length
    if email.len() < 5 {  // a@b.c is minimum valid
        return Err(AppError::BadRequest(
            "آدرس ایمیل خیلی کوتاه است".to_string()
        ));
    }

    // Check for consecutive dots
    if email.contains("..") {
        return Err(AppError::BadRequest(
            "آدرس ایمیل نباید شامل نقطه‌های متوالی باشد".to_string()
        ));
    }

    // Check if starts or ends with dot
    if local.starts_with('.') || local.ends_with('.') || domain.starts_with('.') || domain.ends_with('.') {
        return Err(AppError::BadRequest(
            "آدرس ایمیل نباید با نقطه شروع یا تمام شود".to_string()
        ));
    }

    // Check for common typos and suggest corrections
    if let Some(suggestion) = suggest_email_correction(&email) {
        return Err(AppError::BadRequest(
            format!("آیا منظورتان {} است؟", suggestion)
        ));
    }

    Ok(email)
}

/// Suggest corrections for common email typos
fn suggest_email_correction(email: &str) -> Option<String> {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return None;
    }

    let local = parts[0];
    let domain = parts[1];

    // Common domain typos
    let corrections = vec![
        ("gmial.com", "gmail.com"),
        ("gmai.com", "gmail.com"),
        ("gmil.com", "gmail.com"),
        ("gmail.co", "gmail.com"),
        ("gmail.con", "gmail.com"),
        ("yahoo.co", "yahoo.com"),
        ("yahoo.con", "yahoo.com"),
        ("yaho.com", "yahoo.com"),
        ("hotmial.com", "hotmail.com"),
        ("hotmai.com", "hotmail.com"),
        ("outlok.com", "outlook.com"),
    ];

    for (typo, correct) in corrections {
        if domain == typo {
            return Some(format!("{}@{}", local, correct));
        }
    }

    None
}

/// Normalize email address
pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        // Valid emails
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user.name@company.co.uk").is_ok());
        assert!(validate_email("info@domain.ir").is_ok());

        // Invalid emails
        assert!(validate_email("").is_err());
        assert!(validate_email("notanemail").is_err());
        assert!(validate_email("missing@domain").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
    }

    #[test]
    fn test_normalize_email() {
        assert_eq!(normalize_email("  TEST@EXAMPLE.COM  "), "test@example.com");
        assert_eq!(normalize_email("User@Domain.COM"), "user@domain.com");
    }
}
