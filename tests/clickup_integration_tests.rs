//! # ClickUp Integration Tests
//!
//! This module contains comprehensive integration tests for the ClickUp CLI application.
//! These tests use the real ClickUp API with the CLICKUP_API_TOKEN_TEST environment variable.
//!
//! ## Test Environment
//!
//! Tests require:
//! - CLICKUP_API_TOKEN_TEST environment variable set
//! - .env.test file with test credentials (optional)
//! - Test workspace and list IDs configured
//!
//! ## Running Tests
//!
//! ```bash
//! # Run all integration tests (including ignored ones)
//! cargo test -- --ignored
//!
//! # Run a specific test
//! cargo test test_authentication -- --ignored
//! ```

use assert_cmd::prelude::*;
use chrono;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;

/// Sets up a temporary environment for testing
fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    env::set_var("XDG_CONFIG_HOME", &temp_path);
    env::set_var("APPDATA", &temp_path);
    temp_dir
}

/// Helper to run CLI command with test environment
fn run_cli_with_test_env(args: &[&str]) -> std::process::Output {
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(args);

    // Set the test token environment variable for the CLI process
    if let Ok(test_token) = env::var("CLICKUP_API_TOKEN_TEST") {
        eprintln!(
            "[DEBUG] Using test token: {}",
            &test_token[..8.min(test_token.len())]
        );
        eprintln!("[DEBUG] Full test token length: {}", test_token.len());
        cmd.env("CLICKUP_API_TOKEN_TEST", test_token);
        // Clear the regular token to ensure test token is used
        cmd.env_remove("CLICKUP_API_TOKEN");
        // Prevent loading .env file by setting a dummy value
        cmd.env("CLICKUP_SKIP_ENV_FILE", "1");
    } else {
        eprintln!("[DEBUG] No test token found!");
    }

    cmd.output().unwrap()
}

/// Loads environment variables from .env.test file
fn load_env() {
    let _ = dotenvy::from_filename(".env.test");
}

/// Checks if the required test token is available
fn has_test_token() -> bool {
    env::var("CLICKUP_API_TOKEN_TEST").is_ok()
}

/// Sets up the test environment with proper authentication
#[allow(dead_code)]
fn setup_test_authentication() -> Result<(), Box<dyn std::error::Error>> {
    load_env();

    if !has_test_token() {
        return Err("CLICKUP_API_TOKEN_TEST not set".into());
    }

    Ok(())
}

/// Helper function to get a test list ID for task operations
fn get_test_list_id() -> Option<String> {
    // Try to get from environment first
    if let Ok(list_id) = env::var("CLICKUP_TEST_LIST_ID") {
        return Some(list_id);
    }

    // For now, return None - we'll need to find a list dynamically
    None
}

/// Helper function to get a test workspace ID
#[allow(dead_code)]
fn get_test_workspace_id() -> Option<String> {
    // Try to get from environment first
    if let Ok(workspace_id) = env::var("CLICKUP_TEST_WORKSPACE_ID") {
        return Some(workspace_id);
    }

    // For now, return None - we'll need to find a workspace dynamically
    None
}

/// Helper function to extract task ID from CLI output
fn extract_task_id(output: &str) -> Option<String> {
    // Look for patterns like "Task created with ID: abc123" or similar
    if let Some(id_start) = output.find("ID:") {
        let after_id = &output[id_start + 4..];
        if let Some(id_end) = after_id.find('\n') {
            return Some(after_id[..id_end].trim().to_string());
        }
    }

    // Alternative: look for UUID-like patterns
    let uuid_pattern =
        regex::Regex::new(r"[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}").unwrap();
    if let Some(captures) = uuid_pattern.captures(output) {
        return Some(captures[0].to_string());
    }

    None
}

/// Helper function to extract comment ID from CLI output
#[allow(dead_code)]
fn extract_comment_id(output: &str) -> Option<String> {
    // Look for patterns like "Comment created with ID: abc123" or similar
    if let Some(id_start) = output.find("ID:") {
        let after_id = &output[id_start + 4..];
        if let Some(id_end) = after_id.find('\n') {
            return Some(after_id[..id_end].trim().to_string());
        }
    }

    None
}

