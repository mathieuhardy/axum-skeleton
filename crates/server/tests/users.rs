use rand::distributions::{Alphanumeric, DistString};
use serial_test::serial;
use std::sync::Arc;
use test_utils::*;
use tokio::sync::Mutex;
use urlencoding::encode;
use uuid::Uuid;

use database::models::users::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

#[derive(Clone)]
enum DataType {
    Form,
    Json,
}

enum EmailValidity {
    Invalid,
    Valid,
}

enum FirstNameValidity {
    Invalid,
    Valid,
}

enum LastNameValidity {
    Invalid,
    Valid,
}

enum PasswordValidity {
    Invalid,
    Valid,
}

async fn first_user(client: &Arc<Mutex<TestClient>>) -> UserResponse {
    let client = client.lock().await;

    let response = client.get("/api/users").send().await.unwrap();
    assert_eq!(response.status(), test_utils::StatusCode::OK);

    let users = response.json::<Vec<UserResponse>>().await.unwrap();
    assert!(!users.is_empty());

    users[0].clone()
}

mod delete {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn by_id() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();
            assert!(!users.is_empty());

            let response = client
                .delete(format!("/api/users/{}", users[0].id))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), test_utils::StatusCode::NO_CONTENT);
        }
    }
}

mod get {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn me() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users/me").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "John");
            assert_eq!(user.last_name, "Doe");
            assert_eq!(user.email, "john@doe.com");
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn all() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();
            assert_eq!(users.len(), 2);
            assert!(users.iter().any(|e| e.first_name == "John"
                && e.last_name == "Doe"
                && e.email == "john@doe.com"));
            assert!(users.iter().any(|e| e.first_name == "Jane"
                && e.last_name == "Doe"
                && e.email == "jane@doe.com"));
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn by_filters() {
        |client| async move {
            let client = client.lock().await;

            // By name
            let response = client
                .get(format!("/api/users?first_name={}", encode("John")))
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].first_name, "John");
            assert_eq!(users[0].last_name, "Doe");
            assert_eq!(users[0].email, "john@doe.com");

            // By name (not found)
            let response = client
                .get("/api/users?first_name=404")
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::NOT_FOUND);

            // By email
            let response = client
                .get("/api/users?email=john@doe.com")
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].first_name, "John");
            assert_eq!(users[0].last_name, "Doe");
            assert_eq!(users[0].email, "john@doe.com");

            // By email (not found)
            let response = client.get("/api/users?email=404").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::NOT_FOUND);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn by_id() {
        |client| async move {
            let client = client.lock().await;

            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();
            assert_eq!(users.len(), 2);

            let response = client
                .get(format!("/api/users/{}", users[0].id))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(users[0], user);
        }
    }
}

// TODO: test update with None values
mod patch {
    use super::*;

    async fn test_patch(
        client: &Arc<Mutex<TestClient>>,
        id: Uuid,
        data_type: DataType,
        first_name_validity: FirstNameValidity,
        last_name_validity: LastNameValidity,
        email_validity: EmailValidity,
    ) {
        let client = client.lock().await;

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
            EmailValidity::Invalid => format!("{uniq}"),
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
                    .unwrap()
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
                    .unwrap()
            }
        };

        // Check return code and values
        let expected_status = match (first_name_validity, last_name_validity, email_validity) {
            (FirstNameValidity::Invalid, _, _)
            | (_, LastNameValidity::Invalid, _)
            | (_, _, EmailValidity::Invalid) => test_utils::StatusCode::UNPROCESSABLE_ENTITY,

            _ => test_utils::StatusCode::OK,
        };

        assert_eq!(response.status(), expected_status);

        if expected_status == test_utils::StatusCode::OK {
            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.email, email);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn nominal() {
        |client| async move {
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
    async fn invalid_email() {
        |client| async move {
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
    async fn invalid_first_name() {
        |client| async move {
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
    async fn invalid_last_name() {
        |client| async move {
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
}

// TODO: test post with None values
// TODO: set_password => fetch database to get the actual password
mod post {
    use super::*;

    async fn test_post(
        client: &Arc<Mutex<TestClient>>,
        data_type: DataType,
        first_name_validity: FirstNameValidity,
        last_name_validity: LastNameValidity,
        email_validity: EmailValidity,
        password_validity: PasswordValidity,
        password: Option<&str>,
    ) {
        let client = client.lock().await;

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
            EmailValidity::Invalid => format!("{uniq}"),
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

                client.post("/api/users").json(&user).send().await.unwrap()
            }

            DataType::Form => {
                let user = [
                    ("first_name", &first_name),
                    ("last_name", &last_name),
                    ("email", &email),
                    ("password", &password),
                ];

                client.post("/api/users").form(&user).send().await.unwrap()
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
            | (_, _, _, PasswordValidity::Invalid) => test_utils::StatusCode::UNPROCESSABLE_ENTITY,

            _ => test_utils::StatusCode::CREATED,
        };

        assert_eq!(response.status(), expected_status);

        if expected_status == test_utils::StatusCode::CREATED {
            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.email, email);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn nominal() {
        |client| async move {
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
    async fn invalid_email() {
        |client| async move {
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
    async fn invalid_first_name() {
        |client| async move {
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
    async fn invalid_last_name() {
        |client| async move {
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
    async fn invalid_password() {
        |client| async move {
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

// TODO: test put with None values
mod put {
    use super::*;

    async fn test_put(
        client: &Arc<Mutex<TestClient>>,
        id: Uuid,
        data_type: DataType,
        first_name_validity: FirstNameValidity,
        last_name_validity: LastNameValidity,
        email_validity: EmailValidity,
    ) {
        let client = client.lock().await;

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
            EmailValidity::Invalid => format!("{uniq}"),
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

                client.put("/api/users").json(&user).send().await.unwrap()
            }

            DataType::Form => {
                let user = [
                    ("id", &id.to_string()),
                    ("first_name", &first_name),
                    ("last_name", &last_name),
                    ("email", &email),
                ];

                client.put("/api/users").form(&user).send().await.unwrap()
            }
        };

        // Check return code and values
        let expected_status = match (first_name_validity, last_name_validity, email_validity) {
            (FirstNameValidity::Invalid, _, _)
            | (_, LastNameValidity::Invalid, _)
            | (_, _, EmailValidity::Invalid) => test_utils::StatusCode::UNPROCESSABLE_ENTITY,

            _ => test_utils::StatusCode::OK,
        };

        assert_eq!(response.status(), expected_status);

        if expected_status == test_utils::StatusCode::OK {
            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.email, email);
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn nominal() {
        |client| async move {
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
    async fn invalid_email() {
        |client| async move {
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
    async fn invalid_first_name() {
        |client| async move {
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
    async fn invalid_last_name() {
        |client| async move {
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
