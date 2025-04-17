//! This crate provides test utilities for the other crates.

#![forbid(unsafe_code)]

#[cfg(feature = "rand")]
pub mod rand;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "runner")]
pub mod runner;

#[cfg(feature = "server")]
pub mod server;
