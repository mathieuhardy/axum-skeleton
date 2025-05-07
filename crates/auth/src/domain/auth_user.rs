//! Authentication user related entities.

use chrono::{DateTime, Utc};
use validator::Validate;

use security::password::Password;

use crate::prelude::*;

/// List of users roles.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthUserRole {
    /// User with all privileges.
    Admin,

    /// Normal user.
    Normal,

    /// User with very limited privileges.
    #[default]
    Guest,
}

/// Needed field to handle authentication of a user.
#[derive(Clone, Default, PartialEq, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct AuthUser {
    /// Unique record identifier.
    pub id: Uuid,

    /// User's email.
    pub email: String,

    /// Role of the user.
    pub role: AuthUserRole,

    /// Password of the user (hashed of course).
    #[debug(skip)]
    pub password: Password,

    /// Email confirmed.
    pub email_confirmed: bool,
}

impl AuthUser {
    /// Checks if the user is an admin.
    ///
    /// # Returns
    /// `true` if the user is an admin, `false` otherwise.
    pub fn is_admin(&self) -> bool {
        self.role == AuthUserRole::Admin
    }

    /// Checks if the user is the same as the ID provided.
    ///
    /// # Arguments
    /// * `id` - A user identifier.
    ///
    /// # Returns
    /// `true` if the user matches the ID provided.
    pub fn is(&self, id: &Uuid) -> bool {
        self.id == *id
    }

    /// Checks if the user has confirmed its email.
    ///
    /// # Returns
    /// `true` if the user has confirmed its email.
    pub fn is_email_confirmed(&self) -> bool {
        self.email_confirmed
    }

    /// Returns a vector of u8 representing the hash of the user.
    ///
    /// # Returns
    /// A slice of u8 representing the hash of the user.
    pub fn hash(&self) -> &[u8] {
        // We're using the password as a unique hash so that if the user changes its password,
        // the session is invalidated.
        self.password.as_bytes()
    }
}

/// Needed field to handle authentication of a user.
#[derive(Clone, Default, PartialEq, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct AuthUserConfirmation {
    /// Unique record identifier.
    pub id: Uuid,

    /// User's ID.
    pub user_id: Uuid,

    /// Date of expiration of the token.
    pub expires_at: DateTime<Utc>,
}

impl AuthUserConfirmation {
    /// Checks if the confirmation is expired.
    ///
    /// # Returns
    /// `true` if the confirmation is expired, `false` otherwise.
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use test_utils::rand::*;

    use super::*;

    #[tokio::test]
    async fn test_auth_user_is_admin() -> Result<(), Box<dyn std::error::Error>> {
        let auth_user = AuthUser {
            id: random_id(),
            role: AuthUserRole::Admin,
            ..Default::default()
        };

        assert!(auth_user.is_admin());

        let auth_user = AuthUser {
            id: random_id(),
            role: AuthUserRole::Normal,
            ..Default::default()
        };

        assert!(!auth_user.is_admin());

        let auth_user = AuthUser {
            id: random_id(),
            role: AuthUserRole::Guest,
            ..Default::default()
        };

        assert!(!auth_user.is_admin());

        Ok(())
    }

    #[tokio::test]
    async fn test_auth_user_is_equal_to() -> Result<(), Box<dyn std::error::Error>> {
        let auth_user = AuthUser {
            id: random_id(),
            role: AuthUserRole::Admin,
            ..Default::default()
        };

        assert!(auth_user.is(&auth_user.id));
        assert!(!auth_user.is(&random_id()));

        Ok(())
    }

    #[tokio::test]
    async fn test_auth_user_is_email_confirmed() -> Result<(), Box<dyn std::error::Error>> {
        let auth_user = AuthUser::default();
        assert!(!auth_user.is_email_confirmed());

        let auth_user = AuthUser {
            email_confirmed: true,
            ..Default::default()
        };

        assert!(auth_user.is_email_confirmed());

        Ok(())
    }

    #[tokio::test]
    async fn test_auth_user_hash() -> Result<(), Box<dyn std::error::Error>> {
        let auth_user = AuthUser {
            id: random_id(),
            role: AuthUserRole::Admin,
            ..Default::default()
        };

        let bytes = auth_user.password.as_bytes();

        assert_eq!(auth_user.hash(), bytes);

        Ok(())
    }
}
