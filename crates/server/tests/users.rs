use serial_test::serial;
use test_utils::*;
use urlencoding::encode;

use database::models::users::{User, UserRequest};

async fn setup() -> TestClient {
    init_server().await.unwrap()
}

mod get {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn me() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                let response = client.get("/api/users/me").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "John Doe");
                assert_eq!(user.email, "john@doe.com");
            },
            no_teardown,
        )
        .await;
    }

    #[tokio::test]
    #[serial]
    async fn all() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                let response = client.get("/api/users").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let users = response.json::<Vec<User>>().await.unwrap();
                assert_eq!(users.len(), 2);
                assert!(users.iter().any(|e| e.name == "John Doe"));
                assert!(users.iter().any(|e| e.email == "john@doe.com"));
                assert!(users.iter().any(|e| e.name == "Jane Doe"));
                assert!(users.iter().any(|e| e.email == "jane@doe.com"));
            },
            no_teardown,
        )
        .await;
    }

    #[tokio::test]
    #[serial]
    async fn by_filters() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                // By name
                let response = client
                    .get(format!("/api/users?name={}", encode("John Doe")))
                    .send()
                    .await
                    .unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let users = response.json::<Vec<User>>().await.unwrap();
                assert_eq!(users.len(), 1);
                assert_eq!(users[0].name, "John Doe");
                assert_eq!(users[0].email, "john@doe.com");

                // By name (not found)
                let response = client.get("/api/users?name=404").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::NOT_FOUND);

                // By email
                let response = client
                    .get("/api/users?email=john@doe.com")
                    .send()
                    .await
                    .unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let users = response.json::<Vec<User>>().await.unwrap();
                assert_eq!(users.len(), 1);
                assert_eq!(users[0].name, "John Doe");
                assert_eq!(users[0].email, "john@doe.com");

                // By email (not found)
                let response = client.get("/api/users?email=404").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::NOT_FOUND);
            },
            no_teardown,
        )
        .await;
    }

    #[tokio::test]
    #[serial]
    async fn by_id() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                let response = client.get("/api/users").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let users = response.json::<Vec<User>>().await.unwrap();
                assert_eq!(users.len(), 2);

                let response = client
                    .get(format!("/api/users/{}", users[0].id))
                    .send()
                    .await
                    .unwrap();

                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(users[0], user);
            },
            no_teardown,
        )
        .await;
    }
}

mod patch {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn update_from_form() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                // Get the list of users
                let response = client.get("/api/users").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let users = response.json::<Vec<User>>().await.unwrap();

                // Create a request to be sent
                let user = [("name", "New User"), ("email", "new@user.com")];

                // Update
                let response = client
                    .patch(format!("/api/users/{}", users[0].id))
                    .form(&user)
                    .send()
                    .await
                    .unwrap();

                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User");
                assert_eq!(user.email, "new@user.com");
            },
            no_teardown,
        )
        .await;
    }

    #[tokio::test]
    #[serial]
    async fn update_from_json() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                // Get the list of users
                let response = client.get("/api/users").send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let users = response.json::<Vec<User>>().await.unwrap();

                // Create a request to be sent
                let user = User {
                    name: "New User".to_string(),
                    email: "new@user.com".to_string(),
                    ..User::default()
                };

                // Update
                let response = client
                    .patch(format!("/api/users/{}", users[0].id))
                    .json(&user)
                    .send()
                    .await
                    .unwrap();

                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User");
                assert_eq!(user.email, "new@user.com");
            },
            no_teardown,
        )
        .await;
    }
}

mod post {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn create_from_form() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                let user = [("name", "New User"), ("email", "new@user.com")];

                let response = client.post("/api/users").form(&user).send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User");
                assert_eq!(user.email, "new@user.com");
            },
            no_teardown,
        )
        .await;
    }

    #[tokio::test]
    #[serial]
    async fn create_from_json() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                let user = User {
                    name: "New User".to_string(),
                    email: "new@user.com".to_string(),
                    ..User::default()
                };

                let response = client.post("/api/users").json(&user).send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User");
                assert_eq!(user.email, "new@user.com");
            },
            no_teardown,
        )
        .await;
    }
}

mod put {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn create_from_form() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                // Insert
                let user = [("name", "New User"), ("email", "new@user.com")];

                let response = client.put("/api/users").form(&user).send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User");
                assert_eq!(user.email, "new@user.com");

                // Update
                let user = [
                    ("id", user.id.to_string()),
                    ("name", "New User (2)".to_string()),
                    ("email", "new@user2.com".to_string()),
                ];

                let response = client.put("/api/users").form(&user).send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User (2)");
                assert_eq!(user.email, "new@user2.com");
            },
            no_teardown,
        )
        .await;
    }

    #[tokio::test]
    #[serial]
    async fn create_from_json() {
        run_test(
            setup,
            |client| async move {
                let client = client.lock().unwrap();

                // Insert
                let user = UserRequest {
                    name: Some("New User".to_string()),
                    email: Some("new@user.com".to_string()),
                    ..UserRequest::default()
                };

                let response = client.put("/api/users").json(&user).send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User");
                assert_eq!(user.email, "new@user.com");

                // Update
                let user = UserRequest {
                    id: Some(user.id),
                    name: Some("New User (2)".to_string()),
                    email: Some("new@user2.com".to_string()),
                };

                let response = client.put("/api/users").form(&user).send().await.unwrap();
                assert_eq!(response.status(), test_utils::StatusCode::OK);

                let user = response.json::<User>().await.unwrap();
                assert_eq!(user.name, "New User (2)");
                assert_eq!(user.email, "new@user2.com");
            },
            no_teardown,
        )
        .await;
    }
}