/// Helper to get or create a test workspace ID
fn get_or_discover_workspace_id() -> Option<String> {
    if let Ok(id) = std::env::var("CLICKUP_TEST_WORKSPACE_ID") {
        if !id.trim().is_empty() {
            return Some(id);
        }
    }
    // Discover first workspace via CLI using test environment
    let output = run_cli_with_test_env(&["workspaces", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    // DEBUG: Print the raw output for diagnosis
    eprintln!("[DEBUG] Raw output from 'workspaces list':\n{}", stdout);
    // Try to find a workspace ID in the output (assume table format)
    for line in stdout.lines() {
        if line.trim().starts_with("|")
            && line.contains("|")
            && line.chars().filter(|&c| c == '|').count() > 2
        {
            let parts: Vec<_> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() > 1 && parts[1].chars().all(|c| c.is_digit(10)) {
                return Some(parts[1].to_string());
            }
        }
    }
    None
}

/// Helper to get or discover a space ID in a workspace
fn get_or_discover_space_id(workspace_id: &str) -> Option<String> {
    let output = run_cli_with_test_env(&["spaces", "list", "--workspace-id", workspace_id]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.trim().starts_with("|")
            && line.contains("|")
            && line.chars().filter(|&c| c == '|').count() > 2
        {
            let parts: Vec<_> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() > 1 && parts[1].chars().all(|c| c.is_digit(10)) {
                return Some(parts[1].to_string());
            }
        }
    }
    None
}

/// Helper to get or discover a test list in a space
fn get_or_discover_test_list_id(space_id: &str) -> Option<(String, bool)> {
    if let Ok(id) = std::env::var("CLICKUP_TEST_LIST_ID") {
        if !id.trim().is_empty() {
            return Some((id, false)); // false = not created by test
        }
    }
    // Discover first available list in the space using test environment
    let output = run_cli_with_test_env(&["lists", "list", "--space-id", space_id]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Try to find a list ID in the output (assume table format)
    for line in stdout.lines() {
        if line.trim().starts_with("|")
            && line.contains("|")
            && line.chars().filter(|&c| c == '|').count() > 2
        {
            let parts: Vec<_> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() > 1 && parts[1].chars().all(|c| c.is_digit(10)) {
                return Some((parts[1].to_string(), false)); // false = not created by test
            }
        }
    }
    None
}

/// Helper to delete a list by ID
fn delete_list(list_id: &str) {
    let _ = Command::cargo_bin("clickup-cli")
        .unwrap()
        .args(["lists", "delete", "--id", list_id])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

/// Helper to get allowed statuses for a list
fn get_list_statuses(list_id: &str) -> Vec<String> {
    let output = Command::cargo_bin("clickup-cli")
        .unwrap()
        .args(["lists", "show", "--id", list_id])
        .output()
        .ok();
    if let Some(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut statuses = Vec::new();
        for line in stdout.lines() {
            if line.trim().starts_with("Status:") {
                // Example: Status: to do (color: #d3d3d3, type: open)
                if let Some(status) = line.split(':').nth(1) {
                    let status = status.trim().split(' ').next().unwrap_or("");
                    if !status.is_empty() {
                        statuses.push(status.to_string());
                    }
                }
            }
        }
        statuses
    } else {
        vec![]
    }
}

/// Tests that the CLI can authenticate and access the API
#[test]
#[ignore]
fn test_authentication() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Test that we can list workspaces (which requires authentication)
    let output = run_cli_with_test_env(&["workspaces", "list"]);

    // Should succeed and return some output
    assert!(
        output.status.success(),
        "Authentication failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.len() > 0, "No output from workspaces list");

    // Verify the output contains workspace information
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("Workspace list output: {}", output_str);
    assert!(
        output_str.contains("ID") || output_str.contains("Name"),
        "Output should contain workspace information"
    );
}

/// Tests that the CLI can list workspaces and teams
#[test]
#[ignore]
fn test_workspace_team_listing() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Test listing workspaces
    let output = run_cli_with_test_env(&["workspaces", "list"]);

    // Should succeed and return some output
    assert!(
        output.status.success(),
        "Workspace listing failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.len() > 0, "No output from workspaces list");

    // Test listing teams (if any exist)
    let output = run_cli_with_test_env(&["teams", "list"]);

    // Should succeed (even if no teams exist)
    assert!(
        output.status.success(),
        "Team listing failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Tests the full task lifecycle: create, read, update, delete
#[test]
#[ignore]
fn test_task_lifecycle() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Discover workspace, space, and create a test list if needed
    let workspace_id = match get_or_discover_workspace_id() {
        Some(id) => id,
        None => {
            eprintln!("Skipping test: No workspace found");
            return;
        }
    };
    let space_id = match get_or_discover_space_id(&workspace_id) {
        Some(id) => id,
        None => {
            eprintln!("Skipping test: No space found");
            return;
        }
    };
    let (list_id, created) = match get_or_discover_test_list_id(&space_id) {
        Some((id, created)) => (id, created),
        None => {
            eprintln!("Skipping test: No list found");
            return;
        }
    };

    let task_name = format!("Test Task {}", chrono::Utc::now().timestamp());
    let updated_task_name = format!("Updated Test Task {}", chrono::Utc::now().timestamp());

    // 1. Create a task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "tasks",
        "create",
        "--list-id",
        &list_id,
        "--name",
        &task_name,
        "--description",
        "Integration test task",
        "--status",
        "to do",
        "--priority",
        "2",
    ]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task creation failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Extract task ID from output
    let output_str = String::from_utf8_lossy(&output.stdout);
    let task_id = match extract_task_id(&output_str) {
        Some(id) => id,
        None => {
            eprintln!("Could not extract task ID from output: {}", output_str);
            if created {
                delete_list(&list_id);
            }
            return;
        }
    };

    // 2. Show the task to verify it was created correctly
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["tasks", "show", "--id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task show failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(
        output_str.contains(&task_name),
        "Task name not found in output"
    );
    assert!(
        output_str.contains("to do"),
        "Task status not found in output"
    );

    // Fetch allowed statuses for the list
    let statuses = get_list_statuses(&list_id);
    let update_status = statuses
        .iter()
        .find(|s| *s != "to do")
        .cloned()
        .unwrap_or_else(|| "complete".to_string());
    // 3. Update the task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "tasks",
        "update",
        "--id",
        &task_id,
        "--name",
        &updated_task_name,
        "--status",
        &update_status,
        "--priority",
        "3",
    ]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task update failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 4. Show the task again to verify the update
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["tasks", "show", "--id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task show after update failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(
        output_str.contains(&updated_task_name),
        "Updated task name not found in output"
    );
    assert!(
        output_str.contains(&update_status),
        "Updated task status not found in output"
    );

    // 5. Delete the task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["tasks", "delete", "--id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task deletion failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 6. Verify the task is deleted by trying to show it
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["tasks", "show", "--id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        !output.status.success(),
        "Task should be deleted and show should fail"
    );

    // Clean up the test list if we created it
    if created {
        delete_list(&list_id);
    }
}

/// Tests comment creation and listing
#[test]
#[ignore]
fn test_commenting() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Discover workspace, space, and create a test list if needed
    let workspace_id = match get_or_discover_workspace_id() {
        Some(id) => id,
        None => {
            eprintln!("Skipping test: No workspace found");
            return;
        }
    };
    let space_id = match get_or_discover_space_id(&workspace_id) {
        Some(id) => id,
        None => {
            eprintln!("Skipping test: No space found");
            return;
        }
    };
    let (list_id, created) = match get_or_discover_test_list_id(&space_id) {
        Some((id, created)) => (id, created),
        None => {
            eprintln!("Skipping test: No list found or could not create");
            return;
        }
    };

    let task_name = format!("Comment Test Task {}", chrono::Utc::now().timestamp());
    let comment_text = format!(
        "Integration test comment {}",
        chrono::Utc::now().timestamp()
    );
    let updated_comment_text = format!(
        "Updated integration test comment {}",
        chrono::Utc::now().timestamp()
    );

    // 1. Create a task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "tasks",
        "create",
        "--list-id",
        &list_id,
        "--name",
        &task_name,
        "--description",
        "Task for comment testing",
    ]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task creation failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Extract task ID from output
    let output_str = String::from_utf8_lossy(&output.stdout);
    let task_id = match extract_task_id(&output_str) {
        Some(id) => id,
        None => {
            eprintln!("Could not extract task ID from output: {}", output_str);
            if created {
                delete_list(&list_id);
            }
            return;
        }
    };

    // 2. Add a comment to the task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "comments",
        "create",
        "--task-id",
        &task_id,
        "--text",
        &comment_text,
    ]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Comment creation failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 3. List comments and verify the new comment appears
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["comments", "list", "--task-id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Comment listing failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(
        output_str.contains(&comment_text),
        "Comment text not found in listing"
    );

    // 4. (Optional) Extract the comment ID for update/delete if needed
    // For now, just find the first comment with the expected text
    let comment_id = output_str
        .lines()
        .find(|line| line.contains(&comment_text))
        .and_then(|line| {
            // Try to extract the comment ID from the table row
            let parts: Vec<_> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() > 1 {
                Some(parts[1].to_string())
            } else {
                None
            }
        });
    let comment_id = match comment_id {
        Some(id) => id,
        None => {
            eprintln!("Could not extract comment ID from listing: {}", output_str);
            if created {
                delete_list(&list_id);
            }
            return;
        }
    };

    // 5. Update the comment
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "comments",
        "update",
        "--id",
        &comment_id,
        "--text",
        &updated_comment_text,
    ]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Comment update failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 6. List comments again and verify the updated comment
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["comments", "list", "--task-id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Comment listing after update failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(
        output_str.contains(&updated_comment_text),
        "Updated comment text not found in listing"
    );

    // 7. Delete the comment
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["comments", "delete", "--id", &comment_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Comment deletion failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 8. Clean up - delete the task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["tasks", "delete", "--id", &task_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Task cleanup failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Clean up the test list if we created it
    if created {
        delete_list(&list_id);
    }
}

/// Tests error handling with invalid operations
#[test]
#[ignore]
fn test_error_handling() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Test 1: Try to show a non-existent task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["tasks", "show", "--id", "invalid-task-id-12345"]);

    let output = cmd.output().unwrap();

    // Should fail with an error
    assert!(
        !output.status.success(),
        "Expected failure for non-existent task"
    );
    assert!(output.stderr.len() > 0, "Should have error output");

    // Test 2: Try to create a task with invalid list ID
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "tasks",
        "create",
        "--list-id",
        "invalid-list-id-12345",
        "--name",
        "Test Task",
    ]);

    let output = cmd.output().unwrap();

    // Should fail with an error
    assert!(
        !output.status.success(),
        "Expected failure for invalid list ID"
    );
    assert!(output.stderr.len() > 0, "Should have error output");

    // Test 3: Try to show a non-existent list
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["lists", "show", "--id", "invalid-list-id-12345"]);

    let output = cmd.output().unwrap();

    // Should fail with an error
    assert!(
        !output.status.success(),
        "Expected failure for non-existent list"
    );
    assert!(output.stderr.len() > 0, "Should have error output");

    // Test 4: Try to add a comment to a non-existent task
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "comments",
        "create",
        "--task-id",
        "invalid-task-id-12345",
        "--text",
        "Test comment",
    ]);

    let output = cmd.output().unwrap();

    // Should fail with an error
    assert!(
        !output.status.success(),
        "Expected failure for non-existent task in comment creation"
    );
    assert!(output.stderr.len() > 0, "Should have error output");

    // Test 5: Try to update a non-existent comment
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args([
        "comments",
        "update",
        "--id",
        "invalid-comment-id-12345",
        "--text",
        "Updated comment",
    ]);

    let output = cmd.output().unwrap();

    // Should fail with an error
    assert!(
        !output.status.success(),
        "Expected failure for non-existent comment"
    );
    assert!(output.stderr.len() > 0, "Should have error output");
}

