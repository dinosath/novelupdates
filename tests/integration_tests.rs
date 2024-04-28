mod common;

use common::TestContext;
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_full_application_workflow() {
    let ctx = TestContext::new().await;

    // Step 1: Create a role
    let role_payload = json!({
        "name": "Content Manager",
        "description": "Manages content and RSS feeds"
    });

    let role_response = ctx
        .client
        .post("http://localhost:8080/api/roles")
        .header("Authorization", ctx.get_auth_header())
        .json(&role_payload)
        .send()
        .await
        .expect("Failed to create role");

    assert_eq!(role_response.status(), 201);
    let role: serde_json::Value = role_response.json().await.expect("Failed to parse role");
    let role_id = role["id"].as_str().unwrap();

    // Step 2: Create a user
    let user_payload = json!({
        "username": "content_manager",
        "email": "manager@example.com",
        "first_name": "Content",
        "last_name": "Manager",
        "password": "SecurePass123!",
        "api_key": "content-manager-key",
        "pid": "750e8400-e29b-41d4-a716-446655440000"
    });

    let user_response = ctx
        .client
        .post("http://localhost:8080/api/users")
        .header("Authorization", ctx.get_auth_header())
        .json(&user_payload)
        .send()
        .await
        .expect("Failed to create user");

    assert_eq!(user_response.status(), 201);
    let user: serde_json::Value = user_response.json().await.expect("Failed to parse user");
    let user_id = user["id"].as_str().unwrap();

    // Step 3: Create tags
    let mut tag_ids = Vec::new();
    for tag_name in &["Technology", "News", "Memes"] {
        let tag_payload = json!({ "name": tag_name });
        let tag_response = ctx
            .client
            .post("http://localhost:8080/api/tags")
            .header("Authorization", ctx.get_auth_header())
            .json(&tag_payload)
            .send()
            .await
            .expect("Failed to create tag");

        let tag: serde_json::Value = tag_response.json().await.expect("Failed to parse tag");
        tag_ids.push(tag["id"].as_str().unwrap().to_string());
    }

    // Step 4: Create RSS feeds
    let rss_feeds = vec![
        json!({
            "name": "Tech RSS",
            "url": "https://techcrunch.com/feed",
            "is_active": true,
            "fetch_interval": 3600
        }),
        json!({
            "name": "News RSS",
            "url": "https://news.ycombinator.com/rss",
            "is_active": true,
            "fetch_interval": 1800
        }),
    ];

    let mut rss_ids = Vec::new();
    for rss_feed in &rss_feeds {
        let rss_response = ctx
            .client
            .post("http://localhost:8080/api/rsses")
            .header("Authorization", ctx.get_auth_header())
            .json(rss_feed)
            .send()
            .await
            .expect("Failed to create RSS feed");

        let rss: serde_json::Value = rss_response.json().await.expect("Failed to parse RSS");
        rss_ids.push(rss["id"].as_str().unwrap().to_string());
    }

    // Step 5: Verify all resources were created
    let roles_list = ctx.client.get("http://localhost:8080/api/roles")
        .header("Authorization", ctx.get_auth_header()).send().await.unwrap();
    assert_eq!(roles_list.status(), 200);

    let users_list = ctx.client.get("http://localhost:8080/api/users")
        .header("Authorization", ctx.get_auth_header()).send().await.unwrap();
    assert_eq!(users_list.status(), 200);

    let tags_list = ctx.client.get("http://localhost:8080/api/tags")
        .header("Authorization", ctx.get_auth_header()).send().await.unwrap();
    assert_eq!(tags_list.status(), 200);

    let rss_list = ctx.client.get("http://localhost:8080/api/rsses")
        .header("Authorization", ctx.get_auth_header()).send().await.unwrap();
    assert_eq!(rss_list.status(), 200);

    // Step 6: Clean up - delete all created resources
    for rss_id in rss_ids {
        let _ = ctx.client.delete(&format!("http://localhost:8080/api/rsses/{}", rss_id))
            .header("Authorization", ctx.get_auth_header()).send().await;
    }

    for tag_id in tag_ids {
        let _ = ctx.client.delete(&format!("http://localhost:8080/api/tags/{}", tag_id))
            .header("Authorization", ctx.get_auth_header()).send().await;
    }

    let _ = ctx.client.delete(&format!("http://localhost:8080/api/users/{}", user_id))
        .header("Authorization", ctx.get_auth_header()).send().await;

    let _ = ctx.client.delete(&format!("http://localhost:8080/api/roles/{}", role_id))
        .header("Authorization", ctx.get_auth_header()).send().await;
}

#[tokio::test]
#[serial]
async fn test_concurrent_requests() {
    let ctx = TestContext::new().await;

    // Create multiple resources concurrently
    let mut handles = vec![];

    for i in 0..5 {
        let client = ctx.client.clone();
        let auth = ctx.get_auth_header();

        let handle = tokio::spawn(async move {
            let payload = json!({
                "name": format!("Concurrent Tag {}", i)
            });

            client
                .post("http://localhost:8080/api/tags")
                .header("Authorization", auth)
                .json(&payload)
                .send()
                .await
        });

        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(response)) = handle.await {
            if response.status() == 201 {
                success_count += 1;
            }
        }
    }

    assert_eq!(success_count, 5, "All concurrent requests should succeed");
}

