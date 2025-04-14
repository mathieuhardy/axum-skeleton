//! Use-case for login a user.

use tracing::{event, Level};

use common_core::UseCase;

use crate::domain::auth_backend::{AuthCredentials, AuthSession};
use crate::prelude::*;

/// Login use-case structure.
pub struct Login;

impl Login {
    /// Creates a `Login` use-case instance.
    ///
    /// # Returns
    /// A `Login` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl UseCase for Login {
    type Args = (AuthSession, AuthCredentials);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (mut auth_session, credentials) = args;

        // Try to authenticate the user
        let user = match auth_session.authenticate(credentials.clone()).await {
            Ok(Some(user)) => user,

            Ok(None) => {
                event!(Level::ERROR, "Invalid credentials");

                return Err(Error::Unauthorized);
            }

            Err(_) => {
                return Err(Error::Unauthorized);
            }
        };

        // Create the session for this user
        if auth_session.login(&user).await.is_err() {
            return Err(Error::Internal);
        }

        event!(Level::INFO, "Successfully logged in as {}", user.email);

        Ok(())
    }
}

// TODO: tests
