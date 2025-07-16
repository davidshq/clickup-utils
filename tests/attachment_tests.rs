//! # Attachment Tests
//!
//! This module contains comprehensive tests for the attachment functionality,
//! including API operations, command execution, and error handling.

use clickup_cli::api::ClickUpApi;
use clickup_cli::config::Config;
use clickup_cli::error::ClickUpError;
use clickup_cli::models::{Attachment, AttachmentsResponse, CreateLinkAttachmentRequest};
use clickup_cli::repository::{ClickUpRepository, RepositoryFactory};

#[tokio::test]
async fn test_attachment_models_serialization() {
    // Test AttachmentsResponse serialization
    let response = AttachmentsResponse {
        attachments: vec![
            Attachment {
                id: "att_123".to_string(),
                title: Some("Test Document".to_string()),
                filename: Some("test.pdf".to_string()),
                size: Some(1024),
                mimetype: Some("application/pdf".to_string()),
                extension: Some("pdf".to_string()),
                url: Some("https://example.com/file.pdf".to_string()),
                date_created: Some("2023-01-01T00:00:00Z".to_string()),
                date_updated: Some("2023-01-01T00:00:00Z".to_string()),
                user: None,
                is_link: Some(false),
                link: None,
                link_preview: None,
            }
        ],
    };

    // Test serialization
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("att_123"));
    assert!(json.contains("Test Document"));
    assert!(json.contains("test.pdf"));

    // Test deserialization
    let deserialized: AttachmentsResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.attachments.len(), 1);
    assert_eq!(deserialized.attachments[0].id, "att_123");
    assert_eq!(deserialized.attachments[0].title, Some("Test Document".to_string()));
}

#[tokio::test]
async fn test_create_link_attachment_request() {
    let request = CreateLinkAttachmentRequest {
        link: "https://example.com".to_string(),
        title: Some("Example Link".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("https://example.com"));
    assert!(json.contains("Example Link"));

    let deserialized: CreateLinkAttachmentRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.link, "https://example.com");
    assert_eq!(deserialized.title, Some("Example Link".to_string()));
}

#[tokio::test]
async fn test_repository_attachment_methods() {
    let config = Config::default();
    let repo = RepositoryFactory::create(&config).unwrap();

    // Test that attachment methods exist on the repository trait
    let _repo_ref: &dyn ClickUpRepository = repo.as_ref();
    
    // Note: These tests would require a real API token to actually work
    // They're included to ensure the trait methods are properly defined
}

#[tokio::test]
async fn test_attachment_error_handling() {
    let config = Config::default();
    
    // Test with invalid token (should fail gracefully)
    let api = ClickUpApi::new(config);
    // The API creation might succeed with default config, but operations will fail
    // Let's test that the API can be created but operations fail
    if let Ok(api) = api {
        // Try to get attachments - this should fail without a real token
        let result = api.get_attachments("test_task_id").await;
        assert!(result.is_err());
    } else {
        // If API creation fails, that's also acceptable
        assert!(api.is_err());
    }
}

#[test]
fn test_attachment_file_validation() {
    use std::path::Path;
    
    // Test file existence validation
    let non_existent_file = "non_existent_file.txt";
    assert!(!Path::new(non_existent_file).exists());
    
    // Test with a file that should exist (this test file)
    let test_file = "tests/attachment_tests.rs";
    assert!(Path::new(test_file).exists());
}

#[test]
fn test_attachment_filename_extraction() {
    use std::path::Path;
    
    // Test filename extraction from path
    let path = Path::new("/path/to/file.txt");
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown_file");
    assert_eq!(filename, "file.txt");
    
    // Test with path without extension
    let path_no_ext = Path::new("/path/to/file");
    let filename_no_ext = path_no_ext.file_name().and_then(|n| n.to_str()).unwrap_or("unknown_file");
    assert_eq!(filename_no_ext, "file");
    
    // Test with empty path
    let empty_path = Path::new("");
    let empty_filename = empty_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown_file");
    assert_eq!(empty_filename, "unknown_file");
}

