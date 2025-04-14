mod utils;

use axum::http::StatusCode;
use serial_test::serial;

use test_utils::rand::*;
use test_utils::runner::*;
use test_utils::server::*;
use test_utils_derives::*;

use crate::api::tests::utils::*;
use crate::domain::auth_user::{AuthUser, AuthUserRole};
use crate::tests::utils::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

mod login {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_login_with_invalid_credentials() {
        |client| async move {
            let mut client = client.lock().await;

            let auth_user = AuthUser {
                email: random_email(),
                role: AuthUserRole::Admin,
                password: random_password(),
                ..AuthUser::default()
            };

            let _ = create_user(&auth_user, &client.db).await.unwrap();

            // Invalid email
            login(
                &mut client,
                &auth_user,
                EmailValidity::Invalid,
                PasswordValidity::Valid,
            )
            .await;

            // Invalid password
            login(
                &mut client,
                &auth_user,
                EmailValidity::Valid,
                PasswordValidity::Invalid,
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_login_with_valid_credentials() {
        |client| async move {
            let mut client = client.lock().await;

            let auth_user = AuthUser {
                email: random_email(),
                role: AuthUserRole::Admin,
                password: random_password(),
                ..AuthUser::default()
            };

            let _ = create_user(&auth_user, &client.db).await.unwrap();

            login(
                &mut client,
                &auth_user,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;
        }
    }
}

mod logout {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_logout_not_logged_in() {
        |client| async move {
            let mut client = client.lock().await;

            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_logout_logged_in() {
        |client| async move {
            let mut client = client.lock().await;

            let auth_user = AuthUser {
                email: random_email(),
                role: AuthUserRole::Admin,
                password: random_password(),
                ..AuthUser::default()
            };

            let _ = create_user(&auth_user, &client.db).await.unwrap();

            login(
                &mut client,
                &auth_user,
                EmailValidity::Valid,
                PasswordValidity::Valid,
            )
            .await;

            let response = client.post("/logout").send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
