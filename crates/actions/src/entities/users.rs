use validator::Validate;

use database::models::users::{UserData, UserRole};

use crate::prelude::*;
use crate::validators::password::validate_password;

/// Structure used by HTTP endpoint to query a modification in  the database.
/// This structure is not expected to be used directly in queries. It must be converted first to a
/// `UserData`.
#[derive(Clone, Default, Debug, Deserialize, Serialize, Validate)]
pub struct UserRequest {
    /// See `User::sid`.
    pub id: Option<Uuid>,

    /// See `User::first_name`.
    #[validate(length(min = 1))]
    pub first_name: Option<String>,

    /// See `User::last_name`.
    #[validate(length(min = 1))]
    pub last_name: Option<String>,

    /// See `User::email`.
    #[validate(email)]
    pub email: Option<String>,

    /// See `User::role`.
    pub role: Option<UserRole>,

    /// See `User::password`.
    #[validate(custom = "validate_password")]
    pub password: Option<String>,
}

impl From<UserRequest> for UserData {
    fn from(request: UserRequest) -> Self {
        // Don't copy the password field from request. Keep this field empty by default as it must
        // be hashed before written to database.
        Self {
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            email: request.email.clone(),
            role: request.role.clone(),
            password: None,
        }
    }
}

impl From<&UserRequest> for UserData {
    fn from(request: &UserRequest) -> Self {
        (*request).clone().into()
    }
}

/// Structure provided to update the user's password
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PasswordUpdateRequest {
    /// Current password of the user. Not validated as it will be simply compared with the entry in
    /// database before updating.
    pub current: String,

    /// New password to be set in database.
    #[validate(custom = "validate_password")]
    pub new: String,
}
