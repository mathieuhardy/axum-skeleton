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

#[hook(setup, _)]
#[tokio::test]
#[serial]
async fn all() {
    |client| async move {
        let client = client.lock().await;

        get::unauthorized(&client).await;
        get::after_login(&client).await;
        get::after_logout(&client).await;
    }
}

mod get {
    use super::*;

    pub async fn unauthorized(client: &TestClient) {
        println!("{}::unauthorized", module_path!());

        let response = client.get("/protected").send().await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    pub async fn after_login(client: &TestClient) {
        println!("{}::after_login", module_path!());

        // Login
        auth::post::test_login(
            client,
            auth::DataType::Json,
            EmailValidity::Valid,
            PasswordValidity::Valid,
        )
        .await;

        // Check access after login (must be successful)
        let response = client.get("/protected").send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    pub async fn after_logout(client: &TestClient) {
        println!("{}::after_logout", module_path!());

        // Login
        auth::post::test_login(
            client,
            auth::DataType::Json,
            EmailValidity::Valid,
            PasswordValidity::Valid,
        )
        .await;

        // Logout
        let response = client.post("/logout").send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Check access after login (must be successful)
        let response = client.get("/protected").send().await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
