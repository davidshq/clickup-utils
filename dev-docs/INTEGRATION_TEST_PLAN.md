# Integration Test Plan for ClickUp CLI

This document outlines the main needs and approach for implementing integration tests for the ClickUp CLI, focusing on real API usage with the `CLICKUP_API_TOKEN_TEST` environment variable.

---

## Important Note on Test Status

A test is only considered **implemented** when it runs and passes in the current environment. If a test is skipped or fails due to missing data or configuration, it is **not** considered fully implemented.

---

## 1. Environment Setup ✅ **PASSING**

- **Use `CLICKUP_API_TOKEN_TEST`:**
  - All integration tests should read the `CLICKUP_API_TOKEN_TEST` environment variable for authentication.
- **Use `.env.test` for Test Credentials:**
  - Store test credentials and test workspace/list IDs in a `.env.test` file (not checked into version control).
  - Example `.env.test`:
    ```env
    CLICKUP_API_TOKEN_TEST=your_test_token
    CLICKUP_TEST_LIST_ID=your_test_list_id
    CLICKUP_TEST_WORKSPACE_ID=your_test_workspace_id
    ```
- **Isolate Test Data:**
  - Use unique names or tags for resources created during tests to avoid interfering with real data.
- **Clean Up:**
  - Ensure tests clean up any resources (tasks, lists, etc.) they create.

---

## 2. Core Integration Test Scenarios

### a. Authentication ✅ **PASSING**
- **Test:** Verify that the CLI can authenticate using the test token and access the API (e.g., fetch current user or workspace).
- **Goal:** Ensure the token is valid and the CLI can make authenticated requests.
- **Status:** Implemented and passing.

### b. Workspace/Team Listing ✅ **PASSING**
- **Test:** Run the CLI to list workspaces/teams and check for a successful response.
- **Goal:** Confirm the CLI can fetch and display workspace/team data.
- **Status:** Implemented and passing.

### c. Task Lifecycle ✅ **PASSING**
- **Test:**
  1. Create a new task in a test list.
  2. Fetch the task and verify its properties.
  3. Update the task (e.g., change name or status).
  4. Delete the task.
- **Goal:** Ensure the CLI can perform full CRUD operations on tasks.
- **Status:** Implemented and passing.

### d. Commenting ✅ **PASSING**
- **Test:**
  1. Add a comment to a test task.
  2. Fetch comments for the task and verify the new comment appears.
- **Goal:** Validate comment creation and retrieval.
- **Status:** Implemented and passing.

### e. Error Handling ✅ **PASSING**
- **Test:** Attempt an operation with an invalid token or on a non-existent resource.
- **Goal:** Ensure the CLI reports errors gracefully and does not panic.
- **Status:** Implemented and passing.

### f. Rate Limiting ✅ **PASSING**
- **Test:** Simulate or trigger API rate limiting (HTTP 429) and verify the CLI handles it gracefully (e.g., retries, user-friendly error message).
- **Goal:** Ensure robust handling of rate limits.
- **Status:** Implemented and passing.

### g. List Operations ✅ **PASSING**
- **Test:** Create, list, show, and delete lists within a space.
- **Goal:** Validate list management functionality.
- **Status:** Implemented and passing.

### h. Invalid Authentication ✅ **PASSING**
- **Test:** Test CLI behavior with invalid or missing authentication tokens.
- **Goal:** Ensure proper error handling for authentication failures.
- **Status:** Implemented and passing.

### i. CLI Basic Commands ✅ **PASSING**
- **Test:** Verify basic CLI functionality like help, version, and invalid command handling.
- **Goal:** Ensure CLI interface works correctly.
- **Status:** Implemented and passing.

---

## 3. Test Structure & Best Practices ✅ **PASSING**

- **Use `#[ignore]` Attribute:**
  - Mark integration tests with `#[ignore]` by default, so they only run when explicitly requested (e.g., `cargo test -- --ignored`).
