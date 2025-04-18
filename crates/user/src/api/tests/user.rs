use axum::http::StatusCode;
// use rand::distributions::{Alphanumeric, DistString};
use serial_test::serial;
use urlencoding::encode;
// use uuid::Uuid;

// use security::password::{set_checks, Checks};
use test_utils::rand::*;
use test_utils::runner::*;
use test_utils::server::*;
use test_utils_derives::*;

use crate::api::tests::utils::*;
// use crate::domain::user::{PasswordUpdateRequest, UpdateUserRequest, UpsertUserRequest, User};
use crate::domain::user::{User, UserRole};

#[derive(Clone)]
struct AuthUsers {
    admin: User,
    normal: User,
    guest: User,
}

#[derive(Clone)]
struct Users {
    admin: User,
    normal: User,
    guest: User,
}

#[derive(Clone)]
struct Entities {
    auth_users: AuthUsers,
    users: Users,
}

struct DataSet {
    client: TestClient,
    data: Entities,
}

async fn setup() -> DataSet {
    let client = init_server().await.unwrap();

    let auth_user_admin = User {
        email: random_email(),
        role: UserRole::Admin,
        password: random_password(),
        ..Default::default()
    };

    let auth_user_normal = User {
        email: random_email(),
        role: UserRole::Normal,
        password: random_password(),
        ..Default::default()
    };

    let auth_user_guest = User {
        email: random_email(),
        role: UserRole::Guest,
        password: random_password(),
        ..Default::default()
    };

    let user_admin = create_user(&auth_user_admin, &client.db).await.unwrap();
    let user_normal = create_user(&auth_user_normal, &client.db).await.unwrap();
    let user_guest = create_user(&auth_user_guest, &client.db).await.unwrap();

    DataSet {
        client,
        data: Entities {
            auth_users: AuthUsers {
                admin: User {
                    id: user_admin.id,
                    ..auth_user_admin
                },
                normal: User {
                    id: user_normal.id,
                    ..auth_user_normal
                },
                guest: User {
                    id: user_guest.id,
                    ..auth_user_guest
                },
            },
            users: Users {
                admin: user_admin,
                normal: user_normal,
                guest: user_guest,
            },
        },
    }
}

#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_delete_by_id() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let data = dataset.data.clone();
        let client = &mut dataset.client;

        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        let user = post_normal_user(client, &data.auth_users.admin)
            .await
            .unwrap();

        // Test as normal user
        client
            .login(
                &data.auth_users.normal.email,
                &data.auth_users.normal.password,
            )
            .await;

        let response = client
            .delete(format!("/api/users/{}", user.id))
            .send()
            .await;

        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        // Test as guest user
        client
            .login(
                &data.auth_users.guest.email,
                &data.auth_users.guest.password,
            )
            .await;

        let response = client
            .delete(format!("/api/users/{}", user.id))
            .send()
            .await;

        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        // Test as admin
        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        let response = client
            .delete(format!("/api/users/{}", user.id))
            .send()
            .await;

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}

#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_get_current_unauthorized() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let client = &mut dataset.client;

        let response = client.get("/api/users/current").send().await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_get_current_after_logout() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let data = dataset.data.clone();
        let client = &mut dataset.client;

        // Login as admin
        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        // Logout
        let response = client.post("/logout").send().await;
        assert_eq!(response.status(), StatusCode::OK);

        // Check access after login (must be successful)
        let response = client.get("/api/users/current").send().await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

/// TEST_PLAN: /TC/USERS/GET/ME
#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_get_current_nominal() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let data = dataset.data.clone();
        let client = &mut dataset.client;

        // Login as admin
        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        // Check access after login (must be successful)
        let response = client.get("/api/users/current").send().await;
        assert_eq!(response.status(), StatusCode::OK);

        let user = response.json::<User>().await;
        assert_eq!(user.email, data.auth_users.admin.email);
    }
}

/// TEST_PLAN: /TC/USERS/GET/ALL
#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_get_all() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let data = dataset.data.clone();
        let client = &mut dataset.client;

        // Test as admin user
        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        let response = client.get("/api/users").send().await;
        assert_eq!(response.status(), StatusCode::OK);

        let users = response.json::<Vec<User>>().await;
        assert!(!users.is_empty());
        assert!(users.iter().any(|e| e.email == data.auth_users.admin.email));
        assert!(users
            .iter()
            .any(|e| e.email == data.auth_users.normal.email));
        assert!(users.iter().any(|e| e.email == data.auth_users.guest.email));

        // Test as normal user
        client
            .login(
                &data.auth_users.normal.email,
                &data.auth_users.normal.password,
            )
            .await;

        let response = client.get("/api/users").send().await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        // Test as guest user
        client
            .login(
                &data.auth_users.guest.email,
                &data.auth_users.guest.password,
            )
            .await;

        let response = client.get("/api/users").send().await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}

