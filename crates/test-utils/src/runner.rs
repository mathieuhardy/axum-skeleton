//! This file contains everything needed for unit testing (i.e. creating a
//! server instance, etc).

use std::future::Future;
use std::panic::{catch_unwind, UnwindSafe};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Runs a test calling a setup function before the test and a teardown function
/// at the end.
///
/// # Arguments
/// * `setup` - Setup function called before the body.
/// * `body` - Body function of the test.
/// * `teardown` - Teardown function called after the body.
pub async fn run_test<Setup, Body, Teardown, Data, SetupReturn, BodyReturn, TeardownReturn>(
    setup: Setup,
    body: Body,
    teardown: Teardown,
) where
    Setup: FnOnce() -> SetupReturn + UnwindSafe,
    SetupReturn: Future<Output = Data>,
    Body: FnOnce(Arc<Mutex<Data>>) -> BodyReturn + UnwindSafe,
    BodyReturn: Future<Output = ()>,
    Teardown: FnOnce(Arc<Mutex<Data>>) -> TeardownReturn + UnwindSafe,
    TeardownReturn: Future<Output = ()>,
{
    // Call setup and check result
    let setup_result = catch_unwind(async || setup().await);
    assert!(setup_result.is_ok());

    // Prepare data for the next calls
    let body_data = Arc::new(Mutex::new(setup_result.unwrap().await));
    let teardown_data = body_data.clone();

    // Call body and teardown without checking errors (we want to be sure the teardown is always
    // called)
    let body_result = catch_unwind(std::panic::AssertUnwindSafe(async || body(body_data).await));

    let teardown_result = catch_unwind(std::panic::AssertUnwindSafe(async || {
        teardown(teardown_data).await
    }));

    // Checks final results in order
    assert!(body_result.is_ok());
    body_result.unwrap().await;

    assert!(teardown_result.is_ok());
    teardown_result.unwrap().await;
}

/// Dummy function use as placeholder when calling the `run_test` function.
pub async fn no_setup() {}

/// Dummy function use as placeholder when calling the `run_test` function.
pub async fn no_body<T>(_: T) {}

/// Dummy function use as placeholder when calling the `run_test` function.
pub async fn no_teardown<T>(_: T) {}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_failure() {
        panic!();
    }

    async fn body_failure<T>(_: T) {
        panic!();
    }

    async fn teardown_failure<T>(_: T) {
        panic!();
    }

    mod nominal {
        use super::*;

        #[tokio::test]
        async fn setup() {
            run_test(no_setup, no_body, no_teardown).await
        }
    }

    mod failures {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn setup() {
            run_test(setup_failure, no_body, no_teardown).await
        }

        #[tokio::test]
        #[should_panic]
        async fn body() {
            run_test(no_setup, body_failure, no_teardown).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn teardown() {
            run_test(no_setup, no_body, teardown_failure).await;
        }
    }
}
