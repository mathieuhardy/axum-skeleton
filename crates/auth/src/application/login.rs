//! Use-case for login a user.

use common_core::UseCase;

use crate::domain::auth::{Auth, AuthCredentials};
use crate::domain::port::AuthStore;
use crate::prelude::*;

/// Stores used by this use-case.
pub struct LoginStores<A>
where
    A: AuthStore,
{
    /// Auth store.
    pub auth: A,
}

/// Login use-case structure.
pub struct Login<A>
where
    A: AuthStore,
{
    /// List of stores used.
    stores: LoginStores<A>,
}

impl<A> Login<A>
where
    A: AuthStore,
{
    /// Creates a `Login` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: Stores used by this use-case.
    ///
    /// # Returns
    /// A `Login` instance.
    pub fn new(stores: LoginStores<A>) -> Self {
        Self { stores }
    }
}

impl<A> UseCase for Login<A>
where
    A: AuthStore,
{
    type Args = (Auth, AuthCredentials);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (mut auth, credentials) = args;

        let user = self
            .stores
            .auth
            .find_user_by_email(&credentials.email)
            .await
            .map_err(|_| Error::UserNotFound)?;

        // Try to authenticate the user (i.e. check if user exists and password are correct)
        let user = auth.authenticate(&user, &credentials).await?;

        // Check if the user is verified
        if !user.is_email_confirmed() {
            return Err(Error::EmailNotConfirmed);
        }

        // Create the session for this user
        auth.login(&user).await
    }
}
