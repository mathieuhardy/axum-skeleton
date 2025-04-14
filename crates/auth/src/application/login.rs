//! Use-case for login a user.

use axum_login::{AuthSession, AuthnBackend};
use std::marker::PhantomData;
use tracing::{event, Level};

use common_core::UseCase;

use crate::domain::port::AuthRepository;
use crate::prelude::*;

/// Login use-case structure.
pub struct Login<T> {
    /// Phantom data used to use the T parameter in the struct.
    _marker: PhantomData<T>,
}

impl<T> Login<T> {
    /// Creates a `Login` use-case instance.
    ///
    /// # Returns
    /// A `Login` instance.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> UseCase for Login<T>
where
    T: AuthnBackend + AuthRepository,
{
    type Args = (AuthSession<T>, T::Credentials);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (mut auth_session, credentials) = args;

        // Try to authenticate the user
        let user = match auth_session.authenticate(credentials).await {
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

        event!(Level::INFO, "Successfully logged in as {:?}", user);

        Ok(())
    }
}

// TODO: tests
