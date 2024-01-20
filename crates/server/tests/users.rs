use test_utils::init_server;

#[tokio::test]
async fn me() {
    let client = init_server().await.unwrap();

    let response = client.get("/api/users/me").send().await.unwrap();
    assert_eq!(response.status(), test_utils::StatusCode::OK);

    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello you");
}
