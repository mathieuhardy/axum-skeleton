use axum::http::StatusCode;
use sqlx::postgres::PgPool;
use uuid::Uuid;

use test_utils::rand::*;
use test_utils::server::*;

use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User, UserRole};
use crate::infrastructure::user::{DbUser, DbUserRole};

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

#[derive(Default)]
pub enum FirstNameValidity {
    Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub enum LastNameValidity {
    Invalid,
    #[default]
    Valid,
}

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

#[derive(Default)]
pub struct PostInputs {
    pub caller: User,
    pub email_validity: EmailValidity,
    pub password_validity: PasswordValidity,
    pub role: UserRole,
}

pub async fn post(client: &mut TestClient, inputs: &PostInputs) -> Option<User> {
    // Prepare inputs
    let PostInputs {
        caller,
        email_validity,
        password_validity,
        role,
    } = inputs;

    let email = match email_validity {
        EmailValidity::Invalid => String::new(),
        EmailValidity::Valid => random_email(),
    };

    let password = match password_validity {
        PasswordValidity::Invalid => String::new(),
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
        (_, EmailValidity::Invalid, _) | (_, _, PasswordValidity::Invalid) => {
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

pub async fn post_admin_user(client: &mut TestClient, caller: &User) -> Option<User> {
    post(
        client,
        &PostInputs {
            caller: caller.clone(),
            role: UserRole::Admin,
            ..Default::default()
        },
    )
    .await
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

pub async fn post_guest_user(client: &mut TestClient, caller: &User) -> Option<User> {
    post(
        client,
        &PostInputs {
            caller: caller.clone(),
            role: UserRole::Guest,
            ..Default::default()
        },
    )
    .await
}

#[derive(Default)]
pub struct PatchInputs {
    pub first_name_validity: FirstNameValidity,
    pub last_name_validity: LastNameValidity,
    pub email_validity: EmailValidity,
    pub caller_role: UserRole,
    pub caller_id: Option<Uuid>,
}

pub async fn patch(client: &mut TestClient, id: Uuid, inputs: &PatchInputs) {
    // Prepare inputs
    let PatchInputs {
        first_name_validity,
        last_name_validity,
        email_validity,
        caller_role,
        caller_id,
    } = inputs;

    let caller_id = caller_id.unwrap_or_default();

    let first_name = match first_name_validity {
        FirstNameValidity::Invalid => String::new(),
        FirstNameValidity::Valid => random_string(),
    };

    let last_name = match last_name_validity {
        LastNameValidity::Invalid => String::new(),
        LastNameValidity::Valid => random_string(),
    };

    let email = match email_validity {
        EmailValidity::Invalid => String::new(),
        EmailValidity::Valid => random_email(),
    };

    // Call endpoint and get response
    let user = UpdateUserRequest {
        first_name: first_name.clone(),
        last_name: last_name.clone(),
        email: email.clone(),
        ..Default::default()
    };

    let response = client
        .patch(format!("/api/users/{}", id))
        .json(&user)
        .send()
        .await;
    println!("Response: {:#?}", response.status());

    // Check return code and values
    let expected_status = match caller_role {
        UserRole::Admin => match (first_name_validity, last_name_validity, email_validity) {
            (FirstNameValidity::Invalid, _, _)
            | (_, LastNameValidity::Invalid, _)
            | (_, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,

            _ => StatusCode::OK,
        },

        UserRole::Normal | UserRole::Guest => {
            if caller_id == id {
                match (first_name_validity, last_name_validity, email_validity) {
                    (FirstNameValidity::Invalid, _, _)
                    | (_, LastNameValidity::Invalid, _)
                    | (_, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,

                    _ => StatusCode::OK,
                }
            } else {
                StatusCode::FORBIDDEN
            }
        }
    };

    assert_eq!(response.status(), expected_status);

    if expected_status == StatusCode::OK {
        let user = response.json::<User>().await;
        assert_eq!(user.first_name, first_name);
        assert_eq!(user.last_name, last_name);
        assert_eq!(user.email, email);
    }
}
