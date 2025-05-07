//! User confirmation data structures.

use chrono::{DateTime, Utc};

use crate::prelude::*;

/// Mirrors the `user_confirmations`'s' table.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserConfirmation {
    /// Unique record identifier.
    pub id: Uuid,

    /// ID of the user it relates to.
    pub user_id: Uuid,

    /// Date of expiration of the token.
    pub expires_at: DateTime<Utc>,
}
