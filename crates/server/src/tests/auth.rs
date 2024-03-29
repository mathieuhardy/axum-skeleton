use serial_test::serial;
use test_utils::*;

use database::models::users::{User, UserRole};

use crate::layers::auth::*;
use crate::tests::common::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

pub mod post {
    use super::*;

    pub async fn login(
        client: &mut TestClient,
        user: &User,
        data_type: &DataType,
        email_validity: EmailValidity,
        password_validity: PasswordValidity,
    ) {
        // Prepare inputs
        let email = match email_validity {
            EmailValidity::Invalid => INVALID_EMAIL.to_string(),
            EmailValidity::Valid => user.email.clone(),
        };

        let password = match password_validity {
            PasswordValidity::Invalid => INVALID_PASSWORD.to_string(),
            PasswordValidity::Valid => user.password.clone(),
        };

        // Call endpoint and get response
        let response = match data_type {
            DataType::Json => {
                client
                    .post("/login")
                    .cookie_store(true)
                    .json(&Credentials { email, password })
                    .send()
                    .await
            }

            DataType::Form => {
                let data = [("email", &email), ("password", &password)];

                client
                    .post("/login")
                    .cookie_store(true)
                    .form(&data)
                    .send()
                    .await
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

    pub async fn login_as_admin(client: &mut TestClient) {
        let user = get_user_info(UserRole::Admin).unwrap();

        login_with_user(client, &user).await;
    }

    pub async fn login_as_normal(client: &mut TestClient) {
        let user = get_user_info(UserRole::Normal).unwrap();

        login_with_user(client, &user).await;
    }

    pub async fn login_as_guest(client: &mut TestClient) {
        let user = get_user_info(UserRole::Guest).unwrap();

        login_with_user(client, &user).await;
    }

    pub async fn login_as(client: &mut TestClient, email: &str, password: &str) {
        login_with_user(
            client,
            &User {
                email: email.to_string(),
                password: password.to_string(),
                ..User::default()
            },
        )
        .await
    }

    async fn login_with_user(client: &mut TestClient, user: &User) {
        login(
            client,
            user,
            &DataType::Json,
            EmailValidity::Valid,
            PasswordValidity::Valid,
        )
        .await
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn login_with_invalid_credentials() {
        |client| async move {
            let mut client = client.lock().await;

            let user = get_user_info(UserRole::Admin).unwrap();

            for data_type in [DataType::Form, DataType::Json] {
                // Invalid email
                login(
                    &mut client,
                    &user,
                    &data_type,
                    EmailValidity::Invalid,
                    PasswordValidity::Valid,
                )
                .await;

                // Invalid password
                login(
                    &mut client,
                    &user,
                    &data_type,
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
            let mut client = client.lock().await;

            let user = get_user_info(UserRole::Admin).unwrap();

            for data_type in [DataType::Form, DataType::Json] {
                login(
                    &mut client,
                    &user,
                    &data_type,
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
            let mut client = client.lock().await;

            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn logout_logged_in() {
        |client| async move {
            let mut client = client.lock().await;

            let user = get_user_info(UserRole::Admin).unwrap();

            login(
                &mut client,
                &user,
                &DataType::Json,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;

            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
