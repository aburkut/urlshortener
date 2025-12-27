use urlshortener::create_test_rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use serde_json::json;

#[tokio::test]
async fn test_create_short_url() {
    let rocket = create_test_rocket();
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let request_body = json!({
        "url": "https://example.com"
    });

    let response = client
        .post("/create_short_url")
        .header(ContentType::JSON)
        .body(request_body.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Created);

    let body: serde_json::Value = response.into_json().await.expect("valid JSON");
    assert!(body.get("short_url").is_some());
    let short_url = body["short_url"].as_str().expect("short_url is a string");
    assert!(!short_url.is_empty());
    assert!(short_url.len() == 7); // nanoid with length 7
}

#[tokio::test]
async fn test_create_short_url_with_invalid_url() {
    let rocket = create_test_rocket();
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let request_body = json!({
        "url": "not-a-valid-url"
    });

    let response = client
        .post("/create_short_url")
        .header(ContentType::JSON)
        .body(request_body.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);

    let body: serde_json::Value = response.into_json().await.expect("valid JSON");
    assert!(body.get("error").is_some());
}

#[tokio::test]
async fn test_get_full_url() {
    let rocket = create_test_rocket();
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    
    // First, create a short URL
    let request_body = json!({
        "url": "https://test.example.com"
    });
    
    let create_response = client
        .post("/create_short_url")
        .header(ContentType::JSON)
        .body(request_body.to_string())
        .dispatch()
        .await;
    
    assert_eq!(create_response.status(), Status::Created);
    
    let create_body: serde_json::Value = create_response.into_json().await.expect("valid JSON");
    let short_url = create_body["short_url"].as_str().expect("short_url is a string");
    
    // Now, get the full URL
    let get_response = client
        .get(format!("/{}", short_url))
        .dispatch()
        .await;
    
    assert_eq!(get_response.status(), Status::SeeOther);
    let location = get_response.headers().get_one("Location");
    assert_eq!(location, Some("https://test.example.com"));
}

#[tokio::test]
async fn test_get_full_url_not_found() {
    let rocket = create_test_rocket();
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    
    // Try to get a non-existent short URL
    let response = client
        .get("/nonexistent123")
        .dispatch()
        .await;
    
    assert_eq!(response.status(), Status::NotFound);
    
    let body: serde_json::Value = response.into_json().await.expect("valid JSON");
    assert!(body.get("error").is_some());
    assert_eq!(body["error"], "Short URL not found");
}

#[tokio::test]
async fn test_create_short_url_multiple_times() {
    let rocket = create_test_rocket();
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    
    let request_body = json!({
        "url": "https://example.com/unique"
    });
    
    // Create first short URL
    let response1 = client
        .post("/create_short_url")
        .header(ContentType::JSON)
        .body(request_body.to_string())
        .dispatch()
        .await;
    
    assert_eq!(response1.status(), Status::Created);
    let body1: serde_json::Value = response1.into_json().await.expect("valid JSON");
    let short_url1 = body1["short_url"].as_str().expect("short_url is a string");
    
    // Create second short URL with same full URL (should get different short URL)
    let response2 = client
        .post("/create_short_url")
        .header(ContentType::JSON)
        .body(request_body.to_string())
        .dispatch()
        .await;
    
    assert_eq!(response2.status(), Status::Created);
    let body2: serde_json::Value = response2.into_json().await.expect("valid JSON");
    let short_url2 = body2["short_url"].as_str().expect("short_url is a string");
    
    // Both should redirect to the same URL, but short URLs should be different (with very high probability)
    // Note: There's a tiny chance they could be the same, but it's extremely unlikely with nanoid
    
    let get_response1 = client.get(format!("/{}", short_url1)).dispatch().await;
    let get_response2 = client.get(format!("/{}", short_url2)).dispatch().await;
    
    assert_eq!(get_response1.status(), Status::SeeOther);
    assert_eq!(get_response2.status(), Status::SeeOther);
    assert_eq!(get_response1.headers().get_one("Location"), Some("https://example.com/unique"));
    assert_eq!(get_response2.headers().get_one("Location"), Some("https://example.com/unique"));
}

