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

#[hook(setup, _)]
#[tokio::test]
#[serial]
async fn all() {
    |client| async move {
        let client = client.lock().await;

        post::login_with_invalid_credentials(&client).await;
        post::login_with_valid_credentials(&client).await;
        post::logout_not_logged_in(&client).await;
        post::logout_logged_in(&client).await;
    }
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

    pub async fn login_with_invalid_credentials(client: &TestClient) {
        println!("{}::login_with_invalid_credentials", module_path!());

        for data_type in [DataType::Form, DataType::Json] {
            // Invalid email
            test_login(
                client,
                data_type.clone(),
                EmailValidity::Invalid,
                PasswordValidity::Valid,
            )
            .await;

            // Invalid password
            test_login(
                client,
                data_type,
                EmailValidity::Valid,
                PasswordValidity::Invalid,
            )
            .await;
        }
    }

    pub async fn login_with_valid_credentials(client: &TestClient) {
        println!("{}::login_with_valid_credentials", module_path!());

        for data_type in [DataType::Form, DataType::Json] {
            test_login(
                client,
                data_type,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;
        }
    }

    pub async fn logout_not_logged_in(client: &TestClient) {
        println!("{}::logout_not_logged_in", module_path!());

        let response = client.post("/logout").send().await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    pub async fn logout_logged_in(client: &TestClient) {
        println!("{}::logout_logged_in", module_path!());

        test_login(
            client,
            DataType::Json,
            EmailValidity::Valid,
            PasswordValidity::Valid,
        )
        .await;

        let response = client.post("/logout").send().await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
