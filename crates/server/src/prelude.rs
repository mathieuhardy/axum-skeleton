//! List of common imports for this crate.

// External crates
pub(crate) use axum::extract::{Path, Query, State};
pub(crate) use axum::http::StatusCode;
pub(crate) use axum::routing::{get, patch, post, put};
pub(crate) use axum::{Json, Router};
pub(crate) use tracing::{event, instrument, Level};
pub(crate) use uuid::Uuid;

// Local crates
pub use database::error::Error as DatabaseError;

// Current crate
pub(crate) use crate::error::*;
pub(crate) use crate::state::*;
pub(crate) use crate::types::*;
