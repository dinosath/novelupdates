use std::collections::HashMap;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use sqlx::postgres::PgPoolOptions;
use reqwest::Client;
use serde_json::json;
use tokio::process::{Child, Command};
use std::env;

pub struct TestContext {
    pub postgres_container: ContainerAsync<Postgres>,
    pub keycloak_container: ContainerAsync<GenericImage>,
    pub database_url: String,
    pub keycloak_url: String,
    pub client: Client,
    pub access_token: Option<String>,
    pub app_process: Option<Child>,
}

impl TestContext {
    pub async fn new() -> Self {
        // Start Postgres container
        let postgres_container = Postgres::default()
            .start()
            .await
            .expect("Failed to start Postgres container");

        let postgres_port = postgres_container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get Postgres port");

        let database_url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres_port
        );

        // Start Keycloak container
        let keycloak_container = GenericImage::new("quay.io/keycloak/keycloak", "23.0")
            .with_wait_for(testcontainers::core::WaitFor::message_on_stdout(
                "Running the server in development mode",
            ))
            .with_exposed_port(8080.into())
            .with_env_var("KEYCLOAK_ADMIN", "admin")
            .with_env_var("KEYCLOAK_ADMIN_PASSWORD", "admin")
            .with_env_var("KC_HTTP_ENABLED", "true")
            .with_env_var("KC_HOSTNAME_STRICT", "false")
            .with_cmd(vec!["start-dev"])
            .start()
            .await
            .expect("Failed to start Keycloak container");

        let keycloak_port = keycloak_container
            .get_host_port_ipv4(8080)
            .await
            .expect("Failed to get Keycloak port");

        let keycloak_url = format!("http://127.0.0.1:{}", keycloak_port);

        // Run migrations
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        let client = Client::builder()
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP client");

        let mut context = Self {
            postgres_container,
            keycloak_container,
            database_url: database_url.clone(),
            keycloak_url: keycloak_url.clone(),
            client,
            access_token: None,
            app_process: None,
        };

        // Setup Keycloak realm and client
        context.setup_keycloak().await;

        // Start the application server
        context.start_app_server().await;

        context
    }

    async fn start_app_server(&mut self) {
        // Set environment variables for the app
        let issuer = format!("{}/realms/test-realm", self.keycloak_url);
        let app_url = "http://localhost:8080".to_string();

        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .env("DATABASE_URL", &self.database_url)
            .env("APP_URL", &app_url)
            .env("ISSUER", &issuer)
            .env("CLIENT_ID", "test-client")
            .env("CLIENT_SECRET", "test-secret")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true);

        let child = cmd.spawn().expect("Failed to start application");
        self.app_process = Some(child);

        // Wait for the app to be ready with retries
        let mut retries = 60; // 60 attempts = 60 seconds max
        let mut is_ready = false;

        while retries > 0 && !is_ready {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            // Try to connect to the health endpoint or root
            if let Ok(response) = self.client
                .get("http://localhost:8080/docs")
                .send()
                .await
            {
                if response.status().is_success() || response.status().as_u16() == 404 {
                    is_ready = true;
                    println!("✅ Application server is ready!");
                }
            }

            retries -= 1;
        }

        if !is_ready {
            panic!("Application server failed to start within 60 seconds");
        }
    }

    async fn setup_keycloak(&mut self) {
        // Wait a bit for Keycloak to fully start
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Get admin token
        let admin_token = self.get_keycloak_admin_token().await;

        // Create realm
        let realm_payload = json!({
            "realm": "test-realm",
            "enabled": true,
            "registrationAllowed": true,
            "resetPasswordAllowed": true
        });

        let _ = self
            .client
            .post(format!("{}/admin/realms", self.keycloak_url))
            .bearer_auth(&admin_token)
            .json(&realm_payload)
            .send()
            .await;

        // Create client
        let client_payload = json!({
            "clientId": "test-client",
            "enabled": true,
            "publicClient": false,
            "secret": "test-secret",
            "redirectUris": ["http://localhost:8080/*"],
            "webOrigins": ["*"],
            "directAccessGrantsEnabled": true,
            "serviceAccountsEnabled": true
        });

        let _ = self
            .client
            .post(format!("{}/admin/realms/test-realm/clients", self.keycloak_url))
            .bearer_auth(&admin_token)
            .json(&client_payload)
            .send()
            .await;

        // Create test user
        let user_payload = json!({
            "username": "testuser",
            "enabled": true,
            "email": "test@example.com",
            "firstName": "Test",
            "lastName": "User",
            "credentials": [{
                "type": "password",
                "value": "testpass",
                "temporary": false
            }]
        });

        let _ = self
            .client
            .post(format!("{}/admin/realms/test-realm/users", self.keycloak_url))
            .bearer_auth(&admin_token)
            .json(&user_payload)
            .send()
            .await;

        // Get user access token
        self.access_token = Some(self.get_user_access_token().await);
    }

    async fn get_keycloak_admin_token(&self) -> String {
        let mut params = HashMap::new();
        params.insert("grant_type", "password");
        params.insert("client_id", "admin-cli");
        params.insert("username", "admin");
        params.insert("password", "admin");

        let response = self
            .client
            .post(format!(
                "{}/realms/master/protocol/openid-connect/token",
                self.keycloak_url
            ))
            .form(&params)
            .send()
            .await
            .expect("Failed to get admin token");

        let json: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse admin token response");

        json["access_token"]
            .as_str()
            .expect("No access token in response")
            .to_string()
    }

    async fn get_user_access_token(&self) -> String {
        let mut params = HashMap::new();
        params.insert("grant_type", "password");
        params.insert("client_id", "test-client");
        params.insert("client_secret", "test-secret");
        params.insert("username", "testuser");
        params.insert("password", "testpass");

        let response = self
            .client
            .post(format!(
                "{}/realms/test-realm/protocol/openid-connect/token",
                self.keycloak_url
            ))
            .form(&params)
            .send()
            .await
            .expect("Failed to get user token");

        let json: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse user token response");

        json["access_token"]
            .as_str()
            .expect("No access token in response")
            .to_string()
    }

    pub fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.access_token.as_ref().unwrap())
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Kill the app process if it's still running
        if let Some(mut process) = self.app_process.take() {
            let _ = process.start_kill();
        }
    }
}
