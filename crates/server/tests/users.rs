use serial_test::serial;
use test_utils::*;
use urlencoding::encode;

use database::models::users::*;

async fn setup() -> TestClient {
    init_server().await.unwrap()
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
// TODO: test passwords
mod patch {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn update_from_form() {
        |client| async move {
            let client = client.lock().await;

            // Get the list of users
            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();

            // Create a request to be sent
            let user = [
                ("first_name", "New"),
                ("last_name", "User"),
                ("email", "new@user.com"),
                ("password", "blablabla"),
            ];

            // Update
            let response = client
                .patch(format!("/api/users/{}", users[0].id))
                .form(&user)
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User");
            assert_eq!(user.email, "new@user.com");
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn update_from_json() {
        |client| async move {
            let client = client.lock().await;

            // Get the list of users
            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();

            // Create a request to be sent
            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            // Update
            let response = client
                .patch(format!("/api/users/{}", users[0].id))
                .json(&user)
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User");
            assert_eq!(user.email, "new@user.com");
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_email() {
        |client| async move {
            let client = client.lock().await;

            // Get the list of users
            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();

            // Create a request to be sent
            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("newuser.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            // Update
            let response = client
                .patch(format!("/api/users/{}", users[0].id))
                .json(&user)
                .send()
                .await
                .unwrap();

            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_first_name() {
        |client| async move {
            let client = client.lock().await;

            // Get the list of users
            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();

            // Create a request to be sent
            let user = UserRequest {
                first_name: Some("".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            // Update
            let response = client
                .patch(format!("/api/users/{}", users[0].id))
                .json(&user)
                .send()
                .await
                .unwrap();

            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_last_name() {
        |client| async move {
            let client = client.lock().await;

            // Get the list of users
            let response = client.get("/api/users").send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let users = response.json::<Vec<UserResponse>>().await.unwrap();

            // Create a request to be sent
            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            // Update
            let response = client
                .patch(format!("/api/users/{}", users[0].id))
                .json(&user)
                .send()
                .await
                .unwrap();

            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }
}

// TODO: test post with None values
mod post {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn create_from_form() {
        |client| async move {
            let client = client.lock().await;

            let user = [
                ("first_name", "New"),
                ("last_name", "User"),
                ("email", "new@user.com"),
                ("password", "blablabla"),
            ];

            let response = client.post("/api/users").form(&user).send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::CREATED);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User");
            assert_eq!(user.email, "new@user.com");
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn create_from_json() {
        |client| async move {
            let client = client.lock().await;

            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.post("/api/users").json(&user).send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::CREATED);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User");
            assert_eq!(user.email, "new@user.com");
        }
    }

    // TODO: test patterns
    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_password() {
        |client| async move {
            let client = client.lock().await;

            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: None,
                ..UserRequest::default()
            };

            let response = client.post("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::INTERNAL_SERVER_ERROR
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_email() {
        |client| async move {
            let client = client.lock().await;

            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("newuser.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.post("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_first_name() {
        |client| async move {
            let client = client.lock().await;

            let user = UserRequest {
                first_name: Some("".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.post("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_last_name() {
        |client| async move {
            let client = client.lock().await;

            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.post("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }
}

// TODO: test put with None values
// TODO: test passwords
mod put {
    use super::*;

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn create_from_form() {
        |client| async move {
            let client = client.lock().await;

            // Insert
            let user = [
                ("first_name", "New"),
                ("last_name", "User"),
                ("email", "new@user.com"),
                ("password", "blablabla"),
            ];

            let response = client.put("/api/users").form(&user).send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::CREATED);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User");
            assert_eq!(user.email, "new@user.com");

            // Update
            let user = [
                ("id", user.id.to_string()),
                ("first_name", "New".to_string()),
                ("last_name", "User (2)".to_string()),
                ("email", "new@user2.com".to_string()),
                ("password", "new_password".to_string()),
            ];

            let response = client.put("/api/users").form(&user).send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User (2)");
            assert_eq!(user.email, "new@user2.com");
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn create_from_json() {
        |client| async move {
            let client = client.lock().await;

            // Insert
            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.put("/api/users").json(&user).send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::CREATED);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User");
            assert_eq!(user.email, "new@user.com");

            // Update
            let user = UserRequest {
                id: Some(user.id),
                first_name: Some("New".to_string()),
                last_name: Some("User (2)".to_string()),
                email: Some("new@user2.com".to_string()),
                password: Some("new_password".to_string()),
            };

            let response = client.put("/api/users").form(&user).send().await.unwrap();
            assert_eq!(response.status(), test_utils::StatusCode::OK);

            let user = response.json::<UserResponse>().await.unwrap();
            assert_eq!(user.first_name, "New");
            assert_eq!(user.last_name, "User (2)");
            assert_eq!(user.email, "new@user2.com");
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_email() {
        |client| async move {
            let client = client.lock().await;

            // Insert
            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("User".to_string()),
                email: Some("newuser.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.put("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_first_name() {
        |client| async move {
            let client = client.lock().await;

            // Insert
            let user = UserRequest {
                first_name: Some("".to_string()),
                last_name: Some("User".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.put("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[hook(setup, _)]
    #[tokio::test]
    #[serial]
    async fn invalid_last_name() {
        |client| async move {
            let client = client.lock().await;

            // Insert
            let user = UserRequest {
                first_name: Some("New".to_string()),
                last_name: Some("".to_string()),
                email: Some("new@user.com".to_string()),
                password: Some("blablabla".to_string()),
                ..UserRequest::default()
            };

            let response = client.put("/api/users").json(&user).send().await.unwrap();
            assert_eq!(
                response.status(),
                test_utils::StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }
}
