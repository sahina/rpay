mod helper;

#[actix_rt::test]
async fn health_check() {
    let address = helper::spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn landing_check() {
    let address = helper::spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}