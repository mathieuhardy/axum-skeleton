// TODO: find a better solution
#[allow(clippy::duplicate_mod)]
#[path = "auth.rs"]
mod auth;

#[allow(clippy::duplicate_mod)]
#[path = "common/mod.rs"]
mod common;

use serial_test::serial;
use test_utils::*;

use auth::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

mod get {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn unauthorized() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/protected").send().await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn after_login() {
        |client| async move {
            let client = client.lock().await;

            // Login
            auth::post::test_login(
                &client,
                auth::DataType::Json,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;

            // Check access after login (must be successful)
            let response = client.get("/protected").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn after_logout() {
        |client| async move {
            let client = client.lock().await;

            // Login
            auth::post::test_login(
                &client,
                auth::DataType::Json,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;

            // Logout
            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);

            // Check access after login (must be successful)
            let response = client.get("/protected").send().await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
    }
}
