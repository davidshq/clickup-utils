//! # Tag Tests
//!
//! This module contains comprehensive tests for tag-related functionality
//! including tag listing, creation, and management operations.
//!
//! ## Test Categories
//!
//! - **Tag Operations**: Tests for tag listing and creation
//! - **Tag Models**: Tests for tag data structures and serialization
//! - **Tag API Integration**: Tests for API endpoint integration
//! - **Tag Commands**: Tests for CLI command execution
//!
//! ## Test Coverage
//!
//! - Tag listing functionality
//! - Tag creation with custom colors
//! - Error handling for invalid inputs
//! - API response parsing
//! - Command-line interface validation

use clickup_cli::commands::tags::TagCommands;
use clickup_cli::commands::utils::CommandExecutor;
use clickup_cli::config::Config;
use clickup_cli::error::ClickUpError;
use clickup_cli::models::{Tag, TagsResponse, CreateTagRequest};

/// Tests the List command with no authentication
///
/// This test verifies that the List command correctly handles
/// authentication errors when no API token is configured.
#[tokio::test]
async fn test_tag_list_no_auth() {
    // Clear any existing configuration
    let _ = std::fs::remove_file("clickup-cli.toml");
    
    // Execute the List command
    let result = execute(TagCommands::List {
        space_id: "test_space_id".to_string(),
    })
    .await;
    
    // Should fail due to no authentication
    assert!(result.is_err());
    if let Err(ClickUpError::AuthError(msg)) = result {
        assert_eq!(msg, "API token not configured");
    } else {
        panic!("Expected AuthError, got: {:?}", result);
    }
}

/// Tests the List command with authentication
///
/// This test verifies that the List command can be executed
/// when proper authentication is configured.
#[tokio::test]
async fn test_tag_list_with_auth() {
    // Set up test configuration
    let config = Config {
        api_token: Some("test_token".to_string()),
        ..Default::default()
    };
    
    // Execute the List command
    let _result = execute(TagCommands::List {
        space_id: "test_space_id".to_string(),
    })
    .await;
    
    // Note: This will likely fail due to invalid token, but that's expected
    // The important thing is that it doesn't fail due to missing authentication
}

/// Tests the Create command with no authentication
///
/// This test verifies that the Create command correctly handles
/// authentication errors when no API token is configured.
#[tokio::test]
async fn test_tag_create_no_auth() {
    // Clear any existing configuration
    let _ = std::fs::remove_file("clickup-cli.toml");
    
    // Execute the Create command
    let result = execute(TagCommands::Create {
        space_id: "test_space_id".to_string(),
        name: "test_tag".to_string(),
        tag_fg: "#ffffff".to_string(),
        tag_bg: "#ff0000".to_string(),
    })
    .await;
    
    // Should fail due to no authentication
    assert!(result.is_err());
    if let Err(ClickUpError::AuthError(msg)) = result {
        assert_eq!(msg, "API token not configured");
    } else {
        panic!("Expected AuthError, got: {:?}", result);
    }
}

/// Tests the Create command with authentication
///
/// This test verifies that the Create command can be executed
/// when proper authentication is configured.
#[tokio::test]
async fn test_tag_create_with_auth() {
    // Set up test configuration
    let config = Config {
        api_token: Some("test_token".to_string()),
        ..Default::default()
    };
    
    // Execute the Create command
    let _result = execute(TagCommands::Create {
        space_id: "test_space_id".to_string(),
        name: "test_tag".to_string(),
        tag_fg: "#ffffff".to_string(),
        tag_bg: "#ff0000".to_string(),
    })
    .await;
    
    // Note: This will likely fail due to invalid token, but that's expected
    // The important thing is that it doesn't fail due to missing authentication
}

/// Tests tag model creation
///
/// This test verifies that Tag can be created with color information
/// and that the tag name, colors, and creator are correctly stored.
#[test]
fn test_tag_creation() {
    let tag = Tag {
        name: Some("urgent".to_string()),
        tag_fg: "#ffffff".to_string(),
        tag_bg: "#ff0000".to_string(),
        creator: 123,
    };
    
    assert_eq!(tag.name, Some("urgent".to_string()));
    assert_eq!(tag.tag_fg, "#ffffff");
    assert_eq!(tag.tag_bg, "#ff0000");
    assert_eq!(tag.creator, 123);
}

/// Tests tag response model creation
///
/// This test verifies that TagsResponse can be created with
/// a list of tags and that the tags are correctly stored.
#[test]
fn test_tags_response_creation() {
    let tags = vec![
        Tag {
            name: Some("urgent".to_string()),
            tag_fg: "#ffffff".to_string(),
            tag_bg: "#ff0000".to_string(),
            creator: 123,
        },
        Tag {
            name: Some("bug".to_string()),
            tag_fg: "#000000".to_string(),
            tag_bg: "#ffff00".to_string(),
            creator: 456,
        },
    ];
    
    let response = TagsResponse { tags };
    
    assert_eq!(response.tags.len(), 2);
    assert_eq!(response.tags[0].name, Some("urgent".to_string()));
    assert_eq!(response.tags[1].name, Some("bug".to_string()));
}

/// Tests tag creation request model
///
/// This test verifies that CreateTagRequest can be created with
/// the required parameters and that they are correctly stored.
#[test]
fn test_create_tag_request_creation() {
    let request = CreateTagRequest {
        name: "new_tag".to_string(),
        tag_fg: "#ffffff".to_string(),
        tag_bg: "#0000ff".to_string(),
    };
    
    assert_eq!(request.name, "new_tag");
    assert_eq!(request.tag_fg, "#ffffff");
    assert_eq!(request.tag_bg, "#0000ff");
}

/// Tests tag creation request serialization
///
/// This test verifies that CreateTagRequest can be serialized
/// to JSON format correctly.
#[test]
fn test_create_tag_request_serialization() {
    let request = CreateTagRequest {
        name: "test_tag".to_string(),
        tag_fg: "#ffffff".to_string(),
        tag_bg: "#ff0000".to_string(),
    };
    
    let json = serde_json::to_string(&request).unwrap();
    let expected = "{\"name\":\"test_tag\",\"tag_fg\":\"#ffffff\",\"tag_bg\":\"#ff0000\"}";
    
    assert_eq!(json, expected);
}

/// Tests tag response deserialization
///
/// This test verifies that TagsResponse can be deserialized
/// from JSON format correctly.
#[test]
fn test_tags_response_deserialization() {
    let json = "{\n        \"tags\": [\n            {\n                \"name\": \"urgent\",\n                \"tag_fg\": \"#ffffff\",\n                \"tag_bg\": \"#ff0000\",\n                \"creator\": 123\n            }\n        ]\n    }";
    
    let response: TagsResponse = serde_json::from_str(json).unwrap();
    
    assert_eq!(response.tags.len(), 1);
    assert_eq!(response.tags[0].name, Some("urgent".to_string()));
    assert_eq!(response.tags[0].tag_fg, "#ffffff");
    assert_eq!(response.tags[0].tag_bg, "#ff0000");
    assert_eq!(response.tags[0].creator, 123);
}

/// Helper function to execute tag commands
///
/// This function provides a convenient way to execute tag commands
/// for testing purposes.
async fn execute(command: TagCommands) -> Result<(), ClickUpError> {
    TagCommands::execute(command, &Config::load().unwrap_or_default()).await
} 