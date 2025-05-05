//! Use-case for logout a user.

use std::marker::PhantomData;

use common_core::UseCase;

use crate::domain::auth::Auth;
use crate::domain::port::AuthStore;
use crate::prelude::*;

/// Logout use-case structure.
pub(crate) struct Logout<Store> {
    /// Phantom data used to use the `Store` parameter in the struct.
    _marker: PhantomData<Store>,
}

impl<Store> Logout<Store> {
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

impl<Store> UseCase for Logout<Store>
where
    Store: AuthStore,
{
    type Args = Auth<Store>;
    type Output = ();
    type Error = Error;

    async fn handle(&self, mut auth: Self::Args) -> Result<Self::Output, Self::Error> {
        auth.logout().await
    }
}
