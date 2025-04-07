//! Kubernetes specific crate.

#![forbid(unsafe_code)]

mod api;

pub use api::router;
