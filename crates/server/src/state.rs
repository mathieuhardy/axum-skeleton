//! This file contains all State related structures and functions. The state is
//! an struct passed along routes that will be called in order to share common
//! objects (e.g. the database handle).

/// State structure passed along routes.
#[derive(Clone, Debug, Default)]
pub struct State {}

impl State {
    /// Creates a new State instance with default values.
    ///
    /// # Returns
    /// New instance of State.
    pub fn new() -> Self {
        Self::default()
    }
}
