# Integration Test Plan for ClickUp CLI

This document outlines the main needs and approach for implementing integration tests for the ClickUp CLI, focusing on real API usage with the `CLICKUP_API_TOKEN_TEST` environment variable.

---

## 1. Environment Setup
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

### a. Authentication
- **Test:** Verify that the CLI can authenticate using the test token and access the API (e.g., fetch current user or workspace).
- **Goal:** Ensure the token is valid and the CLI can make authenticated requests.

### b. Workspace/Team Listing
- **Test:** Run the CLI to list workspaces/teams and check for a successful response.
- **Goal:** Confirm the CLI can fetch and display workspace/team data.

### c. Task Lifecycle
- **Test:**
  1. Create a new task in a test list.
  2. Fetch the task and verify its properties.
  3. Update the task (e.g., change name or status).
  4. Delete the task.
- **Goal:** Ensure the CLI can perform full CRUD operations on tasks.

### d. Commenting
- **Test:**
  1. Add a comment to a test task.
  2. Fetch comments for the task and verify the new comment appears.
- **Goal:** Validate comment creation and retrieval.

### e. Error Handling
- **Test:** Attempt an operation with an invalid token or on a non-existent resource.
- **Goal:** Ensure the CLI reports errors gracefully and does not panic.

### f. Rate Limiting
- **Test:** Simulate or trigger API rate limiting (HTTP 429) and verify the CLI handles it gracefully (e.g., retries, user-friendly error message).
- **Goal:** Ensure robust handling of rate limits.

---

## 3. Test Structure & Best Practices
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

## 4. Loading Environment Variables in Tests

Add this to the top of your test file to load `.env.test` automatically:

```rust
fn load_env() {
    let _ = dotenvy::from_filename(".env.test");
}
```

Call `load_env()` at the start of each test or in a test setup function.

---

## 5. Example Test Skeleton

```rust
#[test]
#[ignore]
fn test_create_and_delete_task() {
    load_env();
    let token = match std::env::var("CLICKUP_API_TOKEN_TEST") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
            return;
        }
    };
    // Use assert_cmd to run the CLI and predicates to check output
    // ...
}
```

---

## Next Steps

1. Implement a helper to load `.env.test` and read `CLICKUP_API_TOKEN_TEST`, skipping tests if not set.
2. Write the authentication and workspace listing tests.
3. Expand to task lifecycle, comment, and rate limiting tests.
4. Add error handling tests. 