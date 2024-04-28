mod common;

use common::TestContext;
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_tag_crud_operations() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/tags";

    // CREATE - Create a new tag
    let create_payload = json!({
        "name": "Technology"
    });

    let create_response = ctx
        .client
        .post(base_url)
        .header("Authorization", ctx.get_auth_header())
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create tag");

    assert_eq!(create_response.status(), 201);

    let created_tag: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse created tag");

    let tag_id = created_tag["id"].as_str().expect("No tag ID");
    assert_eq!(created_tag["name"], "Technology");

    // READ - Get the created tag
    let read_response = ctx
        .client
        .get(&format!("{}/{}", base_url, tag_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to get tag");

    assert_eq!(read_response.status(), 200);

    let read_tag: serde_json::Value = read_response
        .json()
        .await
        .expect("Failed to parse tag");

    assert_eq!(read_tag["id"], tag_id);
    assert_eq!(read_tag["name"], "Technology");

    // LIST - Get all tags
    let list_response = ctx
        .client
        .get(base_url)
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to list tags");

    assert_eq!(list_response.status(), 200);

    let tags: serde_json::Value = list_response
        .json()
        .await
        .expect("Failed to parse tags list");

    assert!(tags.as_array().expect("Not an array").len() > 0);

    // UPDATE - Update the tag
    let update_payload = json!({
        "name": "Tech & Innovation"
    });

    let update_response = ctx
        .client
        .put(&format!("{}/{}", base_url, tag_id))
        .header("Authorization", ctx.get_auth_header())
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update tag");

    assert_eq!(update_response.status(), 200);

    let updated_tag: serde_json::Value = update_response
        .json()
        .await
        .expect("Failed to parse updated tag");

    assert_eq!(updated_tag["name"], "Tech & Innovation");

    // DELETE - Delete the tag
    let delete_response = ctx
        .client
        .delete(&format!("{}/{}", base_url, tag_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to delete tag");

    assert_eq!(delete_response.status(), 204);

    // Verify deletion
    let verify_response = ctx
        .client
        .get(&format!("{}/{}", base_url, tag_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to verify deletion");

    assert_eq!(verify_response.status(), 404);
}

#[tokio::test]
#[serial]
async fn test_tag_bulk_operations() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/tags";

    // Create multiple tags
    let tag_names = vec!["Sports", "Music", "Art", "Science"];
    let mut created_ids = Vec::new();

    for name in &tag_names {
        let payload = json!({ "name": name });
        let response = ctx
            .client
            .post(base_url)
            .header("Authorization", ctx.get_auth_header())
            .json(&payload)
            .send()
            .await
            .expect("Failed to create tag");

        let tag: serde_json::Value = response.json().await.expect("Failed to parse tag");
        created_ids.push(tag["id"].as_str().unwrap().to_string());
    }

    // Verify all tags exist
    let list_response = ctx
        .client
        .get(base_url)
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to list tags");

    let tags: serde_json::Value = list_response.json().await.expect("Failed to parse tags");
    assert!(tags.as_array().unwrap().len() >= tag_names.len());

    // Clean up - delete all created tags
    for id in created_ids {
        let _ = ctx
            .client
            .delete(&format!("{}/{}", base_url, id))
            .header("Authorization", ctx.get_auth_header())
            .send()
            .await;
    }
}
