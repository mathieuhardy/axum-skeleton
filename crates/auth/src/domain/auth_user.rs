//! Authentication user related entities.

use axum_login::AuthUser as AxumAuthUser;
use validator::Validate;

use security::password::validate_password;

use crate::prelude::*;

/// Structure used to store the credentials that must be provided by a user to check it's
/// existence. This should match a form displayed to the user where he can enter his email and
/// password.
#[derive(Clone, Deserialize, Serialize, Validate, derive_more::Debug)]
pub struct AuthCredentials {
    /// Email used during authentication.
    #[validate(email)]
    pub email: String,

    /// Password used during authentication.
    #[debug(skip)]
    #[validate(custom = "validate_password")]
    pub password: String,
}

/// List of users roles.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
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
    pub password: String,
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
}

impl AxumAuthUser for AuthUser {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // We're using the password as a unique hash so that if the user changes its password,
        // the session is invalidated.
        self.password.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use test_utils::rand::*;

    use security::password::{set_checks, Checks};

    use super::*;

    #[tokio::test]
    async fn test_credentials_validation_email() -> Result<(), Box<dyn std::error::Error>> {
        set_checks(Checks::default());

        let credentials = AuthCredentials {
            email: random_email(),
            password: random_password(),
        };

        assert!(credentials.validate().is_ok());

        let credentials = AuthCredentials {
            email: random_string(),
            password: random_password(),
        };

        assert!(credentials.validate().is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_credentials_validation_pasword() -> Result<(), Box<dyn std::error::Error>> {
        set_checks(Checks {
            min_length: 8,
            ..Checks::default()
        });

        let credentials = AuthCredentials {
            email: random_email(),
            password: random_password(),
        };

        assert!(credentials.validate().is_ok());

        let credentials = AuthCredentials {
            email: random_email(),
            password: String::new(),
        };

        assert!(credentials.validate().is_err());

        Ok(())
    }

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
    async fn test_auth_user_id() -> Result<(), Box<dyn std::error::Error>> {
        let auth_user = AuthUser {
            id: random_id(),
            role: AuthUserRole::Admin,
            ..Default::default()
        };

        assert_eq!(auth_user.id(), auth_user.id);

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

        assert_eq!(auth_user.session_auth_hash(), bytes);

        Ok(())
    }
}
