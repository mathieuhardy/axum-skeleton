//! Hashing functions mostly used for password storage in database.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

use crate::prelude::*;

/// Hash a given password using Argon2id algorithm.
///
/// # Arguments
/// * `password` - Input string to be hashed.
///
/// # Returns
/// The hashed password or an error.
pub fn hash_password(password: &str) -> Res<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Error::Hashing(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

/// Creates a regex from toggles that can be used to verify passwords.
///
/// # Arguments
/// * `digit` - If true, at least one digit is required.
/// * `lowercase` - If true, at least one lowercase character is required.
/// * `uppercase` - If true, at least one uppercase character is required.
/// * `special` - If true, at least one special character is required.
/// * `spaces` - If false, no spaces are allowed.
/// * `min_length` - Minimum length of the password.
/// * `max_length` - Maximum length of the password (optional).
///
/// # Returns
/// The regular expression built as String.
pub fn password_pattern(
    digit: bool,
    lowercase: bool,
    uppercase: bool,
    special: bool,
    spaces: bool,
    min_length: u32,
    max_length: Option<u32>,
) -> String {
    format!(
        "^{}{}{}{}{}.{{{},{}}}$",
        digit.then_some("(?=.*[0-9])").unwrap_or_default(),
        lowercase.then_some("(?=.*[a-z])").unwrap_or_default(),
        uppercase.then_some("(?=.*[A-Z])").unwrap_or_default(),
        special.then_some("(?=.*[^a-zA-Z0-9])").unwrap_or_default(),
        spaces.then_some("").unwrap_or("(?!.* )"),
        min_length,
        max_length
            .map(|value| format!("{}", value))
            .unwrap_or_default()
    )
}
