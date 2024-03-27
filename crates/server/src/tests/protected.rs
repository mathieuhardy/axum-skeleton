use serial_test::serial;
use test_utils::*;

use crate::tests::auth;
use crate::tests::common::*;

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
            let mut client = client.lock().await;

            let response = client.get("/protected").send().await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn after_login() {
        |client| async move {
            let mut client = client.lock().await;

            // Login
            auth::post::test_login(
                &mut client,
                DataType::Json,
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
            let mut client = client.lock().await;

            // Login
            auth::post::test_login(
                &mut client,
                DataType::Json,
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
