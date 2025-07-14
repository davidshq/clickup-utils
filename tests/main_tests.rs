use std::process::Command;

/// Test that the CLI can be invoked with help and version flags
#[test]
fn test_cli_help_and_version() {
    // Test help command
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Help command should succeed");
    assert!(output.stdout.len() > 0, "Help output should not be empty");

    // Test version command
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Version command should succeed");
    assert!(output.stdout.len() > 0, "Version output should not be empty");
}

/// Test that the CLI handles invalid commands gracefully
#[test]
fn test_cli_invalid_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "invalid-command"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Invalid command should fail");
    assert!(output.stderr.len() > 0, "Error output should not be empty");
}

/// Test that subcommands show help when requested
#[test]
fn test_cli_subcommand_help() {
    // Test auth subcommand help
    let output = Command::new("cargo")
        .args(["run", "--", "auth", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Auth help command should succeed");
    assert!(output.stdout.len() > 0, "Auth help output should not be empty");

    // Test workspaces subcommand help
    let output = Command::new("cargo")
        .args(["run", "--", "workspaces", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Workspaces help command should succeed");
    assert!(output.stdout.len() > 0, "Workspaces help output should not be empty");
}

/// Test that debug logging can be enabled
#[test]
fn test_cli_debug_logging() {
    let output = Command::new("cargo")
        .args(["run", "--", "--debug", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Debug help command should succeed");
    // Note: We can't easily verify debug output without more complex setup,
    // but we can verify the command doesn't crash with debug flag
}

/// Test that command routing works for auth commands
#[test]
fn test_cli_auth_command_routing() {
    let output = Command::new("cargo")
        .args(["run", "--", "auth", "set", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Auth set help command should succeed");
    assert!(output.stdout.len() > 0, "Auth set help output should not be empty");
}

/// Test that command routing works for workspace commands
#[test]
fn test_cli_workspace_command_routing() {
    let output = Command::new("cargo")
        .args(["run", "--", "workspaces", "list", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Workspaces list help command should succeed");
    assert!(output.stdout.len() > 0, "Workspaces list help output should not be empty");
}

/// Test that command routing works for task commands
#[test]
fn test_cli_task_command_routing() {
    let output = Command::new("cargo")
        .args(["run", "--", "tasks", "create", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Tasks create help command should succeed");
    assert!(output.stdout.len() > 0, "Tasks create help output should not be empty");
}

/// Test that the CLI handles configuration errors gracefully
#[test]
fn test_cli_configuration_error_handling() {
    // Test with a command that requires authentication but without a token
    // This should fail gracefully rather than panic
    let output = Command::new("cargo")
        .args(["run", "--", "auth", "get"])
        .env("CLICKUP_API_TOKEN", "")
        .output()
        .expect("Failed to execute command");

    // The command should fail due to missing token, but not panic
    assert!(!output.status.success(), "Command should fail with missing token");
    assert!(output.stderr.len() > 0, "Error output should not be empty");
} 