/// Tests rate limiting handling
#[test]
#[ignore]
fn test_rate_limiting() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Make multiple rapid requests to potentially trigger rate limiting
    let mut success_count = 0;
    let mut rate_limit_count = 0;

    for _i in 0..10 {
        let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
        cmd.args(["workspaces", "list"]);

        let output = cmd.output().unwrap();

        if output.status.success() {
            success_count += 1;
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("429")
                || stderr.contains("rate limit")
                || stderr.contains("too many requests")
            {
                rate_limit_count += 1;
            }
        }

        // Small delay between requests
        std::thread::sleep(Duration::from_millis(100));
    }

    // At least some requests should succeed
    assert!(
        success_count > 0,
        "No requests succeeded, possible authentication issue"
    );

    // If we hit rate limits, that's expected behavior
    if rate_limit_count > 0 {
        println!(
            "Rate limiting detected in {} out of 10 requests",
            rate_limit_count
        );
    }
}

/// Tests list operations
#[test]
#[ignore]
fn test_list_operations() {
    load_env();

    if !has_test_token() {
        eprintln!("Skipping test: CLICKUP_API_TOKEN_TEST not set");
        return;
    }

    let _temp_dir = setup_test_env();

    // Get a test workspace ID
    let workspace_id = match get_or_discover_workspace_id() {
        Some(id) => id,
        None => {
            eprintln!("Skipping test: No test workspace ID available");
            return;
        }
    };

    // Test listing spaces in the workspace
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["spaces", "list", "--workspace-id", &workspace_id]);

    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Space listing failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // If we have a test list ID, test listing tasks in that list
    if let Some(list_id) = get_test_list_id() {
        let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
        cmd.args(["tasks", "list", "--list-id", &list_id]);

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Task listing failed: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Tests authentication with invalid token
#[test]
#[ignore]
#[serial_test::serial]
fn test_invalid_authentication() {
    load_env();

    let _temp_dir = setup_test_env();

    // Temporarily set an invalid token and remove config token
    let original_token = std::env::var("CLICKUP_API_TOKEN_TEST").ok();
    std::env::set_var("CLICKUP_API_TOKEN", "invalid-token-12345");
    std::env::set_var("CLICKUP_API_TOKEN_TEST", "invalid-token-12345");
    // Remove config file if it exists
    if let Some(config_dir) = dirs::config_dir() {
        let config_file = config_dir.join("clickup-cli").join("config.toml");
        let _ = std::fs::remove_file(config_file);
    }

    // Test that the CLI fails gracefully with invalid token
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["workspaces", "list"]);

    let output = cmd.output().unwrap();

    // Should fail with an error
    assert!(
        !output.status.success(),
        "Expected failure with invalid token"
    );
    assert!(output.stderr.len() > 0, "Should have error output");

    // Restore original token if it existed
    if let Some(token) = original_token {
        std::env::set_var("CLICKUP_API_TOKEN_TEST", token);
    } else {
        std::env::remove_var("CLICKUP_API_TOKEN_TEST");
    }
    std::env::remove_var("CLICKUP_API_TOKEN");
}

/// Tests CLI help and version commands
#[test]
fn test_cli_basic_commands() {
    let _temp_dir = setup_test_env();

    // Test help command
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["--help"]);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "Help command failed");
    assert!(output.stdout.len() > 0, "Help output should not be empty");

    // Test version command
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["--version"]);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "Version command failed");
    assert!(
        output.stdout.len() > 0,
        "Version output should not be empty"
    );

    // Test invalid command
    let mut cmd = Command::cargo_bin("clickup-cli").unwrap();
    cmd.args(["invalid-command"]);

    let output = cmd.output().unwrap();
    assert!(!output.status.success(), "Invalid command should fail");
    assert!(output.stderr.len() > 0, "Error output should not be empty");
}
