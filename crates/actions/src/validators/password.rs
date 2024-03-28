use std::sync::OnceLock;
use validator::ValidationError;

use crate::prelude::*;
use utils::password::Checks;

/// Variable used to store all checcks to be performed on a password to ensure that it's valid.
static PASSWORD_CHECKS: OnceLock<Checks> = OnceLock::new();

/// Sets the checks to be done for the password verification.
///
/// # Arguments
/// * `checks` - Structure containing all checks to be made.
pub fn set_checks(checks: Checks) {
    if PASSWORD_CHECKS.set(checks.clone()).is_ok() {
        //event!(Level::INFO, "✱ Password checks created");
        //event!(Level::TRACE, "{:#?}", checks);
    } else {
        //event!(Level::ERROR, "Cannot apply password checks");
        //TODO
    }
}

/// Validate a password accoring to application rules.
///
/// # Arguments
/// * `password` - Password to be checked.
///
/// # Returns
/// No output if the password is correct, an error otherwise.
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let checks = PASSWORD_CHECKS
        .get()
        .ok_or(Error::PasswordChecksAccess)
        .map_err(|_| ValidationError::new("cannot_access_checks"))?
        .to_owned();

    if utils::password::verify(password, checks) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}
