//! Use-case for logout a user.

use tracing::{event, Level};

use common_core::UseCase;

use crate::domain::auth_backend::AuthSession;
use crate::prelude::*;

/// Logout use-case structure.
pub struct Logout;

impl Logout {
    /// Creates a `Logout` use-case instance.
    ///
    /// # Returns
    /// A `Logout` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl UseCase for Logout {
    type Args = AuthSession;
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
