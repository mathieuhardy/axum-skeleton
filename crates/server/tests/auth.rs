// TODO: find a better solution
#[allow(clippy::duplicate_mod)]
#[path = "common/mod.rs"]
mod common;

use serial_test::serial;
use test_utils::*;

use server::layers::auth::*;

pub use common::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

pub mod post {
    use super::*;

    pub async fn test_login(
        client: &TestClient,
        data_type: DataType,
        email_validity: EmailValidity,
        password_validity: PasswordValidity,
    ) {
        // Prepare inputs
        let email = match email_validity {
            EmailValidity::Invalid => INVALID_EMAIL,
            EmailValidity::Valid => ADMIN_EMAIL,
        }
        .to_string();

        let password = match password_validity {
            PasswordValidity::Invalid => INVALID_PASSWORD,
            PasswordValidity::Valid => ADMIN_PASSWORD,
        }
        .to_string();

        // Call endpoint and get response
        let response = match data_type {
            DataType::Json => {
                client
                    .post("/login")
                    .json(&Credentials { email, password })
                    .send()
                    .await
            }

            DataType::Form => {
                let data = [("email", &email), ("password", &password)];

                client.post("/login").form(&data).send().await
            }
        };

        let expected_status = match (email_validity, password_validity) {
            (EmailValidity::Invalid, _) | (_, PasswordValidity::Invalid) => {
                StatusCode::UNAUTHORIZED
            }

            _ => StatusCode::OK,
        };

        assert_eq!(response.status(), expected_status);
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn login_with_invalid_credentials() {
        |client| async move {
            let client = client.lock().await;

            for data_type in [DataType::Form, DataType::Json] {
                // Invalid email
                test_login(
                    &client,
                    data_type.clone(),
                    EmailValidity::Invalid,
                    PasswordValidity::Valid,
                )
                .await;

                // Invalid password
                test_login(
                    &client,
                    data_type,
                    EmailValidity::Valid,
                    PasswordValidity::Invalid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn login_with_valid_credentials() {
        |client| async move {
            let client = client.lock().await;

            for data_type in [DataType::Form, DataType::Json] {
                test_login(
                    &client,
                    data_type,
                    EmailValidity::Valid,
                    PasswordValidity::Valid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn logout_not_logged_in() {
        |client| async move {
            let client = client.lock().await;

            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn logout_logged_in() {
        |client| async move {
            let client = client.lock().await;

            test_login(
                &client,
                DataType::Json,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;

            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
