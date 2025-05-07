//! Use-case trait to be followed by all other crates.

use std::future::Future;

/// A trait representing a use case in an application, typically implementing business logic
/// and interacting with repositories to manage data.
pub trait UseCase {
    /// The input type required to execute the use case
    type Args;
    /// The expect result type upon succesful execution
    type Output;
    /// The error type that can be returned upon execution
    type Error;

    /// Executes a use-case giving him the arguments declared.
    ///
    /// # Arguments
    /// * `args` - Instance of `Self::Args`.
    ///
    /// # Returns
    /// Result of `Self::Outout`.
    fn handle(&self, args: Self::Args) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
