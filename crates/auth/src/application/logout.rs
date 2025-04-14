//! Use-case for logout a user.

use axum_login::{AuthSession, AuthnBackend};
use std::marker::PhantomData;
use tracing::{event, Level};

use common_core::UseCase;

use crate::domain::port::AuthRepository;
use crate::prelude::*;

/// Logout use-case structure.
pub struct Logout<T> {
    /// Phantom data used to use the T parameter in the struct.
    _marker: PhantomData<T>,
}

impl<T> Logout<T> {
    /// Creates a `Logout` use-case instance.
    ///
    /// # Returns
    /// A `Logout` instance.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> UseCase for Logout<T>
where
    T: AuthnBackend + AuthRepository,
{
    type Args = AuthSession<T>;
    type Output = ();
    type Error = Error;

    async fn handle(&self, mut auth_session: Self::Args) -> Result<Self::Output, Self::Error> {
        match auth_session.logout().await {
            Ok(_) => {
                event!(Level::INFO, "Successfully logged out");

                Ok(())
            }

            Err(_) => Err(Error::Internal),
        }
    }
}

// TODO: tests
