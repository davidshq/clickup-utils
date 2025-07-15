# ClickUp CLI Integration Tests

This document explains how to set up and run the integration tests for the ClickUp CLI.

## Overview

The integration tests verify that the CLI works correctly with the real ClickUp API. They test authentication, workspace management, task lifecycle operations, commenting, error handling, and rate limiting.

## Prerequisites

1. **ClickUp Account**: You need a ClickUp account with API access
2. **API Token**: A ClickUp API token for testing
3. **Test Workspace**: A workspace where you can create test data
4. **Test List**: A list where you can create test tasks

## Setup

### 1. Get Your API Token

1. Go to ClickUp Settings > Apps
2. Create a new API token
3. Copy the token (you'll need it for the next step)

### 2. Configure Test Environment

1. Copy the example environment file:
   ```bash
   cp env.test.example .env.test
   ```

2. Edit `.env.test` and add your credentials:
   ```env
   CLICKUP_API_TOKEN_TEST=your_actual_token_here
   CLICKUP_TEST_WORKSPACE_ID=your_workspace_id_here
   CLICKUP_TEST_LIST_ID=your_list_id_here
   ```

**Note**: The test environment automatically loads from `.env.test` and uses separate tokens from your live environment. This ensures tests never interfere with your production data.

### 3. Find Your Workspace and List IDs

#### Workspace ID
1. Go to your ClickUp workspace
2. Look at the URL: `https://app.clickup.com/123456/v/li/...`
3. The workspace ID is `123456` in this example

#### List ID
1. Open a list in ClickUp
2. Look at the URL: `https://app.clickup.com/123456/v/li/789012`
3. The list ID is `789012` in this example

## Running Tests

### Using the Test Runner Script (Recommended)

```bash
# Run all integration tests
./scripts/run_integration_tests.sh

# Run a specific test
./scripts/run_integration_tests.sh test_authentication
```

### Using Cargo Directly

```bash
# Run all integration tests (including ignored ones)
cargo test -- --ignored

# Run a specific test
cargo test test_authentication -- --ignored

# Run tests with verbose output
cargo test -- --ignored --nocapture
```

## Test Categories

### 1. Authentication Tests
- **`test_authentication`**: Verifies the CLI can authenticate with the API
- **`test_invalid_authentication`**: Tests error handling with invalid tokens

### 2. Workspace and Team Tests
- **`test_workspace_team_listing`**: Tests listing workspaces and teams

### 3. Task Lifecycle Tests
- **`test_task_lifecycle`**: Full CRUD operations on tasks (create, read, update, delete)

### 4. Comment Tests
- **`test_commenting`**: Tests comment creation, listing, updating, and deletion

### 5. Error Handling Tests
- **`test_error_handling`**: Tests graceful error handling for invalid operations

### 6. Rate Limiting Tests
- **`test_rate_limiting`**: Tests how the CLI handles API rate limits

### 7. List Operations Tests
- **`test_list_operations`**: Tests listing spaces and tasks

### 8. Basic CLI Tests
- **`test_cli_basic_commands`**: Tests help, version, and invalid command handling

## Test Environment

### Environment Variables

The tests use these environment variables:

- `CLICKUP_API_TOKEN_TEST`: Your ClickUp API token
- `CLICKUP_TEST_WORKSPACE_ID`: ID of a test workspace (optional)
- `CLICKUP_TEST_LIST_ID`: ID of a test list (optional)

### Test Environment Safety

The test environment has been improved with proper isolation. See [ADR 0004: Integration Testing Strategy](../adr/0004-integration-testing-strategy.md) for detailed implementation information.

## Troubleshooting

### Common Issues

1. **"CLICKUP_API_TOKEN_TEST not set"**
   - Make sure you've created `.env.test` with your token
   - Verify the token is valid in ClickUp

2. **"No test list ID available"**
   - Add `CLICKUP_TEST_LIST_ID` to your `.env.test`
   - Or the tests will skip list-dependent operations

3. **"Authentication failed"**
   - Check that your API token is correct
   - Ensure the token has the necessary permissions

4. **Rate limiting errors**
   - This is expected behavior
   - Tests will retry and continue

### Debugging

To see detailed output from tests:

```bash
# Run with verbose output
cargo test -- --ignored --nocapture

# Run a specific test with verbose output
cargo test test_task_lifecycle -- --ignored --nocapture
```

### Test Output

Successful tests will show:
- ✅ Task creation, update, and deletion
- ✅ Comment creation and management
- ✅ Error handling for invalid operations
- ✅ Rate limiting detection

## Adding New Tests

When adding new integration tests:

1. **Use `#[ignore]`**: Mark tests with `#[ignore]` to prevent accidental execution
2. **Check prerequisites**: Verify required environment variables are set
3. **Clean up**: Always clean up resources created during tests
4. **Use unique names**: Include timestamps in resource names to avoid conflicts
5. **Handle errors gracefully**: Skip tests if prerequisites aren't met

Example test structure:

```rust
#[test]
#[ignore]
fn test_new_feature() {
    load_env();
    
    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }
    
    let _temp_dir = setup_test_env();
    
    // Test implementation...
}
```

## Security Notes

- **Never commit `.env.test`**: It contains sensitive credentials
- **Use test tokens**: Create separate API tokens for testing
- **Clean up test data**: Tests should clean up after themselves
- **Automatic token separation**: Tests automatically use `.env.test` tokens, never production tokens
- **Safe test isolation**: Thread-local storage prevents test interference
- **Isolate test environments**: Use separate workspaces/lists for testing

## Continuous Integration

For CI/CD pipelines:

1. Set `CLICKUP_API_TOKEN_TEST` as a secret environment variable
2. Run tests with: `cargo test -- --ignored`
3. Consider running integration tests in a separate job from unit tests
4. Set appropriate timeouts for API operations 