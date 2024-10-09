use std::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:4100";

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind address");
    let port = listener.local_addr().unwrap().port();

    let server =  isotopes::run(listener).await.expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn health_check_works() {

    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let resp = client.get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request");
    // Assert
    assert!(resp.status().is_success());
    assert_eq!(Some(19), resp.content_length());
}

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn sibscribe_returns_a_400_when_date_is_missing()  {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_messages) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await.
            expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16());
    }



}
