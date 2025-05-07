//! User data structures.

use chrono::{DateTime, Utc};
use validator::Validate;

use auth::AuthUserConfirmation;
use security::password::Password;

use crate::prelude::*;

/// List of users roles.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    /// User with all privileges.
    Admin,

    /// Normal user.
    Normal,

    /// User with very limited privileges.
    #[default]
    Guest,
}

/// Structure that list all filters available for querying database.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserFilters {
    /// First name of the user (or None).
    pub first_name: Option<String>,

    /// Last name of the user (or None).
    pub last_name: Option<String>,

    /// Email of the user (or None).
    pub email: Option<String>,

    /// Role of the user (or None).
    pub role: Option<UserRole>,
}

/// Mirrors the `users`'s' table.
#[derive(Clone, Default, PartialEq, Deserialize, Serialize, derive_more::Debug)]
pub struct User {
    /// Unique record identifier.
    pub id: Uuid,

    /// First name of the user.
    pub first_name: String,

    /// Last name of the user.
    pub last_name: String,

    /// Email of the user.
    pub email: String,

    /// Role of the user.
    pub role: UserRole,

    /// Password of the user (hashed of course).
    #[debug(skip)]
    pub password: Password,

    /// Date of record's creation.
    pub created_at: DateTime<Utc>,

    /// Date of record's last update.
    pub updated_at: DateTime<Utc>,

    /// User confirmation information.
    pub pending_confirmation: Option<AuthUserConfirmation>,
}

impl User {
    /// Checks if the user has confirmed its email.
    pub fn is_email_confirmed(&self) -> bool {
        self.pending_confirmation.is_none()
    }
}

/// Data structure passed to database queries when inserting or updating entries.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct UserData {
    /// See `User::first_name`.
    pub first_name: Option<String>,

    /// See `User::last_name`.
    pub last_name: Option<String>,

    /// See `User::email`.
    pub email: String,

    /// See `User::role`.
    pub role: UserRole,

    /// See `User::password`.
    pub password: Password,
}

/// Structure used by HTTP endpoint to query an update in the database.
/// This structure is not expected to be used directly in queries. It must be converted first to a
/// `UserData`.
#[derive(Clone, Default, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct CreateUserRequest {
    /// See `User::first_name`.
    #[validate(length(min = 1))]
    pub first_name: String,

    /// See `User::last_name`.
    #[validate(length(min = 1))]
    pub last_name: String,

    /// See `User::email`.
    #[validate(email)]
    pub email: String,

    /// See `User::role`.
    pub role: UserRole,

    /// See `User::password`.
    #[debug(skip)]
    #[validate(nested)]
    pub password: Password,
}

impl From<CreateUserRequest> for UserData {
    fn from(request: CreateUserRequest) -> Self {
        // Don't copy the password field from request. Keep this field empty by default as it must
        // be hashed before written to database.
        Self {
            first_name: if !request.first_name.is_empty() {
                Some(request.first_name.clone())
            } else {
                None
            },
            last_name: if !request.last_name.is_empty() {
                Some(request.last_name.clone())
            } else {
                None
            },
            email: request.email.clone(),
            role: request.role.clone(),
            password: request.password.clone(),
        }
    }
}

/// Structure used by HTTP endpoint to query an update in the database.
/// This structure is not expected to be used directly in queries. It must be converted first to a
/// `UserData`.
#[derive(Clone, Default, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct UpdateUserRequest {
    /// See `User::first_name`.
    #[validate(length(min = 1))]
    pub first_name: String,

    /// See `User::last_name`.
    #[validate(length(min = 1))]
    pub last_name: String,

    /// See `User::email`.
    #[validate(email)]
    pub email: String,

    /// See `User::role`.
    pub role: UserRole,
}

impl From<UpdateUserRequest> for UserData {
    fn from(request: UpdateUserRequest) -> Self {
        // Don't copy the password field from request. Keep this field empty by default as it must
        // be hashed before written to database.
        Self {
            first_name: if !request.first_name.is_empty() {
                Some(request.first_name.clone())
            } else {
                None
            },
            last_name: if !request.last_name.is_empty() {
                Some(request.last_name.clone())
            } else {
                None
            },
            email: request.email.clone(),
            role: request.role.clone(),
            password: Password::default(),
        }
    }
}

/// Structure used by HTTP endpoint to query a modification in the database.
/// This structure is not expected to be used directly in queries. It must be converted first to a
/// `UserData`.
#[derive(Clone, Default, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct UpsertUserRequest {
    /// See `User::id`.
    pub user_id: Option<Uuid>,

    /// See `User::password`.
    #[debug(skip)]
    #[validate(nested)]
    pub password: Option<Password>,

    /// Data from `UserRequest`
    #[serde(flatten)]
    #[validate(nested)]
    pub user: UpdateUserRequest,
}

impl From<UpsertUserRequest> for UserData {
    fn from(request: UpsertUserRequest) -> Self {
        Self {
            first_name: if !request.user.first_name.is_empty() {
                Some(request.user.first_name.clone())
            } else {
                None
            },
            last_name: if !request.user.last_name.is_empty() {
                Some(request.user.last_name.clone())
            } else {
                None
            },
            email: request.user.email.clone(),
            role: request.user.role.clone(),
            password: request.password.unwrap_or_default(),
        }
    }
}

/// Structure provided to update the user's password
#[derive(Default, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct PasswordUpdateRequest {
    /// Current password of the user. Not validated as it will be simply compared with the entry in
    /// database before updating.
    pub current: Password,

    /// New password to be set in database.
    #[debug(skip)]
    #[validate(nested)]
    pub new: Password,
}
