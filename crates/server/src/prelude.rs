//! List of common imports for this crate.

// External crates
pub(crate) use axum::extract::{Path, Query, State};
pub(crate) use axum::http::StatusCode;
pub(crate) use axum::response::IntoResponse;
pub(crate) use axum::routing::{delete, get, patch, post, put};
pub(crate) use axum::{Json, Router};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use tracing::{event, instrument, Level};
pub(crate) use uuid::Uuid;
pub(crate) use validator::Validate;

// Current crate
pub(crate) use crate::error::*;
pub(crate) use crate::extractors::*;
pub(crate) use crate::layers::auth::*;
pub(crate) use crate::state::*;
