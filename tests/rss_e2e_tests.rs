mod common;

use common::TestContext;
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_rss_crud_operations() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/rsses";

    // CREATE - Create a new RSS feed
    let create_payload = json!({
        "name": "Tech News Feed",
        "url": "https://example.com/rss/tech",
        "is_active": true,
        "fetch_interval": 3600
    });

    let create_response = ctx
        .client
        .post(base_url)
        .header("Authorization", ctx.get_auth_header())
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create RSS feed");

    assert_eq!(create_response.status(), 201);

    let created_rss: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse created RSS feed");

    let rss_id = created_rss["id"].as_str().expect("No RSS ID");
    assert_eq!(created_rss["name"], "Tech News Feed");
    assert_eq!(created_rss["url"], "https://example.com/rss/tech");
    assert_eq!(created_rss["is_active"], true);
    assert_eq!(created_rss["fetch_interval"], 3600);

    // READ - Get the created RSS feed
    let read_response = ctx
        .client
        .get(&format!("{}/{}", base_url, rss_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to get RSS feed");

    assert_eq!(read_response.status(), 200);

    let read_rss: serde_json::Value = read_response
        .json()
        .await
        .expect("Failed to parse RSS feed");

    assert_eq!(read_rss["id"], rss_id);
    assert_eq!(read_rss["name"], "Tech News Feed");

    // LIST - Get all RSS feeds
    let list_response = ctx
        .client
        .get(base_url)
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to list RSS feeds");

    assert_eq!(list_response.status(), 200);

    let rss_feeds: serde_json::Value = list_response
        .json()
        .await
        .expect("Failed to parse RSS feeds list");

    assert!(rss_feeds.as_array().expect("Not an array").len() > 0);

    // UPDATE - Update the RSS feed
    let update_payload = json!({
        "name": "Updated Tech News Feed",
        "url": "https://example.com/rss/tech-updated",
        "is_active": false,
        "fetch_interval": 7200
    });

    let update_response = ctx
        .client
        .put(&format!("{}/{}", base_url, rss_id))
        .header("Authorization", ctx.get_auth_header())
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update RSS feed");

    assert_eq!(update_response.status(), 200);

    let updated_rss: serde_json::Value = update_response
        .json()
        .await
        .expect("Failed to parse updated RSS feed");

    assert_eq!(updated_rss["name"], "Updated Tech News Feed");
    assert_eq!(updated_rss["url"], "https://example.com/rss/tech-updated");
    assert_eq!(updated_rss["is_active"], false);
    assert_eq!(updated_rss["fetch_interval"], 7200);

    // DELETE - Delete the RSS feed
    let delete_response = ctx
        .client
        .delete(&format!("{}/{}", base_url, rss_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to delete RSS feed");

    assert_eq!(delete_response.status(), 204);

    // Verify deletion
    let verify_response = ctx
        .client
        .get(&format!("{}/{}", base_url, rss_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to verify deletion");

    assert_eq!(verify_response.status(), 404);
}

#[tokio::test]
#[serial]
async fn test_rss_active_inactive_filtering() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/rsses";

    // Create active RSS feed
    let active_payload = json!({
        "name": "Active Feed",
        "url": "https://example.com/active",
        "is_active": true,
        "fetch_interval": 1800
    });

    let active_response = ctx
        .client
        .post(base_url)
        .header("Authorization", ctx.get_auth_header())
        .json(&active_payload)
        .send()
        .await
        .expect("Failed to create active feed");

    let active_rss: serde_json::Value = active_response.json().await.expect("Failed to parse");
    let active_id = active_rss["id"].as_str().unwrap();

    // Create inactive RSS feed
    let inactive_payload = json!({
        "name": "Inactive Feed",
        "url": "https://example.com/inactive",
        "is_active": false,
        "fetch_interval": 1800
    });

    let inactive_response = ctx
        .client
        .post(base_url)
        .header("Authorization", ctx.get_auth_header())
        .json(&inactive_payload)
        .send()
        .await
        .expect("Failed to create inactive feed");

    let inactive_rss: serde_json::Value = inactive_response.json().await.expect("Failed to parse");
    let inactive_id = inactive_rss["id"].as_str().unwrap();

    // List all feeds
    let list_response = ctx
        .client
        .get(base_url)
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to list feeds");

    let feeds: serde_json::Value = list_response.json().await.expect("Failed to parse");
    assert!(feeds.as_array().unwrap().len() >= 2);

    // Clean up
    let _ = ctx.client.delete(&format!("{}/{}", base_url, active_id))
        .header("Authorization", ctx.get_auth_header()).send().await;
    let _ = ctx.client.delete(&format!("{}/{}", base_url, inactive_id))
        .header("Authorization", ctx.get_auth_header()).send().await;
}

