//! Use-case for logout a user.

use common_core::UseCase;

use crate::domain::auth::Auth;
use crate::prelude::*;

/// Logout use-case structure.
pub struct Logout {}

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
    type Args = Auth;
    type Output = ();
    type Error = Error;

    async fn handle(&self, mut auth: Self::Args) -> Result<Self::Output, Self::Error> {
        auth.logout().await
    }
}
