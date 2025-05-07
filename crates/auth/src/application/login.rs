//! Use-case for login a user.

use std::marker::PhantomData;

use common_core::UseCase;

use crate::domain::auth::{Auth, AuthCredentials};
use crate::domain::port::AuthStore;
use crate::prelude::*;

/// Login use-case structure.
pub(crate) struct Login<Store> {
    /// Phantom data used to use the `Store` parameter in the struct.
    _marker: PhantomData<Store>,
}

impl<Store> Login<Store> {
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

impl<Store> UseCase for Login<Store>
where
    Store: AuthStore,
{
    type Args = (Auth<Store>, AuthCredentials);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (mut auth, credentials) = args;

        // Try to authenticate the user (i.e. check if user exists and password are correct)
        let user = auth.authenticate(credentials).await?;

        // Check if the user is verified
        if !user.is_email_confirmed() {
            return Err(Error::EmailNotConfirmed);
        }

        // Create the session for this user
        auth.login(&user.id).await
    }
}
