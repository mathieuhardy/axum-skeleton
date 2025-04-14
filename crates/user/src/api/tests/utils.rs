use axum::http::StatusCode;
use sqlx::postgres::PgPool;

use test_utils::rand::*;
use test_utils::server::*;

use crate::domain::user::{CreateUserRequest, User, UserRole};
use crate::infrastructure::user::{DbUser, DbUserRole};

#[derive(Default)]
pub enum EmailValidity {
    _Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub enum PasswordValidity {
    _Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub struct PostInputs<'a> {
    caller: User,
    email_validity: EmailValidity,
    password_validity: PasswordValidity,
    password: Option<&'a str>,
    role: UserRole,
}

pub async fn post(client: &mut TestClient, inputs: &PostInputs<'_>) -> Option<User> {
    // Prepare inputs
    let PostInputs {
        caller,
        email_validity,
        password_validity,
        password,
        role,
    } = inputs;

    let email = match email_validity {
        EmailValidity::_Invalid => String::new(),
        EmailValidity::Valid => random_email(),
    };

    let password = match password_validity {
        PasswordValidity::_Invalid => password.unwrap_or("").to_string(),
        PasswordValidity::Valid => random_password(),
    };

    // Call endpoint and get response
    let user = CreateUserRequest {
        first_name: random_string(),
        last_name: random_string(),
        email: email.clone(),
        password,
        role: role.clone(),
    };

    let response = client.post("/api/users").json(&user).send().await;

    // Check return code and values
    let expected_status = match (&caller.role, email_validity, password_validity) {
        // Only admin can create users
        (UserRole::Normal, _, _) | (UserRole::Guest, _, _) => StatusCode::FORBIDDEN,

        // Do not allow invalid email or password
        (_, EmailValidity::_Invalid, _) | (_, _, PasswordValidity::_Invalid) => {
            StatusCode::UNPROCESSABLE_ENTITY
        }

        _ => StatusCode::CREATED,
    };

    assert_eq!(response.status(), expected_status);

    if expected_status == StatusCode::CREATED {
        let user = response.json::<User>().await;
        assert_eq!(user.email, email);

        return Some(user);
    }

    None
}

pub async fn post_normal_user(client: &mut TestClient, caller: &User) -> Option<User> {
    post(
        client,
        &PostInputs {
            caller: caller.clone(),
            role: UserRole::Normal,
            ..Default::default()
        },
    )
    .await
}

pub async fn create_user(user: &User, pool: &PgPool) -> Result<User, Box<dyn std::error::Error>> {
    let first_name = random_string();
    let last_name = random_string();

    let user = sqlx::query_as!(
        DbUser,
        "
        INSERT INTO users (first_name, last_name, email, role, password)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            first_name,
            last_name,
            email,
            role AS \"role: _\",
            password,
            created_at,
            updated_at",
        first_name,
        last_name,
        user.email.clone(),
        DbUserRole::from(user.role.clone()) as DbUserRole,
        utils::hashing::hash_password(&user.password)?,
    )
    .fetch_one(pool)
    .await?;

    Ok(user.into())
}
