use serial_test::serial;
use test_utils::init_server;
use urlencoding::encode;

use database::models::users::User;

mod get {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn me() {
        let client = init_server().await.unwrap();

        let response = client.get("/api/users/me").send().await.unwrap();
        assert_eq!(response.status(), test_utils::StatusCode::OK);

        let user = response.json::<User>().await.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@doe.com");
    }

    #[tokio::test]
    #[serial]
    async fn by_filters() {
        let client = init_server().await.unwrap();

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
    }
}

mod post {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn create_from_form() {
        let client = init_server().await.unwrap();

        let user = [("name", "John Doe"), ("email", "john@doe.com")];

        let response = client.post("/api/users").form(&user).send().await.unwrap();
        assert_eq!(response.status(), test_utils::StatusCode::OK);

        let user = response.json::<User>().await.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@doe.com");
    }

    #[tokio::test]
    #[serial]
    async fn create_from_json() {
        let client = init_server().await.unwrap();

        let user = User {
            name: "John Doe".to_string(),
            email: "john@doe.com".to_string(),
            ..User::default()
        };

        let response = client.post("/api/users").json(&user).send().await.unwrap();
        assert_eq!(response.status(), test_utils::StatusCode::OK);

        let user = response.json::<User>().await.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@doe.com");
    }
}
