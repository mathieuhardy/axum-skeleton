//! Use-case for user email confirmation.

use std::marker::PhantomData;
use uuid::Uuid;

use common_core::UseCase;

use crate::domain::auth::Auth;
use crate::domain::port::AuthStore;
use crate::prelude::*;

/// User confirmation use-case structure.
pub(crate) struct ConfirmEmail<Store> {
    /// Phantom data used to use the `Store` parameter in the struct.
    _marker: PhantomData<Store>,
}

impl<Store> ConfirmEmail<Store> {
    /// Creates a `ConfirmEmail` use-case instance.
    ///
    /// # Returns
    /// A `ConfirmEmail` instance.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<Store> UseCase for ConfirmEmail<Store>
where
    Store: AuthStore,
{
    type Args = (Auth<Store>, Uuid);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (auth, confirmation_id) = args;

        let confirmation = auth
            .store
            .get_user_confirmation_by_id(&confirmation_id)
            .await?;

        if confirmation.is_expired() {
            // Frontend is responsible to alert the user and provide a way to resend the
            // confirmation email with a new link.
            return Err(Error::ConfirmationLinkExpired);
        }

        // Delete confirmation from the store (i.e. the user will now be considered as confirmed).
        auth.store
            .delete_user_confirmation_by_id(&confirmation_id)
            .await
    }
}
