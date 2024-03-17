//! Hashing functions mostly used for password storage in database.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordVerifier, SaltString};
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

/// Verifies that a password matches a hashed password.
///
/// # Arguments
/// * `password` - Password to be checked.
/// * `hashed` - Hashed password to compare with.
///
/// # Returns
/// A result containing the bool that tell if the passwords matches.
pub fn verify(password: &str, hashed: String) -> Res<bool> {
    let hash = PasswordHash::new(&hashed).map_err(|e| Error::Hashing(e.to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok())
}
