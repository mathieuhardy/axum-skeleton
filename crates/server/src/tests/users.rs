use rand::distributions::{Alphanumeric, DistString};
use serial_test::serial;
use test_utils::*;
use urlencoding::encode;
use uuid::Uuid;

use database::models::users::*;

use crate::tests::common::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

async fn first_user(client: &TestClient) -> User {
    let response = client.get("/api/users").send().await;
    assert_eq!(response.status(), StatusCode::OK);

    let users = response.json::<Vec<User>>().await;
    assert!(!users.is_empty());

    users[0].clone()
}

mod delete {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn by_id() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users").send().await;
            assert_eq!(response.status(), StatusCode::OK);

            let users = response.json::<Vec<User>>().await;
            assert!(!users.is_empty());

            let response = client
                .delete(format!("/api/users/{}", users[0].id))
                .send()
                .await;

            assert_eq!(response.status(), StatusCode::NO_CONTENT);
        }
    }
}

mod get {
    use super::*;

    /// TEST_PLAN: /TC/USERS/GET/ME
    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn me() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users/me").send().await;
            assert_eq!(response.status(), StatusCode::OK);

            let user = response.json::<User>().await;
            assert_eq!(user.first_name, "John");
            assert_eq!(user.last_name, "Doe");
            assert_eq!(user.email, "john@doe.com");
        }
    }

    /// TEST_PLAN: /TC/USERS/GET/ALL
    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn all() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users").send().await;
            assert_eq!(response.status(), StatusCode::OK);

            let users = response.json::<Vec<User>>().await;
            assert_eq!(users.len(), 3);
            assert!(users.iter().any(|e| e.first_name == "Giga"
                && e.last_name == "Chad"
                && e.email == "giga@chad.com"));
            assert!(users.iter().any(|e| e.first_name == "John"
                && e.last_name == "Doe"
                && e.email == "john@doe.com"));
            assert!(users.iter().any(|e| e.first_name == "Jane"
                && e.last_name == "Doe"
                && e.email == "jane@doe.com"));
        }
    }

    /// TEST_PLAN: /TC/USERS/GET/FILTERED
    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn by_filters() {
        |client| async move {
            let client = client.lock().await;

            // By name
            let response = client
                .get(format!("/api/users?first_name={}", encode("John")))
                .send()
                .await;
            assert_eq!(response.status(), StatusCode::OK);

            let users = response.json::<Vec<User>>().await;
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].first_name, "John");
            assert_eq!(users[0].last_name, "Doe");
            assert_eq!(users[0].email, "john@doe.com");

            // By name (not found)
            let response = client.get("/api/users?first_name=404").send().await;
            assert_eq!(response.status(), StatusCode::NOT_FOUND);

            // By email
            let response = client.get("/api/users?email=john@doe.com").send().await;
            assert_eq!(response.status(), StatusCode::OK);

            let users = response.json::<Vec<User>>().await;
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].first_name, "John");
            assert_eq!(users[0].last_name, "Doe");
            assert_eq!(users[0].email, "john@doe.com");

            // By email (not found)
            let response = client.get("/api/users?email=404").send().await;
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }
    }

    /// TEST_PLAN: /TC/USERS/GET/ID
    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn by_id() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users").send().await;
            assert_eq!(response.status(), StatusCode::OK);

            let users = response.json::<Vec<User>>().await;
            assert_eq!(users.len(), 3);

            let response = client
                .get(format!("/api/users/{}", users[0].id))
                .send()
                .await;

            assert_eq!(response.status(), StatusCode::OK);

            let user = response.json::<User>().await;
            assert_eq!(users[0], user);
        }
    }
}

mod patch {
    use super::*;

    async fn test_patch(
        client: &TestClient,
        id: Uuid,
        data_type: DataType,
        first_name_validity: FirstNameValidity,
        last_name_validity: LastNameValidity,
        email_validity: EmailValidity,
    ) {
        // Prepare inputs
        let uniq = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let first_name = match first_name_validity {
            FirstNameValidity::Invalid => String::new(),
            FirstNameValidity::Valid => format!("{uniq}-first-name"),
        };

        let last_name = match last_name_validity {
            LastNameValidity::Invalid => String::new(),
            LastNameValidity::Valid => format!("{uniq}-last-name"),
        };

        let email = match email_validity {
            EmailValidity::Invalid => uniq,
            EmailValidity::Valid => format!("{uniq}@email.com"),
        };

        // Call endpoint and get response
        let response = match data_type {
            DataType::Json => {
                let user = UserRequest {
                    first_name: Some(first_name.clone()),
                    last_name: Some(last_name.clone()),
                    email: Some(email.clone()),
                    ..UserRequest::default()
                };

                client
                    .patch(format!("/api/users/{}", id))
                    .json(&user)
                    .send()
                    .await
            }

            DataType::Form => {
                let user = [
                    ("first_name", &first_name),
                    ("last_name", &last_name),
                    ("email", &email),
                ];

                client
                    .patch(format!("/api/users/{}", id))
                    .form(&user)
                    .send()
                    .await
            }
        };

        // Check return code and values
        let expected_status = match (first_name_validity, last_name_validity, email_validity) {
            (FirstNameValidity::Invalid, _, _)
            | (_, LastNameValidity::Invalid, _)
            | (_, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,

            _ => StatusCode::OK,
        };

        assert_eq!(response.status(), expected_status);

        if expected_status == StatusCode::OK {
            let user = response.json::<User>().await;
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.email, email);
        }
    }

