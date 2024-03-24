fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to find a port.");
    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let data = "name=Pietro%20Agnoli&email=thecaptaincraken%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let test_cases = [
        ("name=Pietro%20Agnoli", "missing the email."),
        ("email=thecaptaincraken%40gmail.com", "missing the name."),
        ("", "missing both the email and name."),
    ];

    for (invalid_body, invalid_body_reason) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API was supposed to fail with a 400 code on a payload that was {}",
            invalid_body_reason
        )
    }
}