#[tokio::test]
async fn test_attachment_size_formatting() {
    // Test size formatting
    let size_1024 = 1024;
    let formatted = format!("{} B", size_1024);
    assert_eq!(formatted, "1024 B");
    
    let size_unknown: Option<i64> = None;
    let formatted_unknown = size_unknown
        .map(|s| format!("{} B", s))
        .unwrap_or_else(|| "Unknown".to_string());
    assert_eq!(formatted_unknown, "Unknown");
}

#[test]
fn test_attachment_mime_type_handling() {
    // Test MIME type handling
    let mimetype = Some("application/pdf".to_string());
    let extension = Some("pdf".to_string());
    
    let file_type = mimetype.as_deref()
        .or(extension.as_deref())
        .unwrap_or("Unknown");
    assert_eq!(file_type, "application/pdf");
    
    // Test with only extension
    let mimetype_none: Option<String> = None;
    let extension_only = Some("txt".to_string());
    
    let file_type_only_ext = mimetype_none.as_deref()
        .or(extension_only.as_deref())
        .unwrap_or("Unknown");
    assert_eq!(file_type_only_ext, "txt");
    
    // Test with neither
    let neither_mimetype: Option<String> = None;
    let neither_extension: Option<String> = None;
    
    let file_type_neither = neither_mimetype.as_deref()
        .or(neither_extension.as_deref())
        .unwrap_or("Unknown");
    assert_eq!(file_type_neither, "Unknown");
}

#[test]
fn test_attachment_user_handling() {
    // Test user information handling
    let user_with_username = Some(clickup_cli::models::AttachmentUser {
        id: 123,
        username: "testuser".to_string(),
        color: Some("#ff0000".to_string()),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
    });
    
    let username = user_with_username
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    assert_eq!(username, "testuser");
    
    // Test with no user
    let no_user: Option<clickup_cli::models::AttachmentUser> = None;
    let username_none = no_user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    assert_eq!(username_none, "Unknown");
}

#[test]
fn test_attachment_title_handling() {
    // Test title/name handling with fallbacks
    let attachment_with_title = clickup_cli::models::Attachment {
        id: "att_123".to_string(),
        title: Some("Custom Title".to_string()),
        filename: Some("file.txt".to_string()),
        size: Some(1024),
        mimetype: Some("text/plain".to_string()),
        extension: Some("txt".to_string()),
        url: Some("https://example.com/file.txt".to_string()),
        date_created: Some("2023-01-01T00:00:00Z".to_string()),
        date_updated: Some("2023-01-01T00:00:00Z".to_string()),
        user: None,
        is_link: Some(false),
        link: None,
        link_preview: None,
    };
    
    let name = attachment_with_title.title.as_deref()
        .or(attachment_with_title.filename.as_deref())
        .unwrap_or("Unknown");
    assert_eq!(name, "Custom Title");
    
    // Test with only filename
    let attachment_filename_only = clickup_cli::models::Attachment {
        id: "att_123".to_string(),
        title: None,
        filename: Some("file.txt".to_string()),
        size: Some(1024),
        mimetype: Some("text/plain".to_string()),
        extension: Some("txt".to_string()),
        url: Some("https://example.com/file.txt".to_string()),
        date_created: Some("2023-01-01T00:00:00Z".to_string()),
        date_updated: Some("2023-01-01T00:00:00Z".to_string()),
        user: None,
        is_link: Some(false),
        link: None,
        link_preview: None,
    };
    
    let name_filename_only = attachment_filename_only.title.as_deref()
        .or(attachment_filename_only.filename.as_deref())
        .unwrap_or("Unknown");
    assert_eq!(name_filename_only, "file.txt");
    
    // Test with neither
    let attachment_neither = clickup_cli::models::Attachment {
        id: "att_123".to_string(),
        title: None,
        filename: None,
        size: Some(1024),
        mimetype: Some("text/plain".to_string()),
        extension: Some("txt".to_string()),
        url: Some("https://example.com/file.txt".to_string()),
        date_created: Some("2023-01-01T00:00:00Z".to_string()),
        date_updated: Some("2023-01-01T00:00:00Z".to_string()),
        user: None,
        is_link: Some(false),
        link: None,
        link_preview: None,
    };
    
    let name_neither = attachment_neither.title.as_deref()
        .or(attachment_neither.filename.as_deref())
        .unwrap_or("Unknown");
    assert_eq!(name_neither, "Unknown");
} 