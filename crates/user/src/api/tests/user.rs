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
    _admin: User,
    _normal: User,
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
                admin: auth_user_admin,
                normal: auth_user_normal,
                guest: auth_user_guest,
            },
            users: Users {
                _admin: user_admin,
                _normal: user_normal,
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

// TODO: re-enable tests
// mod patch {
//     use super::*;
//
//     #[derive(Default)]
//     pub struct PatchInputs {
//         first_name_validity: FirstNameValidity,
//         last_name_validity: LastNameValidity,
//         email_validity: EmailValidity,
//         caller_role: UserRole,
//         caller_id: Option<Uuid>,
//     }
//
//     async fn patch(client: &mut TestClient, id: Uuid, data_type: &DataType, inputs: &PatchInputs) {
//         // Prepare inputs
//         let PatchInputs {
//             first_name_validity,
//             last_name_validity,
//             email_validity,
//             caller_role,
//             caller_id,
//         } = inputs;
//
//         let caller_id = caller_id.unwrap_or_default();
//
//         let uniq = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
//
//         let first_name = match first_name_validity {
//             FirstNameValidity::Invalid => String::new(),
//             FirstNameValidity::Valid => format!("{uniq}-first-name"),
//         };
//
//         let last_name = match last_name_validity {
//             LastNameValidity::Invalid => String::new(),
//             LastNameValidity::Valid => format!("{uniq}-last-name"),
//         };
//
//         let email = match email_validity {
//             EmailValidity::Invalid => uniq,
//             EmailValidity::Valid => format!("{uniq}@email.com"),
//         };
//
//         // Call endpoint and get response
//         let response = match data_type {
//             DataType::Json => {
//                 let user = UpdateUserRequest {
//                     first_name: Some(first_name.clone()),
//                     last_name: Some(last_name.clone()),
//                     email: Some(email.clone()),
//                     ..Default::default()
//                 };
//
//                 client
//                     .patch(format!("/api/users/{}", id))
//                     .json(&user)
//                     .send()
//                     .await
//             }
//
//             DataType::Form => {
//                 let user = [
//                     ("first_name", &first_name),
//                     ("last_name", &last_name),
//                     ("email", &email),
//                 ];
//
//                 client
//                     .patch(format!("/api/users/{}", id))
//                     .form(&user)
//                     .send()
//                     .await
//             }
//         };
//
//         // Check return code and values
//         let expected_status = match caller_role {
//             UserRole::Admin => match (first_name_validity, last_name_validity, email_validity) {
//                 (FirstNameValidity::Invalid, _, _)
//                 | (_, LastNameValidity::Invalid, _)
//                 | (_, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,
//
//                 _ => StatusCode::OK,
//             },
//
//             UserRole::Normal => {
//                 if caller_id == id {
//                     match (first_name_validity, last_name_validity, email_validity) {
//                         (FirstNameValidity::Invalid, _, _)
//                         | (_, LastNameValidity::Invalid, _)
//                         | (_, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,
//
//                         _ => StatusCode::OK,
//                     }
//                 } else {
//                     StatusCode::FORBIDDEN
//                 }
//             }
//
//             UserRole::Guest => StatusCode::FORBIDDEN,
//         };
//
//         assert_eq!(response.status(), expected_status);
//
//         if expected_status == StatusCode::OK {
//             let user = response.json::<User>().await;
//             assert_eq!(user.first_name, first_name);
//             assert_eq!(user.last_name, last_name);
//             assert_eq!(user.email, email);
//         }
//     }
//
//     #[derive(Default)]
//     pub struct PatchPasswordInputs<'a> {
//         current_password_validity: PasswordValidity,
//         current_password: Option<&'a str>,
//         password_validity: PasswordValidity,
//         password: Option<&'a str>,
//         caller_role: UserRole,
//         caller_id: Option<Uuid>,
//     }
//
//     async fn patch_password(
//         client: &mut TestClient,
//         id: &Uuid,
//         data_type: &DataType,
//         inputs: &PatchPasswordInputs<'_>,
//     ) {
//         let PatchPasswordInputs {
//             current_password_validity,
//             current_password,
//             password_validity,
//             password,
//             caller_role,
//             caller_id,
//         } = inputs;
//
//         let caller_id = caller_id.unwrap_or_default();
//
//         let current_password = current_password.unwrap_or("").to_string();
//
//         let password = match password_validity {
//             PasswordValidity::Invalid => password.unwrap_or("").to_string(),
//             PasswordValidity::Valid => "0#Abcdef".to_string(),
//         };
//
//         // Call endpoint and get response
//         let response = match data_type {
//             DataType::Json => {
//                 let request = PasswordUpdateRequest {
//                     current: current_password,
//                     new: password,
//                 };
//
//                 client
//                     .patch(format!("/api/users/{}/password", id))
//                     .json(&request)
//                     .send()
//                     .await
//             }
//
//             DataType::Form => {
//                 let request = [("current", &current_password), ("new", &password)];
//
//                 client
//                     .patch(format!("/api/users/{}/password", id))
//                     .form(&request)
//                     .send()
//                     .await
//             }
//         };
//
//         // Check return code and values
//         let expected_status = match caller_role {
//             UserRole::Admin | UserRole::Normal => {
//                 if caller_id == *id {
//                     match (current_password_validity, password_validity) {
//                         (PasswordValidity::Invalid, _) => StatusCode::FORBIDDEN,
//                         (_, PasswordValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,
//                         _ => StatusCode::OK,
//                     }
//                 } else {
//                     StatusCode::FORBIDDEN
//                 }
//             }
//
//             UserRole::Guest => StatusCode::FORBIDDEN,
//         };
//
//         assert_eq!(response.status(), expected_status);
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn as_admin_user() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 patch(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PatchInputs {
//                         caller_role: UserRole::Admin,
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn as_normal_user() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//             let user_2 = post::post_normal_user(&mut client).await.unwrap();
//
//             auth::post::login_as(&mut client, &user.email, NORMAL_PASSWORD).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 // Can update myself
//                 patch(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PatchInputs {
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//
//                 // Cannot update another user
//                 patch(
//                     &mut client,
//                     user_2.id,
//                     &data_type,
//                     &PatchInputs {
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn as_guest_user() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_guest_user(&mut client).await.unwrap();
//
//             auth::post::login_as(&mut client, &user.email, GUEST_PASSWORD).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 patch(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PatchInputs {
//                         caller_role: UserRole::Guest,
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_email() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             auth::post::login_as(&mut client, &user.email, NORMAL_PASSWORD).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 patch(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PatchInputs {
//                         email_validity: EmailValidity::Invalid,
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_first_name() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             auth::post::login_as(&mut client, &user.email, NORMAL_PASSWORD).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 patch(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PatchInputs {
//                         first_name_validity: FirstNameValidity::Invalid,
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_last_name() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             auth::post::login_as(&mut client, &user.email, NORMAL_PASSWORD).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 patch(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PatchInputs {
//                         last_name_validity: LastNameValidity::Invalid,
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn set_password_as_admin_user() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 auth::post::login_as_admin(&mut client).await;
//
//                 // Create a user to update
//                 let user = post::post_admin_user(&mut client).await.unwrap();
//                 let user_2 = post::post_admin_user(&mut client).await.unwrap();
//
//                 auth::post::login_as(&mut client, &user.email, ADMIN_PASSWORD).await;
//
//                 // Cannot update another user
//                 patch_password(
//                     &mut client,
//                     &user_2.id,
//                     &data_type,
//                     &PatchPasswordInputs {
//                         current_password: Some(ADMIN_PASSWORD),
//                         caller_role: UserRole::Admin,
//                         caller_id: Some(user.id),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//
//                 // Can update myself
//                 patch_password(
//                     &mut client,
//                     &user.id,
//                     &data_type,
//                     &PatchPasswordInputs {
//                         current_password: Some(ADMIN_PASSWORD),
//                         caller_role: UserRole::Admin,
//                         caller_id: Some(user.id),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn set_password_as_normal_user() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 auth::post::login_as_admin(&mut client).await;
//
//                 // Create a user to update
//                 let user = post::post_normal_user(&mut client).await.unwrap();
//                 let user_2 = post::post_normal_user(&mut client).await.unwrap();
//
//                 auth::post::login_as(&mut client, &user.email, NORMAL_PASSWORD).await;
//
//                 // Cannot update another user
//                 patch_password(
//                     &mut client,
//                     &user_2.id,
//                     &data_type,
//                     &PatchPasswordInputs {
//                         current_password: Some(NORMAL_PASSWORD),
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//
//                 // Can update myself
//                 patch_password(
//                     &mut client,
//                     &user.id,
//                     &data_type,
//                     &PatchPasswordInputs {
//                         current_password: Some(NORMAL_PASSWORD),
//                         caller_role: UserRole::Normal,
//                         caller_id: Some(user.id),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn set_password_as_guest_user() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 auth::post::login_as_admin(&mut client).await;
//
//                 // Create a user to update
//                 let user = post::post_guest_user(&mut client).await.unwrap();
//                 let user_2 = post::post_guest_user(&mut client).await.unwrap();
//
//                 auth::post::login_as(&mut client, &user.email, GUEST_PASSWORD).await;
//
//                 // Cannot update another user
//                 patch_password(
//                     &mut client,
//                     &user_2.id,
//                     &data_type,
//                     &PatchPasswordInputs {
//                         current_password: Some(GUEST_PASSWORD),
//                         caller_role: UserRole::Guest,
//                         caller_id: Some(user.id),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//
//                 // Can update myself
//                 patch_password(
//                     &mut client,
//                     &user.id,
//                     &data_type,
//                     &PatchPasswordInputs {
//                         current_password: Some(GUEST_PASSWORD),
//                         caller_role: UserRole::Guest,
//                         caller_id: Some(user.id),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn set_password() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             // Create a user to update
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             let data_types = vec![DataType::Form, DataType::Json];
//
//             // Invalid current password
//             for data_type in &data_types {
//                 patch_password(
//                     &mut client,
//                     &user.id,
//                     data_type,
//                     &PatchPasswordInputs {
//                         current_password_validity: PasswordValidity::Invalid,
//                         current_password: Some("INVALID_PASSWORD"),
//                         ..PatchPasswordInputs::default()
//                     },
//                 )
//                 .await;
//             }
//
//             // Invalid new passwords
//             let passwords = vec![
//                 ".#Abcdef",
//                 "0#ABCDEF",
//                 "0#abcdef",
//                 "0Abcdefg",
//                 "0#Abcde f",
//                 "0#Abcde",
//             ];
//
//             for data_type in &data_types {
//                 for password in &passwords {
//                     patch_password(
//                         &mut client,
//                         &user.id,
//                         data_type,
//                         &PatchPasswordInputs {
//                             password_validity: PasswordValidity::Invalid,
//                             password: Some(password),
//                             ..PatchPasswordInputs::default()
//                         },
//                     )
//                     .await;
//                 }
//             }
//         }
//     }
// }
//
// mod post {
//     use super::*;

//
//     pub async fn post_admin_user(client: &mut TestClient) -> Option<User> {
//         post(
//             client,
//             &DataType::Json,
//             &PostInputs {
//                 role: UserRole::Admin,
//                 caller_role: UserRole::Admin,
//                 ..PostInputs::default()
//             },
//         )
//         .await
//     }
//

//
//     pub async fn post_guest_user(client: &mut TestClient) -> Option<User> {
//         post(
//             client,
//             &DataType::Json,
//             &PostInputs {
//                 role: UserRole::Guest,
//                 caller_role: UserRole::Admin,
//                 ..PostInputs::default()
//             },
//         )
//         .await
//     }
//
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn nominal() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 post(
//                     &mut client,
//                     &data_type,
//                     &PostInputs {
//                         caller_role: UserRole::Admin,
//                         ..PostInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn as_non_admin() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_normal(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 post(
//                     &mut client,
//                     &data_type,
//                     &PostInputs {
//                         caller_role: UserRole::Normal,
//                         ..PostInputs::default()
//                     },
//                 )
//                 .await;
//             }
//
//             auth::post::login_as_guest(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 post(
//                     &mut client,
//                     &data_type,
//                     &PostInputs {
//                         caller_role: UserRole::Guest,
//                         ..PostInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_email() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 post(
//                     &mut client,
//                     &data_type,
//                     &PostInputs {
//                         caller_role: UserRole::Admin,
//                         email_validity: EmailValidity::Invalid,
//                         ..PostInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_first_name() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 post(
//                     &mut client,
//                     &data_type,
//                     &PostInputs {
//                         caller_role: UserRole::Admin,
//                         first_name_validity: FirstNameValidity::Invalid,
//                         ..PostInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_last_name() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 post(
//                     &mut client,
//                     &data_type,
//                     &PostInputs {
//                         caller_role: UserRole::Admin,
//                         last_name_validity: LastNameValidity::Invalid,
//                         ..PostInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_password() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let passwords = vec![
//                 ".#Abcdef",
//                 "0#ABCDEF",
//                 "0#abcdef",
//                 "0Abcdefg",
//                 "0#Abcde f",
//                 "0#Abcde",
//             ];
//
//             let data_types = vec![DataType::Form, DataType::Json];
//
//             for data_type in data_types {
//                 for password in &passwords {
//                     post(
//                         &mut client,
//                         &data_type,
//                         &PostInputs {
//                             caller_role: UserRole::Admin,
//                             password_validity: PasswordValidity::Invalid,
//                             password: Some(password),
//                             ..PostInputs::default()
//                         },
//                     )
//                     .await;
//                 }
//             }
//         }
//     }
// }
//
// mod put {
//     use super::*;
//
//     #[derive(Default)]
//     pub struct PutInputs {
//         first_name_validity: FirstNameValidity,
//         last_name_validity: LastNameValidity,
//         email_validity: EmailValidity,
//         caller_role: UserRole,
//     }
//
//     async fn put(client: &mut TestClient, id: Uuid, data_type: &DataType, inputs: &PutInputs) {
//         let PutInputs {
//             first_name_validity,
//             last_name_validity,
//             email_validity,
//             caller_role,
//         } = inputs;
//
//         // Prepare inputs
//         let uniq = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
//
//         let first_name = match first_name_validity {
//             FirstNameValidity::Invalid => String::new(),
//             FirstNameValidity::Valid => format!("{uniq}-first-name"),
//         };
//
//         let last_name = match last_name_validity {
//             LastNameValidity::Invalid => String::new(),
//             LastNameValidity::Valid => format!("{uniq}-last-name"),
//         };
//
//         let email = match email_validity {
//             EmailValidity::Invalid => uniq,
//             EmailValidity::Valid => format!("{uniq}@email.com"),
//         };
//
//         // Call endpoint and get response
//         let response = match data_type {
//             DataType::Json => {
//                 let user = UpsertUserRequest {
//                     id: Some(id),
//                     first_name: Some(first_name.clone()),
//                     last_name: Some(last_name.clone()),
//                     email: Some(email.clone()),
//                     ..Default::default()
//                 };
//
//                 client.put("/api/users").json(&user).send().await
//             }
//
//             DataType::Form => {
//                 let user = [
//                     ("id", &id.to_string()),
//                     ("first_name", &first_name),
//                     ("last_name", &last_name),
//                     ("email", &email),
//                 ];
//
//                 client.put("/api/users").form(&user).send().await
//             }
//         };
//
//         // Check return code and values
//         let expected_status = match (
//             caller_role,
//             first_name_validity,
//             last_name_validity,
//             email_validity,
//         ) {
//             (UserRole::Normal, _, _, _) | (UserRole::Guest, _, _, _) => StatusCode::FORBIDDEN,
//
//             (_, FirstNameValidity::Invalid, _, _)
//             | (_, _, LastNameValidity::Invalid, _)
//             | (_, _, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,
//
//             _ => StatusCode::OK,
//         };
//
//         assert_eq!(response.status(), expected_status);
//
//         if expected_status == StatusCode::OK {
//             let user = response.json::<User>().await;
//             assert_eq!(user.first_name, first_name);
//             assert_eq!(user.last_name, last_name);
//             assert_eq!(user.email, email);
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn nominal() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 put(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PutInputs {
//                         caller_role: UserRole::Admin,
//                         ..PutInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn as_non_admin() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             // As normal user
//             auth::post::login_as_normal(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 put(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PutInputs {
//                         caller_role: UserRole::Normal,
//                         ..PutInputs::default()
//                     },
//                 )
//                 .await;
//             }
//
//             // As guest user
//             auth::post::login_as_guest(&mut client).await;
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 put(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PutInputs {
//                         caller_role: UserRole::Guest,
//                         ..PutInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_email() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 put(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PutInputs {
//                         email_validity: EmailValidity::Invalid,
//                         caller_role: UserRole::Admin,
//                         ..PutInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_first_name() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 put(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PutInputs {
//                         first_name_validity: FirstNameValidity::Invalid,
//                         caller_role: UserRole::Admin,
//                         ..PutInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
//
//     #[hook(setup, _)]
//     #[tokio::test]
//     #[serial]
//     pub async fn invalid_last_name() {
//         |client| async move {
//             let mut client = client.lock().await;
//
//             auth::post::login_as_admin(&mut client).await;
//
//             let user = post::post_normal_user(&mut client).await.unwrap();
//
//             for data_type in [DataType::Form, DataType::Json] {
//                 put(
//                     &mut client,
//                     user.id,
//                     &data_type,
//                     &PutInputs {
//                         last_name_validity: LastNameValidity::Invalid,
//                         caller_role: UserRole::Admin,
//                         ..PutInputs::default()
//                     },
//                 )
//                 .await;
//             }
//         }
//     }
// }
