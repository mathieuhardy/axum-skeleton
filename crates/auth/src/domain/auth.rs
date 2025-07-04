//! Authentication related entities.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::{event, Level};
use validator::Validate;

use security::password::Password;

use crate::domain::auth_user::AuthUser;
use crate::domain::error::Error;
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
    #[validate(nested)]
    pub password: Password,
}

/// Structure used to store all needed information for authentication.
/// This structure aims to be declared as argument of the HTTP endpoints.
#[derive(Debug)]
pub struct Auth {
    /// User information (optional as some endpoint will be called without a user logged in).
    pub user: Option<AuthUser>,

    /// Session store.
    pub session: Session,
}

impl Auth {
    /// Key used to store the user information in the session.
    pub const KEY: &'static str = "auth_user";

    /// Get the user information from the session.
    ///
    /// # Returns
    /// Optional user information.
    pub fn user(&self) -> &Option<AuthUser> {
        &self.user
    }

    /// Get the user information from the session as Result.
    ///
    /// # Returns
    /// Result containing the user information if found, or an error.
    pub fn try_user(&self) -> ApiResult<AuthUser> {
        self.user.as_ref().ok_or(Error::UserNotFound).cloned()
    }

    /// Try to authenticate a user by checking its credentials.
    ///
    /// # Arguments
    /// * `user`: User obtained from the database.
    /// * `credentials`: User credentials to check.
    ///
    /// # Returns
    /// Result containing the user information if found, or an error.
    pub async fn authenticate(
        &mut self,
        user: &AuthUser,
        credentials: &AuthCredentials,
    ) -> ApiResult<AuthUser> {
        match credentials.password.matches(&user.password).await {
            Ok(true) => Ok(user.clone()),

            Ok(false) => {
                event!(Level::ERROR, "Invalid credentials");
                Err(Error::Unauthorized)
            }

            _ => Err(Error::Unauthorized),
        }
    }

    /// Creates a session for the user.
    ///
    /// # Arguments
    /// * `user`: User obtained from the database.
    ///
    /// # Returns
    /// Result indicating success or failure.
    pub async fn login(&mut self, user: &AuthUser) -> ApiResult<()> {
        let auth_user = Some(user.clone());

        if self.user.is_none() {
            // Session-fixation
            self.session.cycle_id().await?;
        }

        self.session.insert(Self::KEY, auth_user.clone()).await?;

        self.user = auth_user;

        event!(Level::INFO, "Successfully logged in as {:?}", self.user);

        Ok(())
    }

    /// Deletes the current session.
    ///
    /// # Returns
    /// Result indicating success or failure.
    pub async fn logout(&mut self) -> ApiResult<()> {
        let user = self.user.take();

        self.session.flush().await?;

        event!(Level::INFO, "{:?} successfully logged out", user);

        Ok(())
    }
}

/// Checks if the user is authenticated. If not, it returns a 401 Unauthorized response.
///
/// # Arguments
/// * `auth`: Authentication object containing user information.
/// * `request`: HTTP request.
/// * `next`: Next middleware in the chain.
///
/// # Returns
/// Result containing the next response or a 401 Unauthorized response.
pub async fn require_authentication(
    auth: Auth,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if auth.user().is_some() {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Macro used to create a middleware that checks if the user is authenticated.
#[macro_export]
macro_rules! require_authentication {
    ($state: ident) => {
        axum::middleware::from_fn_with_state($state, require_authentication)
    };
}

#[cfg(test)]
mod tests {
    use test_utils::rand::*;

    use security::password::{set_checks, Checks};

    use super::*;

    #[tokio::test]
    async fn test_credentials_validation_email() -> Result<(), Box<dyn std::error::Error>> {
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
            email: random_string(),
            password: random_password(),
        };

        assert!(credentials.validate().is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_credentials_validation_password() -> Result<(), Box<dyn std::error::Error>> {
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
            password: Password::default(),
        };

        assert!(credentials.validate().is_err());

        Ok(())
    }
}
