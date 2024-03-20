//! This file contains password related variables and functions used for the
//! verification of the validity of a password provided by the user.

use std::sync::OnceLock;

use utils::password::Checks;

use crate::prelude::*;

static PASSWORD_CHECKS: OnceLock<Checks> = OnceLock::new();

/// Sets the checks to be done for the password verification.
///
/// # Arguments
/// * `checks` - Structure containing all checks to be made.
pub fn set_checks(checks: Checks) {
    if PASSWORD_CHECKS.set(checks.clone()).is_ok() {
        event!(Level::INFO, "✱ Password checks created");
        event!(Level::TRACE, "{:#?}", checks);
    } else {
        event!(Level::ERROR, "Cannot apply password checks");
    }
}

/// Gets the checks to be done for the password verification.
///
/// # Returns
/// Checks structure or an error.
pub fn checks() -> Res<Checks> {
    PASSWORD_CHECKS
        .get()
        .ok_or(Error::PasswordChecksAccess)
        .cloned()
}
