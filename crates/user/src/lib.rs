#![forbid(unsafe_code)]

mod api;
mod application;
mod domain;
mod infrastructure;
mod prelude;

#[cfg(test)]
mod tests;

pub use api::user::router;
pub use domain::user::{User, UserRole};