    async fn test_patch_password(
        client: &TestClient,
        id: &Uuid,
        data_type: DataType,
        current_password_validity: PasswordValidity,
        current_password: Option<&str>,
        password_validity: PasswordValidity,
        password: Option<&str>,
    ) {
        let current_password = current_password.unwrap_or("").to_string();

        let password = match password_validity {
            PasswordValidity::Invalid => password.unwrap_or("").to_string(),
            PasswordValidity::Valid => "0#Abcdef".to_string(),
        };

        // Call endpoint and get response
        let response = match data_type {
            DataType::Json => {
                let request = PasswordUpdateRequest {
                    current: current_password,
                    new: password,
                };

                client
                    .patch(format!("/api/users/{}/password", id))
                    .json(&request)
                    .send()
                    .await
            }

            DataType::Form => {
                let request = [("current", &current_password), ("new", &password)];

                client
                    .patch(format!("/api/users/{}/password", id))
                    .form(&request)
                    .send()
                    .await
            }
        };

        // Check return code and values
        let expected_status = match (current_password_validity, password_validity) {
            (PasswordValidity::Invalid, _) => StatusCode::FORBIDDEN,
            (_, PasswordValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,

            _ => StatusCode::OK,
        };

        assert_eq!(response.status(), expected_status);
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn nominal() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_patch(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Valid,
                    EmailValidity::Valid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_email() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_patch(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Valid,
                    EmailValidity::Invalid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_first_name() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_patch(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Invalid,
                    LastNameValidity::Valid,
                    EmailValidity::Valid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_last_name() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_patch(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Invalid,
                    EmailValidity::Valid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn set_password() {
        |client| async move {
            let client = client.lock().await;

            // Create a user to update
            let uniq = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

            let request = UserRequest {
                first_name: Some(format!("{uniq}-first-name")),
                last_name: Some(format!("{uniq}-last-name")),
                email: Some(format!("{uniq}@email.com")),
                password: Some("0#Abcdef".to_string()),
                ..UserRequest::default()
            };

            let response = client.post("/api/users").json(&request).send().await;

            assert_eq!(response.status(), StatusCode::CREATED);

            let user = response.json::<User>().await;

            let data_types = vec![DataType::Form, DataType::Json];

            // Invalid current password
            for data_type in &data_types {
                test_patch_password(
                    &client,
                    &user.id,
                    data_type.clone(),
                    PasswordValidity::Invalid,
                    Some("INVALID_PASSWORD"),
                    PasswordValidity::Valid,
                    None,
                )
                .await;
            }

            // Invalid new passwords
            let passwords = vec![
                ".#Abcdef",
                "0#ABCDEF",
                "0#abcdef",
                "0Abcdefg",
                "0#Abcde f",
                "0#Abcde",
            ];

            for data_type in &data_types {
                for password in &passwords {
                    test_patch_password(
                        &client,
                        &user.id,
                        data_type.clone(),
                        PasswordValidity::Valid,
                        None,
                        PasswordValidity::Invalid,
                        Some(password),
                    )
                    .await;
                }
            }

            // Nominal
            for data_type in data_types {
                test_patch_password(
                    &client,
                    &user.id,
                    data_type.clone(),
                    PasswordValidity::Valid,
                    Some("0#Abcdef"),
                    PasswordValidity::Valid,
                    None,
                )
                .await;
            }
        }
    }
}

mod post {
    use super::*;

    async fn test_post(
        client: &TestClient,
        data_type: DataType,
        first_name_validity: FirstNameValidity,
        last_name_validity: LastNameValidity,
        email_validity: EmailValidity,
        password_validity: PasswordValidity,
        password: Option<&str>,
    ) {
        // Prepare inputs
        let uniq = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let first_name = match first_name_validity {
            FirstNameValidity::Invalid => String::new(),
            FirstNameValidity::Valid => format!("{uniq}-first-name"),
        };

        let last_name = match last_name_validity {
            LastNameValidity::Invalid => String::new(),
            LastNameValidity::Valid => format!("{uniq}-last-name"),
        };

        let email = match email_validity {
            EmailValidity::Invalid => uniq,
            EmailValidity::Valid => format!("{uniq}@email.com"),
        };

        let password = match password_validity {
            PasswordValidity::Invalid => password.unwrap_or("").to_string(),
            PasswordValidity::Valid => "0#Abcdef".to_string(),
        };

        // Call endpoint and get response
        let response = match data_type {
            DataType::Json => {
                let user = UserRequest {
                    first_name: Some(first_name.clone()),
                    last_name: Some(last_name.clone()),
                    email: Some(email.clone()),
                    password: Some(password),
                    ..UserRequest::default()
                };

                client.post("/api/users").json(&user).send().await
            }

            DataType::Form => {
                let user = [
                    ("first_name", &first_name),
                    ("last_name", &last_name),
                    ("email", &email),
                    ("password", &password),
                ];

                client.post("/api/users").form(&user).send().await
            }
        };

        // Check return code and values
        let expected_status = match (
            first_name_validity,
            last_name_validity,
            email_validity,
            password_validity,
        ) {
            (FirstNameValidity::Invalid, _, _, _)
            | (_, LastNameValidity::Invalid, _, _)
            | (_, _, EmailValidity::Invalid, _)
            | (_, _, _, PasswordValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,

            _ => StatusCode::CREATED,
        };

        assert_eq!(response.status(), expected_status);

        if expected_status == StatusCode::CREATED {
            let user = response.json::<User>().await;
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.email, email);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn nominal() {
        |client| async move {
            let client = client.lock().await;

            for data_type in [DataType::Form, DataType::Json] {
                test_post(
                    &client,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Valid,
                    EmailValidity::Valid,
                    PasswordValidity::Valid,
                    None,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_email() {
        |client| async move {
            let client = client.lock().await;

            for data_type in [DataType::Form, DataType::Json] {
                test_post(
                    &client,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Valid,
                    EmailValidity::Invalid,
                    PasswordValidity::Valid,
                    None,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_first_name() {
        |client| async move {
            let client = client.lock().await;

            for data_type in [DataType::Form, DataType::Json] {
                test_post(
                    &client,
                    data_type,
                    FirstNameValidity::Invalid,
                    LastNameValidity::Valid,
                    EmailValidity::Valid,
                    PasswordValidity::Valid,
                    None,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_last_name() {
        |client| async move {
            let client = client.lock().await;

            for data_type in [DataType::Form, DataType::Json] {
                test_post(
                    &client,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Invalid,
                    EmailValidity::Valid,
                    PasswordValidity::Valid,
                    None,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_password() {
        |client| async move {
            let client = client.lock().await;

            let passwords = vec![
                ".#Abcdef",
                "0#ABCDEF",
                "0#abcdef",
                "0Abcdefg",
                "0#Abcde f",
                "0#Abcde",
            ];

            let data_types = vec![DataType::Form, DataType::Json];

            for data_type in data_types {
                for password in &passwords {
                    test_post(
                        &client,
                        data_type.clone(),
                        FirstNameValidity::Valid,
                        LastNameValidity::Valid,
                        EmailValidity::Valid,
                        PasswordValidity::Invalid,
                        Some(password),
                    )
                    .await;
                }
            }
        }
    }
}

mod put {
    use super::*;

    async fn test_put(
        client: &TestClient,
        id: Uuid,
        data_type: DataType,
        first_name_validity: FirstNameValidity,
        last_name_validity: LastNameValidity,
        email_validity: EmailValidity,
    ) {
        // Prepare inputs
        let uniq = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let first_name = match first_name_validity {
            FirstNameValidity::Invalid => String::new(),
            FirstNameValidity::Valid => format!("{uniq}-first-name"),
        };

        let last_name = match last_name_validity {
            LastNameValidity::Invalid => String::new(),
            LastNameValidity::Valid => format!("{uniq}-last-name"),
        };

        let email = match email_validity {
            EmailValidity::Invalid => uniq,
            EmailValidity::Valid => format!("{uniq}@email.com"),
        };

        // Call endpoint and get response
        let response = match data_type {
            DataType::Json => {
                let user = UserRequest {
                    id: Some(id),
                    first_name: Some(first_name.clone()),
                    last_name: Some(last_name.clone()),
                    email: Some(email.clone()),
                    ..UserRequest::default()
                };

                client.put("/api/users").json(&user).send().await
            }

            DataType::Form => {
                let user = [
                    ("id", &id.to_string()),
                    ("first_name", &first_name),
                    ("last_name", &last_name),
                    ("email", &email),
                ];

                client.put("/api/users").form(&user).send().await
            }
        };

        // Check return code and values
        let expected_status = match (first_name_validity, last_name_validity, email_validity) {
            (FirstNameValidity::Invalid, _, _)
            | (_, LastNameValidity::Invalid, _)
            | (_, _, EmailValidity::Invalid) => StatusCode::UNPROCESSABLE_ENTITY,

            _ => StatusCode::OK,
        };

        assert_eq!(response.status(), expected_status);

        if expected_status == StatusCode::OK {
            let user = response.json::<User>().await;
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.email, email);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn nominal() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_put(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Valid,
                    EmailValidity::Valid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_email() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_put(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Valid,
                    EmailValidity::Invalid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_first_name() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_put(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Invalid,
                    LastNameValidity::Valid,
                    EmailValidity::Valid,
                )
                .await;
            }
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    pub async fn invalid_last_name() {
        |client| async move {
            let client = client.lock().await;

            let user = first_user(&client).await;

            for data_type in [DataType::Form, DataType::Json] {
                test_put(
                    &client,
                    user.id,
                    data_type,
                    FirstNameValidity::Valid,
                    LastNameValidity::Invalid,
                    EmailValidity::Valid,
                )
                .await;
            }
        }
    }
}
