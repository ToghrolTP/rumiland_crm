use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::AppResult;

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> AppResult<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    Ok(verify(password, hash)?)
}