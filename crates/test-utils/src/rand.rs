//! Utilit functions used to generate random values to be used when testing features of this
//! application.

use uuid::Uuid;

use security::password::Password;

/// Generates a random `Uuid`.
///
/// # Returns
/// A random `Uuid` value.
#[inline(always)]
pub fn random_id() -> Uuid {
    Uuid::new_v4()
}

/// Generates a random `String`.
///
/// # Returns
/// A random `String` value.
#[inline(always)]
pub fn random_string() -> String {
    random_id().to_string()
}

/// Generates a random email as `String`.
///
/// # Returns
/// A random `String` email value.
#[inline(always)]
pub fn random_email() -> String {
    format!("{}@{}.com", random_string(), random_string())
}

/// Generates a random password as `String`.
///
/// # Returns
/// A random `String` password value.
#[inline(always)]
pub fn random_password() -> Password {
    Password::from(format!("A0#{}", random_string()))
}
