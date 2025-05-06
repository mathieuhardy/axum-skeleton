//! Hashing functions mostly used for password storage in database.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHasher};

use crate::error::*;

/// Hash a given string using Argon2id algorithm.
///
/// # Arguments
/// * `value` - Input string to be hashed.
///
/// # Returns
/// The hashed string or an error.
pub fn hash_string(value: &str) -> Res<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(value.as_bytes(), &salt)
        .map_err(|e| Error::Hashing(e.to_string()))?
        .to_string();

    Ok(hash)
}

/// Verifies that a string matches a hashed string.
///
/// # Arguments
/// * `value` - String to be checked.
/// * `hashed` - Hashed string to compare with.
///
/// # Returns
/// A result containing the bool that tell if the strings matches.
pub async fn verify(value: &str, hashed: &str) -> Res<bool> {
    let value = value.to_owned();
    let hashed = hashed.to_owned();

    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(hashed.as_str()).map_err(|e| Error::Hashing(e.to_string()))?;

        Ok(Argon2::default()
            .verify_password(value.as_bytes(), &hash)
            .is_ok())
    })
    .await?
}
