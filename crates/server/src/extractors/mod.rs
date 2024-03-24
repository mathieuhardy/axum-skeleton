//! This file contains all exports for Axum extractors that can be used in handlers.

pub mod form_or_json;
pub mod redis;

pub use form_or_json::*;
