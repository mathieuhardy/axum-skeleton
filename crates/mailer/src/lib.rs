//! This crate contains the implementation of the user service. It allows to manage the list of
//! users and allow users to register themselves.

#![forbid(unsafe_code)]

mod domain;
mod prelude;
mod provider;

pub use domain::error::Error;
pub use domain::port::MailerProvider;
pub use provider::fake::FakeMailer;

#[cfg(feature = "mock")]
pub use domain::port::MockMailerProvider;
