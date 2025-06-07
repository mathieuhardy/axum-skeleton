//! Imports to be used only inside the crate

pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use tracing::instrument;
pub(crate) use uuid::Uuid;

pub(crate) use crate::domain::error::{ApiResult, Error};
