//! This crate is used to manage authentication process in the application.
//! It provides serveral things:
//!
//! - A `AuthUser` structure that will be obtain when a user calls a request.
//! - A `AuthBackend` structure passed in Axum as middleware. Its purpose is to implement the
//!   authentication process.
//! - An extractor providing a `AuthUser` for the Axum endpoints.
//! - Endpoints and use-cases use to login or logout a user.
//!
//! Note that this is a very basic implementation. You'll want, for example enable any kind of 2FA here.
//!
//! Let's say we want to send one-time code by email to the user before allowing its login. In the
//! `login` handler, instead of verifying the password of the user, and before creating the session,
//! we'll proceed this way:
//!
//! 1. create a one-time code or select one in a predefined secret list.
//! 2. send an email with this code to the user.
//! 3. redirect the caller to a page to verify the code.
//!
//! The frontend will then post the code entered by the user to another handler that will (upon
//! success) create the session. The user is now logged in officially.

#![forbid(unsafe_code)]

// Modules
mod api;
mod application;
mod domain;
mod extractor;
mod prelude;

#[cfg(test)]
mod tests;

// Exports
pub use api::router;
pub use domain::auth_backend::{AuthBackend, AuthCredentials};
pub use domain::auth_user::{AuthUser, AuthUserRole};
