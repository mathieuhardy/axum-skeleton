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
