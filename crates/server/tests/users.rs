use serial_test::serial;
use test_utils::init_server;

use database::models::users::User;

#[tokio::test]
#[serial]
async fn me() {
    let client = init_server().await.unwrap();

    let response = client.get("/api/users/me").send().await.unwrap();
    assert_eq!(response.status(), test_utils::StatusCode::OK);

    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello you");
}

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