/// TEST_PLAN: /TC/USERS/GET/FILTERED
#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_get_by_filters() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let data = dataset.data.clone();
        let client = &mut dataset.client;

        // Login as admin
        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        // By name
        let response = client
            .get(format!(
                "/api/users?first_name={}",
                encode(&data.users.guest.first_name)
            ))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let users = response.json::<Vec<User>>().await;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].first_name, data.users.guest.first_name);
        assert_eq!(users[0].last_name, data.users.guest.last_name);
        assert_eq!(users[0].email, data.users.guest.email);

        // By name (not found)
        let response = client.get("/api/users?first_name=404").send().await;
        assert_eq!(response.status(), StatusCode::OK);

        let users = response.json::<Vec<User>>().await;
        assert!(users.is_empty());

        // By email
        let response = client
            .get(format!("/api/users?email={}", data.auth_users.admin.email))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let users = response.json::<Vec<User>>().await;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, data.auth_users.admin.email);

        // By email (not found)
        let response = client.get("/api/users?email=404").send().await;
        assert_eq!(response.status(), StatusCode::OK);

        let users = response.json::<Vec<User>>().await;
        assert!(users.is_empty());

        // Test as normal user
        client
            .login(
                &data.auth_users.normal.email,
                &data.auth_users.normal.password,
            )
            .await;

        let response = client.get("/api/users").send().await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        // Test as guest user
        client
            .login(
                &data.auth_users.guest.email,
                &data.auth_users.guest.password,
            )
            .await;

        let response = client.get("/api/users").send().await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}

/// TEST_PLAN: /TC/USERS/GET/ID
#[hook(setup, _)]
#[tokio::test]
#[serial]
pub async fn test_get_by_id() {
    |dataset| async move {
        let mut dataset = dataset.lock().await;
        let data = dataset.data.clone();
        let client = &mut dataset.client;

        // Test as admin
        client
            .login(
                &data.auth_users.admin.email,
                &data.auth_users.admin.password,
            )
            .await;

        let response = client
            .get(format!("/api/users/{}", data.users.guest.id))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let fetched = response.json::<User>().await;
        assert_eq!(fetched.id, data.users.guest.id);

        // Test as normal user
        client
            .login(
                &data.auth_users.normal.email,
                &data.auth_users.normal.password,
            )
            .await;

        let response = client
            .get(format!("/api/users/{}", data.users.guest.id))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        // Test as guest user
        client
            .login(
                &data.auth_users.guest.email,
                &data.auth_users.guest.password,
            )
            .await;

        let response = client
            .get(format!("/api/users/{}", data.users.guest.id))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}

mod update {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_update_user_as_admin() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.admin.email,
                    &data.auth_users.admin.password,
                )
                .await;

            patch(
                client,
                data.users.admin.id,
                &PatchInputs {
                    caller_role: UserRole::Admin,
                    ..Default::default()
                },
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_update_user_as_normal() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.normal.email,
                    &data.auth_users.normal.password,
                )
                .await;

            // Cannot update another user
            patch(
                client,
                data.auth_users.guest.id,
                &PatchInputs {
                    caller_role: UserRole::Normal,
                    caller_id: Some(data.auth_users.normal.id),
                    ..Default::default()
                },
            )
            .await;

            // Can update myself
            patch(
                client,
                data.auth_users.normal.id,
                &PatchInputs {
                    caller_role: UserRole::Normal,
                    caller_id: Some(data.auth_users.normal.id),
                    ..Default::default()
                },
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_update_user_as_guest() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.guest.email,
                    &data.auth_users.guest.password,
                )
                .await;

            // Cannot update another user
            patch(
                client,
                data.auth_users.normal.id,
                &PatchInputs {
                    caller_role: UserRole::Guest,
                    caller_id: Some(data.auth_users.guest.id),
                    ..Default::default()
                },
            )
            .await;

            // Can update myself
            patch(
                client,
                data.auth_users.guest.id,
                &PatchInputs {
                    caller_role: UserRole::Guest,
                    caller_id: Some(data.auth_users.guest.id),
                    ..Default::default()
                },
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_update_user_validation() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.admin.email,
                    &data.auth_users.admin.password,
                )
                .await;

            // Invalid email
            patch(
                client,
                data.users.normal.id,
                &PatchInputs {
                    email_validity: EmailValidity::Invalid,
                    caller_role: UserRole::Admin,
                    ..Default::default()
                },
            )
            .await;

            // Invalid first name
            patch(
                client,
                data.users.normal.id,
                &PatchInputs {
                    first_name_validity: FirstNameValidity::Invalid,
                    caller_role: UserRole::Admin,
                    ..Default::default()
                },
            )
            .await;

            // Invalid last name
            patch(
                client,
                data.users.normal.id,
                &PatchInputs {
                    last_name_validity: LastNameValidity::Invalid,
                    caller_role: UserRole::Admin,
                    ..Default::default()
                },
            )
            .await;
        }
    }
}

mod create {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_create_user_nominal() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.admin.email,
                    &data.auth_users.admin.password,
                )
                .await;

            post(
                client,
                &PostInputs {
                    caller: data.auth_users.admin,
                    ..Default::default()
                },
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_create_user_as_normal() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.normal.email,
                    &data.auth_users.normal.password,
                )
                .await;

            post(
                client,
                &PostInputs {
                    caller: data.auth_users.normal,
                    ..Default::default()
                },
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_create_user_as_guest() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.guest.email,
                    &data.auth_users.guest.password,
                )
                .await;

            post(
                client,
                &PostInputs {
                    caller: data.auth_users.guest,
                    ..Default::default()
                },
            )
            .await;
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn test_create_user_validation() {
        |dataset| async move {
            let mut dataset = dataset.lock().await;
            let data = dataset.data.clone();
            let client = &mut dataset.client;

            client
                .login(
                    &data.auth_users.admin.email,
                    &data.auth_users.admin.password,
                )
                .await;

            // Invalid email
            post(
                client,
                &PostInputs {
                    caller: data.auth_users.admin.clone(),
                    email_validity: EmailValidity::Invalid,
                    ..Default::default()
                },
            )
            .await;

            // Invalid password
            post(
                client,
                &PostInputs {
                    caller: data.auth_users.admin,
                    password_validity: PasswordValidity::Invalid,
                    ..Default::default()
                },
            )
            .await;
        }
    }
}
