//! List of common imports for this crate.

// Local crates
pub(crate) use database::sqlx::PgPool;
pub(crate) use database::traits::sqlx::postgres::crud::CRUD;
pub(crate) use database::uuid::Uuid;

// Current crate
pub use crate::error::*;
