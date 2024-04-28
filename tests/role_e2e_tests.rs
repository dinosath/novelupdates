mod common;

use common::TestContext;
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_role_crud_operations() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/roles";

    // CREATE - Create a new role
    let create_payload = json!({
        "name": "Admin Role",
        "description": "Administrator role with full permissions"
    });

    let create_response = ctx
        .client
        .post(base_url)
        .header("Authorization", ctx.get_auth_header())
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create role");

    assert_eq!(create_response.status(), 201);

    let created_role: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse created role");

    let role_id = created_role["id"].as_str().expect("No role ID");
    assert_eq!(created_role["name"], "Admin Role");
    assert_eq!(created_role["description"], "Administrator role with full permissions");

    // READ - Get the created role
    let read_response = ctx
        .client
        .get(&format!("{}/{}", base_url, role_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to get role");

    assert_eq!(read_response.status(), 200);

    let read_role: serde_json::Value = read_response
        .json()
        .await
        .expect("Failed to parse role");

    assert_eq!(read_role["id"], role_id);
    assert_eq!(read_role["name"], "Admin Role");

    // LIST - Get all roles
    let list_response = ctx
        .client
        .get(base_url)
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to list roles");

    assert_eq!(list_response.status(), 200);

    let roles: serde_json::Value = list_response
        .json()
        .await
        .expect("Failed to parse roles list");

    assert!(roles.as_array().expect("Not an array").len() > 0);

    // UPDATE - Update the role
    let update_payload = json!({
        "name": "Super Admin Role",
        "description": "Updated administrator role"
    });

    let update_response = ctx
        .client
        .put(&format!("{}/{}", base_url, role_id))
        .header("Authorization", ctx.get_auth_header())
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update role");

    assert_eq!(update_response.status(), 200);

    let updated_role: serde_json::Value = update_response
        .json()
        .await
        .expect("Failed to parse updated role");

    assert_eq!(updated_role["name"], "Super Admin Role");
    assert_eq!(updated_role["description"], "Updated administrator role");

    // DELETE - Delete the role
    let delete_response = ctx
        .client
        .delete(&format!("{}/{}", base_url, role_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to delete role");

    assert_eq!(delete_response.status(), 204);

    // Verify deletion
    let verify_response = ctx
        .client
        .get(&format!("{}/{}", base_url, role_id))
        .header("Authorization", ctx.get_auth_header())
        .send()
        .await
        .expect("Failed to verify deletion");

    assert_eq!(verify_response.status(), 404);
}

#[tokio::test]
#[serial]
async fn test_role_validation() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/roles";

    // Test creating role without required field
    let invalid_payload = json!({
        "description": "Role without name"
    });

    let response = ctx
        .client
        .post(base_url)
        .header("Authorization", ctx.get_auth_header())
        .json(&invalid_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_client_error());
}

#[tokio::test]
#[serial]
async fn test_role_unauthorized_access() {
    let ctx = TestContext::new().await;
    let base_url = "http://localhost:8080/api/roles";

    // Test without authentication
    let response = ctx
        .client
        .get(base_url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 401);
}

