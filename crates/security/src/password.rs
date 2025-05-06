//! Structures and utilities related to password management.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tracing::{event, Level};
use validator::{Validate, ValidationError};

use utils::hashing::hash_string;

use crate::prelude::*;

/// Variable used to store all checks to be performed on a password to ensure that it's valid.
static PASSWORD_CHECKS: OnceLock<Checks> = OnceLock::new();

/// Password structure (to avoid using plain strings in the application).
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Validate)]
#[serde(transparent)]
pub struct Password {
    /// Password value.
    #[validate(custom(function = "validate_password"))]
    value: String,
}

impl Password {
    /// Returns the password as array of bytes.
    ///
    /// # Returns
    /// A slice of bytes representing the password.
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    /// Returns the password as a slice of string.
    ///
    /// # Returns
    /// A slice of string representing the password.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Returns a hashed version of the password.
    ///
    /// # Returns
    /// A result containing the hashed password.
    pub fn hashed(&self) -> ApiResult<Self> {
        Ok(Password::from(hash_string(self.as_str())?))
    }

    /// Checks that the password verifies the checks.
    ///
    /// # Arguments
    /// * `checks` - List of checks to be verified.
    ///
    /// # Returns
    /// true if the password matches, false otherwise.
    pub fn verify_checks(&self, mut checks: Checks) -> bool {
        let length = self.value.len() as u32;

        if length < checks.min_length {
            return false;
        }

        if let Some(max_length) = checks.max_length {
            if length > max_length {
                return false;
            }
        }

        let expect_spaces = checks.spaces;

        for c in self.value.chars() {
            if checks.digit && c.is_numeric() {
                checks.digit = false;
            } else if checks.lowercase && c.is_lowercase() {
                checks.lowercase = false;
            } else if checks.uppercase && c.is_uppercase() {
                checks.uppercase = false;
            } else if checks.special && !c.is_alphanumeric() {
                checks.special = false;
            } else if expect_spaces && c.is_whitespace() {
                checks.spaces = false;
            } else if !expect_spaces && c.is_whitespace() {
                checks.spaces = true;
            }
        }

        checks.is_ok()
    }

    /// Verifies that a password matches a hashed password.
    ///
    /// # Arguments
    /// * `hashed` - Hashed password to compare with.
    ///
    /// # Returns
    /// A result containing the bool that tell if the passwords matches.
    pub async fn matches(&self, hashed: &Self) -> ApiResult<bool> {
        Ok(utils::hashing::verify(self.as_str(), hashed.as_str()).await?)
    }
}

impl std::convert::From<&str> for Password {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl std::convert::From<String> for Password {
    fn from(value: String) -> Self {
        Self { value }
    }
}

/// Structure used to store fields to be checked for a password.
#[derive(Clone, Debug, Default)]
pub struct Checks {
    /// Check presence of digit if true.
    pub digit: bool,

    /// Check presence of lowercase if true.
    pub lowercase: bool,

    /// Check presence of uppercase if true.
    pub uppercase: bool,

    /// Check presence of special character if true.
    pub special: bool,

    /// Check presence of whitespaces if true.
    pub spaces: bool,

    /// Check minimum length of the password.
    pub min_length: u32,

    /// Check maximum length of the password.
    pub max_length: Option<u32>,
}

impl Checks {
    /// Checks that all conditions are fulfilled.
    ///
    /// # Returns
    /// true if all checks are fulfilled, false otherwise.
    pub fn is_ok(&self) -> bool {
        !self.digit && !self.lowercase && !self.uppercase && !self.special && !self.spaces
    }
}

/// Validate a password according to application rules (use by validator crate).
///
/// # Arguments
/// * `password` - Password to be checked.
///
/// # Returns
/// No output if the password is correct, an error otherwise.
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let checks = PASSWORD_CHECKS
        .get()
        .ok_or(ValidationError::new("cannot_access_checks"))?
        .to_owned();

    if Password::from(password).verify_checks(checks) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}

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
