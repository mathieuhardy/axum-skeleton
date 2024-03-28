//! List of common imports for this crate.

// External crates
pub(crate) use serde::{Deserialize, Serialize};

// Local crates
pub(crate) use database::sqlx::PgPool;
pub(crate) use database::traits::sqlx::postgres::crud::CRUD;
pub(crate) use database::uuid::Uuid;

// Current crate
pub use crate::error::*;
