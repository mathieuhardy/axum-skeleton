//! This file contains password related variables and functions used for the
//! verification of the validity of a password provided by the user.

use std::sync::{Arc, Mutex};

use utils::password::Checks;

use crate::prelude::*;

lazy_static::lazy_static! {
    /// Static variable used to store the checks to be made.
    static ref PASSWORD_CHECKS: Arc<Mutex<Checks>> = Arc::new(Mutex::new(Checks::default()));
}

/// Sets the checks to be done for the password verification.
///
/// # Arguments
/// * `checks` - Structure containing all checks to be made.
pub fn set_checks(checks: Checks) {
    if let Ok(mut value) = PASSWORD_CHECKS.lock() {
        *value = checks;

        event!(Level::INFO, "✱ Password checks created");
        event!(Level::TRACE, "{:#?}", *value);
    } else {
        event!(Level::ERROR, "Cannot apply password checks");
    }
}

/// Gets the checks to be done for the password verification.
///
/// # Returns
/// Checks structure or an error.
pub fn checks() -> Res<Checks> {
    if let Ok(value) = PASSWORD_CHECKS.lock() {
        Ok((*value).clone())
    } else {
        Err(Error::PasswordChecksAccess)
    }
}
