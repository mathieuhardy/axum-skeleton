use axum::http::StatusCode;

use test_utils::server::*;

use crate::domain::auth_user::{AuthCredentials, AuthUser};

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

pub async fn login(
    client: &mut TestClient,
    user: &AuthUser,
    email_validity: EmailValidity,
    password_validity: PasswordValidity,
) {
    // Prepare inputs
    let email = match email_validity {
        EmailValidity::Invalid => String::new(),
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
        (EmailValidity::Invalid, _) | (_, PasswordValidity::Invalid) => {
            StatusCode::UNPROCESSABLE_ENTITY
        }

        _ => StatusCode::OK,
    };

    assert_eq!(response.status(), expected_status);
}
