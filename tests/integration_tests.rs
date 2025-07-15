//! # Integration Tests
//!
//! This module contains integration tests for the ClickUp CLI application.
//! These tests verify that the CLI works end-to-end with real command execution.

use std::env;
use std::process::Command;
use tempfile::TempDir;

/// Sets up a temporary environment for testing
fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    env::set_var("XDG_CONFIG_HOME", &temp_path);
    env::set_var("APPDATA", &temp_path);
    temp_dir
}

/// Tests that the CLI application can be invoked and shows help
#[test]
fn test_cli_help() {
    let _temp_dir = setup_test_env();

    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute CLI");

    assert!(output.status.success(), "CLI help command failed");
    assert!(!output.stdout.is_empty(), "Help output should not be empty");
}

/// Tests that the CLI shows version information
#[test]
fn test_cli_version() {
    let _temp_dir = setup_test_env();

    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute CLI");

    assert!(output.status.success(), "CLI version command failed");
    assert!(
        !output.stdout.is_empty(),
        "Version output should not be empty"
    );
}

/// Tests that the CLI handles invalid commands gracefully
#[test]
fn test_cli_invalid_command() {
    let _temp_dir = setup_test_env();

    let output = Command::new("cargo")
        .args(["run", "--", "invalid-command"])
        .output()
        .expect("Failed to execute CLI");

    // Should fail with an error code
    assert!(!output.status.success(), "Invalid command should fail");
    // Should have error output
    assert!(!output.stderr.is_empty(), "Error output should not be empty");
}
