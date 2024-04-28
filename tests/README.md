# E2E Tests for Meme Generator

This directory contains end-to-end tests for the CRUD operations in the meme-generator application using testcontainers.

## Overview

The E2E tests use:
- **Testcontainers**: Spin up real Postgres and Keycloak containers for testing
- **Postgres**: Database backend for the application
- **Keycloak**: OIDC authentication provider
- **Reqwest**: HTTP client for making API requests

## Test Files

- `common/mod.rs`: Shared test utilities and TestContext setup
- `role_e2e_tests.rs`: CRUD tests for Role resource
- `user_e2e_tests.rs`: CRUD tests for User resource
- `tag_e2e_tests.rs`: CRUD tests for Tag resource
- `rss_e2e_tests.rs`: CRUD tests for RSS feed resource
- `integration_tests.rs`: Full workflow and integration tests

## Prerequisites

1. **Docker**: Ensure Docker is running on your machine (required for testcontainers)
2. **Rust**: Rust toolchain installed
3. **Database Migrations**: The migrations in `./migrations` directory will be run automatically

## Running the Tests

### Run all tests:
```bash
cargo test --test '*'
```

### Run specific test file:
```bash
cargo test --test role_e2e_tests
cargo test --test user_e2e_tests
cargo test --test tag_e2e_tests
cargo test --test rss_e2e_tests
cargo test --test integration_tests
```

### Run a specific test:
```bash
cargo test --test role_e2e_tests test_role_crud_operations
```

### Run tests with output:
```bash
cargo test --test '*' -- --nocapture
```

## Test Structure

Each test follows this pattern:

1. **Setup**: TestContext creates and configures Postgres and Keycloak containers
2. **Authentication**: Obtains OAuth2 token from Keycloak
3. **Test Operations**: Executes CRUD operations (Create, Read, Update, Delete)
4. **Assertions**: Verifies expected behavior
5. **Cleanup**: Automatically cleaned up when TestContext is dropped

## TestContext

The `TestContext` struct handles:
- Starting Postgres container
- Starting Keycloak container
- Running database migrations
- Setting up Keycloak realm and client
- Creating a test user
- Obtaining OAuth2 access tokens
- Providing authenticated HTTP client

## Authentication Flow

Tests use the following Keycloak setup:
- **Realm**: `test-realm`
- **Client ID**: `test-client`
- **Client Secret**: `test-secret`
- **Test User**: `testuser` / `testpass`

## Notes

- Tests are marked with `#[serial]` to run sequentially (prevents port conflicts)
- Containers are automatically cleaned up after tests complete
- Each test is independent and creates its own resources
- The application server must be running on `localhost:8080` for tests to work

## Troubleshooting

### Docker not running
```
Error: Failed to start Postgres container
```
**Solution**: Start Docker Desktop or Docker daemon

### Port conflicts
```
Error: Address already in use
```
**Solution**: Ensure no other services are running on ports 5432 or 8080

### Slow tests
The first run may be slow as Docker pulls the Postgres and Keycloak images. Subsequent runs will be faster.

### Container cleanup
If containers are not being cleaned up properly:
```bash
docker ps -a | grep testcontainers | awk '{print $1}' | xargs docker rm -f
```

## Example Test Output

```
running 5 tests
test role_e2e_tests::test_role_crud_operations ... ok
test role_e2e_tests::test_role_validation ... ok
test user_e2e_tests::test_user_crud_operations ... ok
test tag_e2e_tests::test_tag_crud_operations ... ok
test integration_tests::test_full_application_workflow ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