- **Parallel Safety:**
  - Avoid tests that could interfere with each other if run in parallel (e.g., by using unique resource names).
  - For tests that must not run in parallel, use the [`serial_test`](https://docs.rs/serial_test/latest/serial_test/) crate.
- **Minimal External Dependencies:**
  - Only depend on the ClickUp API and the test token; avoid requiring other setup.
- **Use Ergonomic Testing Crates:**
  - [`assert_cmd`](https://docs.rs/assert_cmd/latest/assert_cmd/) for running and asserting on CLI commands.
  - [`predicates`](https://docs.rs/predicates/latest/predicates/) for output assertions.
  - [`dotenvy`](https://docs.rs/dotenvy/latest/dotenvy/) for loading environment variables from `.env.test`.

---

## 4. Loading Environment Variables in Tests ✅ **PASSING**

The current implementation uses this pattern:

```rust
fn load_env() {
    let _ = dotenvy::from_filename(".env.test");
}

fn has_test_token() -> bool {
    env::var("CLICKUP_API_TOKEN_TEST").is_ok()
}
```

Tests call `load_env()` at the start and check `has_test_token()` before proceeding.

---

## 5. Current Test Implementation Status

### ✅ **All Tests Passing:**
- `test_authentication()` - Verifies API authentication
- `test_workspace_team_listing()` - Tests workspace and team listing
- `test_task_lifecycle()` - Tests full CRUD operations on tasks
- `test_commenting()` - Tests comment creation and retrieval
- `test_error_handling()` - Tests error handling scenarios
- `test_rate_limiting()` - Tests rate limiting behavior
- `test_list_operations()` - Tests list management functionality
- `test_invalid_authentication()` - Tests authentication error handling
- `test_cli_basic_commands()` - Tests basic CLI functionality

### ✅ **Basic CLI Tests Passing:**
- `test_cli_help()` - Tests CLI help command
- `test_cli_version()` - Tests CLI version command
- `test_cli_invalid_command()` - Tests CLI error handling

### 🔧 **Helper Functions Implemented:**
- `setup_test_env()` - Temporary environment setup
- `run_cli_with_test_env()` - CLI execution with test environment
- `get_or_discover_workspace_id()` - Dynamic workspace discovery
- `get_or_discover_space_id()` - Dynamic space discovery
- `get_or_discover_test_list_id()` - Dynamic list discovery
- `extract_task_id()` - Parse task IDs from CLI output
- `extract_comment_id()` - Parse comment IDs from CLI output
- `delete_list()` - Cleanup helper
- `get_list_statuses()` - Get available statuses for a list

---

## 6. Running Integration Tests

### Using the Test Script ✅ **PASSING**
```bash
# Run all integration tests
./scripts/run_integration_tests.sh

# Run specific test
./scripts/run_integration_tests.sh test_authentication
```

### Manual Execution
```bash
# Run all integration tests (including ignored ones)
cargo test -- --ignored

# Run specific test
cargo test test_authentication -- --ignored

# Run with output
cargo test test_authentication -- --ignored --nocapture
```

---

## 7. Test Environment Setup

### Required Files:
- `.env.test` - Test credentials (copy from `env.test.example`)
- `CLICKUP_API_TOKEN_TEST` environment variable

### Optional Configuration:
- `CLICKUP_TEST_WORKSPACE_ID` - Specific workspace for testing
- `CLICKUP_TEST_LIST_ID` - Specific list for testing
- `CLICKUP_TEST_SPACE_ID` - Specific space for testing

If optional IDs are not provided, tests will dynamically discover available workspaces, spaces, and lists. If discovery fails, tests may be skipped.

---

## 8. Current Status & Improvements Needed

### ✅ **Major Accomplishments:**
1. **All Integration Tests Passing:** All 9 main integration tests are now implemented and passing.
2. **Dynamic Resource Discovery:** Tests can automatically discover workspaces, spaces, and lists when not explicitly configured.
3. **Comprehensive Coverage:** Tests cover authentication, CRUD operations, error handling, rate limiting, and CLI functionality.
4. **Robust Test Environment:** Proper environment setup and cleanup working correctly.

### 🔧 **Minor Issues to Address:**
1. **Dead Code Warnings:** Some helper functions are marked as unused but may be needed for future tests:
   - `setup_test_authentication()` - Unused function
   - `get_test_workspace_id()` - Unused function  
   - `extract_comment_id()` - Unused function
2. **Test Output Parsing:** The `extract_task_id()` and `extract_comment_id()` functions could be more robust.

### 📋 **Potential Enhancements:**
1. **Test Data Isolation:** Add unique prefixes/tags to all test resources for better isolation.
2. **Parallel Test Safety:** Implement `serial_test` for tests that modify shared resources.
3. **Test Reporting:** Add more detailed test output and reporting.
4. **Mock API Support:** Add optional mock API support for faster unit testing.
5. **Performance Benchmarks:** Add performance testing for API operations.
6. **Additional Test Scenarios:** Add tests for file attachments, time tracking, and other advanced features.

---

## 9. Next Steps

### ✅ **Completed:**
- All integration tests implemented and passing
- Environment setup and helper functions working
- Test script and documentation complete
- Graceful handling of missing credentials
- Dynamic resource discovery working
- Comprehensive error handling and rate limiting tests

### 🔄 **In Progress:**
- Cleaning up unused helper functions
- Fine-tuning test reliability and output parsing
- Improving environment variable loading
- Adding more robust error handling

### 📋 **Future Enhancements:**
1. Add more comprehensive test scenarios (e.g., file attachments, time tracking)
2. Implement test data cleanup automation
3. Add performance benchmarks
4. Create test result reporting and analytics
5. Add integration tests for new CLI features as they are developed
6. Implement test coverage reporting
7. Add automated test result notifications 