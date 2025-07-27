use crate::error::{AppError, AppResult};

/// Phone number types in Iran
#[derive(Debug, PartialEq)]
pub enum PhoneType {
    Mobile,
    Landline,
}

/// Validate and normalize Iranian phone numbers (both mobile and landline)
pub fn normalize_phone_number(phone: &str) -> AppResult<String> {
    // Remove all non-digit characters
    let digits: String = phone.chars().filter(|c| c.is_digit(10)).collect();

    // Check various formats
    match digits.len() {
        // Standard format with 0 prefix (mobile or landline)
        11 => {
            if digits.starts_with("09") {
                // Mobile number
                Ok(digits)
            } else if digits.starts_with("0") {
                // Landline number
                Ok(digits)
            } else {
                Err(AppError::BadRequest(
                    "شماره تلفن نامعتبر است. شماره تلفن باید با 0 شروع شود".to_string(),
                ))
            }
        }
        // Mobile without 0 prefix
        10 => {
            if digits.starts_with("9") {
                // Add 0 prefix for mobile
                Ok(format!("0{}", digits))
            } else {
                Err(AppError::BadRequest(
                    "شماره موبایل نامعتبر است. شماره موبایل باید با 9 شروع شود".to_string(),
                ))
            }
        }
        // International format without + (mobile)
        12 => {
            if digits.starts_with("989") {
                // Convert to local format
                Ok(format!("0{}", &digits[2..]))
            } else {
                Err(AppError::BadRequest(
                    "فرمت شماره تلفن نامعتبر است".to_string(),
                ))
            }
        }
        // International format with full country code
        13 => {
            if digits.starts_with("989") {
                // Convert to local format (remove 98)
                Ok(format!("0{}", &digits[2..]))
            } else {
                Err(AppError::BadRequest(
                    "فرمت شماره تلفن نامعتبر است".to_string(),
                ))
            }
        }
        _ => Err(AppError::BadRequest(
            "شماره تلفن نامعتبر است. لطفاً یک شماره تلفن معتبر 11 رقمی وارد کنید".to_string(),
        )),
    }
}

/// Determine phone type
pub fn get_phone_type(phone: &str) -> Option<PhoneType> {
    let digits: String = phone.chars().filter(|c| c.is_digit(10)).collect();

    if digits.len() == 11 {
        if digits.starts_with("09") {
            Some(PhoneType::Mobile)
        } else if digits.starts_with("0") {
            Some(PhoneType::Landline)
        } else {
            None
        }
    } else {
        None
    }
}

/// Format phone number for display
pub fn format_phone_for_display(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_digit(10)).collect();

    match get_phone_type(&digits) {
        Some(PhoneType::Mobile) => {
            // Format: 0912 345 6789 (4-3-4)
            if digits.len() == 11 {
                format!(
                    "{} {} {}",
                    &digits[0..4],  // 0912
                    &digits[4..7],  // 345
                    &digits[7..11]  // 6789
                )
            } else {
                phone.to_string()
            }
        }
        Some(PhoneType::Landline) => {
            // Different formatting based on area code length
            if digits.len() == 11 {
                // Major cities with 3-digit area codes
                if digits.starts_with("021") ||  // Tehran
                   digits.starts_with("026") ||  // Karaj/Alborz
                   digits.starts_with("031") ||  // Isfahan
                   digits.starts_with("041") ||  // Tabriz
                   digits.starts_with("051") ||  // Mashhad
                   digits.starts_with("071") ||  // Shiraz
                   digits.starts_with("061") ||  // Ahvaz
                   digits.starts_with("034")
                {
                    // Kerman
                    // 3 digit area code: 021 4455 6677
                    format!(
                        "{} {} {}",
                        &digits[0..3],  // 021
                        &digits[3..7],  // 4455
                        &digits[7..11]  // 6677
                    )
                } else {
                    // Most cities - 4 digit area code: 0241 333 4444
                    format!(
                        "{} {} {}",
                        &digits[0..4],  // 0241
                        &digits[4..7],  // 333
                        &digits[7..11]  // 4444
                    )
                }
            } else {
                phone.to_string()
            }
        }
        None => phone.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_phone_number() {
        // Test mobile formats
        assert_eq!(
            normalize_phone_number("09123456789").unwrap(),
            "09123456789"
        );
        assert_eq!(normalize_phone_number("9123456789").unwrap(), "09123456789");
        assert_eq!(
            normalize_phone_number("0912 345 6789").unwrap(),
            "09123456789"
        );
        assert_eq!(
            normalize_phone_number("+989123456789").unwrap(),
            "09123456789"
        );
        assert_eq!(
            normalize_phone_number("989123456789").unwrap(),
            "09123456789"
        );

        // Test landline formats
        assert_eq!(
            normalize_phone_number("02144556677").unwrap(),
            "02144556677"
        );
        assert_eq!(
            normalize_phone_number("0241 333 4444").unwrap(),
            "02413334444"
        );

        // Test invalid formats
        assert!(normalize_phone_number("123456").is_err());
        assert!(normalize_phone_number("08123456789").is_err()); // Invalid mobile prefix
    }

    #[test]
    fn test_format_phone() {
        // Mobile numbers
        assert_eq!(format_phone_for_display("09123456789"), "0912 345 6789");
        assert_eq!(format_phone_for_display("09374749005"), "0937 474 9005");

        // Landline - Tehran
        assert_eq!(format_phone_for_display("02144556677"), "021 4455 6677");

        // Landline - Other cities
        assert_eq!(format_phone_for_display("02413334444"), "0241 333 4444");
        assert_eq!(format_phone_for_display("02435751742"), "0243 575 1742");

        // Major cities with 3-digit codes
        assert_eq!(format_phone_for_display("03133445566"), "031 3344 5566"); // Isfahan
        assert_eq!(format_phone_for_display("04133445566"), "041 3344 5566"); // Tabriz
    }
}
