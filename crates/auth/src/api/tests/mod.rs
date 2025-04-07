use axum::http::StatusCode;
use serial_test::serial;

use test_utils::rand::*;
use test_utils::runner::*;
use test_utils::server::*;
use test_utils_derives::*;

use crate::domain::auth_backend::AuthCredentials;
use crate::domain::auth_user::{AuthUser, AuthUserRole};
use crate::tests::create_user;

#[derive(Default)]
pub enum EmailValidity {
    Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub enum PasswordValidity {
    Invalid,
    #[default]
    Valid,
}

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

pub async fn login(
    client: &mut TestClient,
    user: &AuthUser,
    email_validity: EmailValidity,
    password_validity: PasswordValidity,
) {
    // Prepare inputs
    let email = match email_validity {
        EmailValidity::Invalid => random_email(),
        EmailValidity::Valid => user.email.clone(),
    };

    let password = match password_validity {
        PasswordValidity::Invalid => String::new(),
        PasswordValidity::Valid => user.password.clone(),
    };

    // Call endpoint and get response
    let response = client
        .post("/login")
        .cookie_store(true)
        .json(&AuthCredentials { email, password })
        .send()
        .await;

    let expected_status = match (email_validity, password_validity) {
        (EmailValidity::Invalid, _) | (_, PasswordValidity::Invalid) => StatusCode::UNAUTHORIZED,

        _ => StatusCode::OK,
    };

    assert_eq!(response.status(), expected_status);
}

#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn login_with_invalid_credentials() {
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
pub async fn login_with_valid_credentials() {
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
