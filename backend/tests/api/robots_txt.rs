// tests/api/robots_txt.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn robots_txt_returns_200_with_correct_content() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/robots.txt", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Missing content-type header");
    assert!(content_type.to_str().unwrap().starts_with("text/plain"));

    let body = response.text().await.expect("Failed to read response body.");
    assert!(body.contains("User-agent: *"));
    assert!(body.contains("Allow: /"));
}
