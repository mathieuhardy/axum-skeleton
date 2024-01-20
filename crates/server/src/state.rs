//! This file contains all State related structures and functions. The state is
//! an struct passed along routes that will be called in order to share common
//! objects (e.g. the database handle).

use sqlx::{Pool, Postgres};

/// State structure passed along routes.
#[derive(Clone, Debug)]
pub struct AppState {
    /// PostgreSQL database handle.
    pub db: Pool<Postgres>,
}

impl AppState {
    /// Creates a new AppState instance with default values.
    ///
    /// # Returns
    /// New instance of AppState.
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}
