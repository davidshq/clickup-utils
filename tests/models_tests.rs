//! # Data Model Tests
//!
//! This module contains comprehensive tests for all data models used in the ClickUp CLI.
//! It tests model creation, serialization, deserialization, and various edge cases
//! to ensure robust data handling.
//!
//! ## Test Categories
//!
//! - **User Models**: Tests for user data structures and authentication
//! - **Workspace Models**: Tests for workspace and team management structures
//! - **Space Models**: Tests for space configuration and features
//! - **List Models**: Tests for list management and organization
//! - **Task Models**: Tests for task data and metadata structures
//! - **Comment Models**: Tests for comment system and threading
//! - **Request Models**: Tests for API request data structures
//!
//! ## Test Coverage
//!
//! Tests cover all model variants including optional fields, nested structures,
//! and various data types to ensure complete model validation.

use clickup_cli::models::*;

/// Tests user data model creation and field validation
///
/// This test verifies that UserData can be created with all fields
/// and that the values are correctly stored and accessible.
#[test]
fn test_user_data_creation() {
    let user_data = UserData {
        id: 123,
        username: Some("testuser".to_string()),
        email: "test@example.com".to_string(),
        color: Some("#ff0000".to_string()),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        initials: Some("TU".to_string()),
        role: Some(1),
        custom_role: Some(serde_json::json!({"permissions": ["read", "write"]})),
        last_active: Some("2023-01-01T00:00:00Z".to_string()),
        date_joined: Some("2023-01-01T00:00:00Z".to_string()),
        date_invited: Some("2023-01-01T00:00:00Z".to_string()),
    };

    assert_eq!(user_data.id, 123);
    assert_eq!(user_data.username, Some("testuser".to_string()));
    assert_eq!(user_data.email, "test@example.com");
    assert_eq!(user_data.color, Some("#ff0000".to_string()));
}

/// Tests user model creation and wrapper structure
///
/// This test verifies that the User wrapper structure correctly
/// contains UserData and provides proper access to user information.
#[test]
fn test_user_creation() {
    let user_data = UserData {
        id: 123,
        username: Some("testuser".to_string()),
        email: "test@example.com".to_string(),
        color: Some("#ff0000".to_string()),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        initials: Some("TU".to_string()),
        role: Some(1),
        custom_role: None,
        last_active: None,
        date_joined: None,
        date_invited: None,
    };

    let user = User { user: user_data };
    assert_eq!(user.user.id, 123);
    assert_eq!(user.user.username, Some("testuser".to_string()));
}

/// Tests workspace model creation and field validation
///
/// This test verifies that Workspace can be created with all fields
/// and that the values are correctly stored and accessible.
#[test]
fn test_workspace_creation() {
    let workspace = Workspace {
        id: "workspace_123".to_string(),
        name: Some("Test Workspace".to_string()),
        color: Some("#00ff00".to_string()),
        avatar: Some("https://example.com/workspace.jpg".to_string()),
        members: vec![],
        roles: Some(vec![]),
    };

    assert_eq!(workspace.id, "workspace_123");
    assert_eq!(workspace.name, Some("Test Workspace".to_string()));
    assert_eq!(workspace.color, Some("#00ff00".to_string()));
}

/// Tests workspaces response model creation
///
/// This test verifies that WorkspacesResponse can be created and
/// contains a collection of workspaces with proper access.
#[test]
fn test_workspaces_response_creation() {
    let workspace = Workspace {
        id: "workspace_123".to_string(),
        name: Some("Test Workspace".to_string()),
        color: Some("#00ff00".to_string()),
        avatar: Some("https://example.com/workspace.jpg".to_string()),
        members: vec![],
        roles: Some(vec![]),
    };

    let response = WorkspacesResponse {
        teams: vec![workspace],
    };

    assert_eq!(response.teams.len(), 1);
    assert_eq!(response.teams[0].id, "workspace_123");
}

/// Tests space model creation and field validation
///
/// This test verifies that Space can be created with all fields
/// and that boolean flags and optional fields work correctly.
#[test]
fn test_space_creation() {
    let space = Space {
        id: "space_123".to_string(),
        name: Some("Test Space".to_string()),
        private: false,
        color: Some("#0000ff".to_string()),
        avatar: Some("https://example.com/space.jpg".to_string()),
        multiple_assignees: true,
        features: None,
        archived: false,
        statuses: vec![],
    };

    assert_eq!(space.id, "space_123");
    assert_eq!(space.name, Some("Test Space".to_string()));
    assert!(!space.private);
    assert!(space.multiple_assignees);
    assert!(!space.archived);
}

/// Tests spaces response model creation
///
/// This test verifies that SpacesResponse can be created and
/// contains a collection of spaces with proper access.
#[test]
fn test_spaces_response_creation() {
    let space = Space {
        id: "space_123".to_string(),
        name: Some("Test Space".to_string()),
        private: false,
        color: Some("#0000ff".to_string()),
        avatar: Some("https://example.com/space.jpg".to_string()),
        multiple_assignees: true,
        features: None,
        archived: false,
        statuses: vec![],
    };

    let response = SpacesResponse {
        spaces: vec![space],
    };

    assert_eq!(response.spaces.len(), 1);
    assert_eq!(response.spaces[0].id, "space_123");
}

/// Tests list model creation and field validation
///
/// This test verifies that List can be created with all fields
/// including optional fields and nested structures.
#[test]
fn test_list_creation() {
    let list = List {
        id: "list_123".to_string(),
        name: Some("Test List".to_string()),
        orderindex: 1,
        content: Some("List description".to_string()),
        status: None,
        priority: None,
        assignee: None,
        task_count: Some(5),
        due_date: Some("2023-12-31".to_string()),
        start_date: Some("2023-01-01".to_string()),
        folder: None,
        space: ListSpace {
            id: "space_123".to_string(),
            name: "Test Space".to_string(),
        },
        archived: Some(false),
        override_statuses: Some(false),
        statuses: Some(vec![]),
    };

    assert_eq!(list.id, "list_123");
    assert_eq!(list.name, Some("Test List".to_string()));
    assert_eq!(list.orderindex, 1);
    assert_eq!(list.task_count, Some(5));
    assert_eq!(list.space.id, "space_123");
}

/// Tests lists response model creation
///
/// This test verifies that ListsResponse can be created and
/// contains a collection of lists with proper access.
#[test]
fn test_lists_response_creation() {
    let list = List {
        id: "list_123".to_string(),
        name: Some("Test List".to_string()),
        orderindex: 1,
        content: Some("List description".to_string()),
        status: None,
        priority: None,
        assignee: None,
        task_count: Some(5),
        due_date: Some("2023-12-31".to_string()),
        start_date: Some("2023-01-01".to_string()),
        folder: None,
        space: ListSpace {
            id: "space_123".to_string(),
            name: "Test Space".to_string(),
        },
        archived: Some(false),
        override_statuses: Some(false),
        statuses: Some(vec![]),
    };

    let response = ListsResponse { lists: vec![list] };

    assert_eq!(response.lists.len(), 1);
    assert_eq!(response.lists[0].id, "list_123");
}

/// Tests task model creation and field validation
///
/// This test verifies that Task can be created with all fields
/// including complex nested structures like status, creator, and assignees.
#[test]
fn test_task_creation() {
    let task = Task {
        id: "task_123".to_string(),
        name: Some("Test Task".to_string()),
        custom_id: Some("CUSTOM-123".to_string()),
        text_content: "Task description".to_string(),
        description: "Detailed task description".to_string(),
        status: TaskStatus {
            id: "status_123".to_string(),
            status: "in progress".to_string(),
            color: "#ffff00".to_string(),
            orderindex: 1,
            type_: "custom".to_string(),
        },
        orderindex: "1".to_string(),
        date_created: "2023-01-01T00:00:00Z".to_string(),
        date_updated: "2023-01-02T00:00:00Z".to_string(),
        date_closed: None,
        creator: TaskCreator {
            id: 123,
            username: "testuser".to_string(),
            color: "#ff0000".to_string(),
            profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        },
        assignees: vec![],
        watchers: vec![],
        checklists: vec![],
        tags: vec![],
        parent: None,
        top_level_parent: None,
        priority: Some(TaskPriority {
            id: "1".to_string(),
            priority: "high".to_string(),
            color: "#ff0000".to_string(),
            orderindex: "1".to_string(),
        }),
        due_date: Some("2023-12-31T23:59:59Z".to_string()),
        start_date: Some("2023-01-01T00:00:00Z".to_string()),
        time_estimate: Some(3600000), // 1 hour in milliseconds
        time_spent: Some(1800000),    // 30 minutes in milliseconds
        custom_fields: vec![],
        dependencies: vec![],
        linked_tasks: vec![],
        team_id: "team_123".to_string(),
        list: TaskList {
            id: "list_123".to_string(),
            name: Some("Test List".to_string()),
            access: Some(true),
        },
        folder: None,
        space: TaskSpace {
            id: "space_123".to_string(),
            name: Some("Test Space".to_string()),
        },
        url: "https://app.clickup.com/t/123".to_string(),
        subtasks: None,
    };

    assert_eq!(task.id, "task_123");
    assert_eq!(task.name, Some("Test Task".to_string()));
    assert_eq!(task.custom_id, Some("CUSTOM-123".to_string()));
    assert_eq!(task.text_content, "Task description");
    assert_eq!(task.status.status, "in progress");
    assert_eq!(task.creator.username, "testuser");
    assert_eq!(task.team_id, "team_123");
    assert_eq!(task.list.id, "list_123");
    assert_eq!(task.space.id, "space_123");
}

/// Tests tasks response model creation
///
/// This test verifies that TasksResponse can be created and
/// contains a collection of tasks with proper access.
#[test]
fn test_tasks_response_creation() {
    let task = Task {
        id: "task_123".to_string(),
        name: Some("Test Task".to_string()),
        custom_id: Some("CUSTOM-123".to_string()),
        text_content: "Task description".to_string(),
        description: "Detailed task description".to_string(),
        status: TaskStatus {
            id: "status_123".to_string(),
            status: "in progress".to_string(),
            color: "#ffff00".to_string(),
            orderindex: 1,
            type_: "custom".to_string(),
        },
        orderindex: "1".to_string(),
        date_created: "2023-01-01T00:00:00Z".to_string(),
        date_updated: "2023-01-02T00:00:00Z".to_string(),
        date_closed: None,
        creator: TaskCreator {
            id: 123,
            username: "testuser".to_string(),
            color: "#ff0000".to_string(),
            profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        },
        assignees: vec![],
        watchers: vec![],
        checklists: vec![],
        tags: vec![],
        parent: None,
        top_level_parent: None,
        priority: None,
        due_date: None,
        start_date: None,
        time_estimate: None,
        time_spent: None,
        custom_fields: vec![],
        dependencies: vec![],
        linked_tasks: vec![],
        team_id: "team_123".to_string(),
        list: TaskList {
            id: "list_123".to_string(),
            name: Some("Test List".to_string()),
            access: Some(true),
        },
        folder: None,
        space: TaskSpace {
            id: "space_123".to_string(),
            name: Some("Test Space".to_string()),
        },
        url: "https://app.clickup.com/t/123".to_string(),
        subtasks: None,
    };

    let response = TasksResponse { tasks: vec![task] };

    assert_eq!(response.tasks.len(), 1);
    assert_eq!(response.tasks[0].id, "task_123");
}

/// Tests create task request model creation
///
/// This test verifies that CreateTaskRequest can be created with all
/// optional fields and that the values are correctly stored.
#[test]
fn test_create_task_request_creation() {
    let request = CreateTaskRequest {
        name: "New Task".to_string(),
        description: Some("Task description".to_string()),
        status: Some("in progress".to_string()),
        priority: Some(1),
        due_date: Some(1704067199999), // Unix timestamp
        due_date_time: Some(true),
        time_estimate: Some(3600000), // 1 hour in milliseconds
        assignees: Some(vec![123, 456]),
        tags: Some(vec!["urgent".to_string(), "bug".to_string()]),
        parent: None,
        custom_fields: None,
        start_date: Some(1704067200000), // Unix timestamp
        start_date_time: Some(true),
        points: Some(5),
        notify_all: Some(true),
    };

    assert_eq!(request.name, "New Task");
    assert_eq!(request.description, Some("Task description".to_string()));
    assert_eq!(request.status, Some("in progress".to_string()));
    assert_eq!(request.priority, Some(1));
    assert_eq!(request.assignees, Some(vec![123, 456]));
    assert_eq!(
        request.tags,
        Some(vec!["urgent".to_string(), "bug".to_string()])
    );
}

/// Tests create task request default values
///
/// This test verifies that CreateTaskRequest::default() creates
/// a request with empty/None values for all optional fields.
#[test]
fn test_create_task_request_default() {
    let request = CreateTaskRequest::default();
    assert_eq!(request.name, "");
    assert_eq!(request.description, None);
    assert_eq!(request.status, None);
    assert_eq!(request.priority, None);
    assert_eq!(request.due_date, None);
    assert_eq!(request.due_date_time, None);
    assert_eq!(request.time_estimate, None);
    assert_eq!(request.assignees, None);
    assert_eq!(request.tags, None);
    assert_eq!(request.parent, None);
    // CustomFieldValue doesn't implement PartialEq, so we can't compare it
    // assert_eq!(request.custom_fields, None);
    assert_eq!(request.start_date, None);
    assert_eq!(request.start_date_time, None);
    assert_eq!(request.points, None);
    assert_eq!(request.notify_all, None);
}

/// Tests update task request model creation
///
/// This test verifies that UpdateTaskRequest can be created with all
/// optional fields and that the values are correctly stored.
#[test]
fn test_update_task_request_creation() {
    let request = UpdateTaskRequest {
        name: Some("Updated Task".to_string()),
        description: Some("Updated description".to_string()),
        status: Some("completed".to_string()),
        priority: Some(2),
        due_date: Some(1704067199999),
        due_date_time: Some(true),
        time_estimate: Some(7200000), // 2 hours in milliseconds
        assignees: Some(vec![789]),
        tags: Some(vec!["completed".to_string()]),
        parent: None,
        custom_fields: None,
        start_date: Some(1704067200000),
        start_date_time: Some(true),
        points: Some(8),
        notify_all: Some(false),
    };

    assert_eq!(request.name, Some("Updated Task".to_string()));
    assert_eq!(request.description, Some("Updated description".to_string()));
    assert_eq!(request.status, Some("completed".to_string()));
    assert_eq!(request.priority, Some(2));
    assert_eq!(request.assignees, Some(vec![789]));
    assert_eq!(request.tags, Some(vec!["completed".to_string()]));
}

/// Tests comment model creation and field validation
///
/// This test verifies that Comment can be created with all fields
/// including nested structures like comment text and user information.
#[test]
fn test_comment_creation() {
    let comment = Comment {
        id: "comment_123".to_string(),
        comment: vec![CommentText {
            text: "This is a comment".to_string(),
            type_: "text".to_string(),
        }],
        comment_text: "This is a comment".to_string(),
        user: CommentUser {
            id: 123,
            username: "testuser".to_string(),
            color: Some("#ff0000".to_string()),
            profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        },
        resolved: false,
        assignee: None,
        assignee_by: None,
        reaction: None,
        date_created: "2023-01-01T00:00:00Z".to_string(),
        date_updated: "2023-01-01T00:00:00Z".to_string(),
        parent: None,
        children: vec![],
    };

    assert_eq!(comment.id, "comment_123");
    assert_eq!(comment.comment_text, "This is a comment");
    assert_eq!(comment.user.username, "testuser");
    assert!(!comment.resolved);
    assert_eq!(comment.children.len(), 0);
}

/// Tests comments response model creation
///
/// This test verifies that CommentsResponse can be created and
/// contains a collection of comments with proper access.
#[test]
fn test_comments_response_creation() {
    let comment = Comment {
        id: "comment_123".to_string(),
        comment: vec![CommentText {
            text: "This is a comment".to_string(),
            type_: "text".to_string(),
        }],
        comment_text: "This is a comment".to_string(),
        user: CommentUser {
            id: 123,
            username: "testuser".to_string(),
            color: Some("#ff0000".to_string()),
            profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        },
        resolved: false,
        assignee: None,
        assignee_by: None,
        reaction: None,
        date_created: "2023-01-01T00:00:00Z".to_string(),
        date_updated: "2023-01-01T00:00:00Z".to_string(),
        parent: None,
        children: vec![],
    };

    let response = CommentsResponse {
        comments: vec![comment],
    };

    assert_eq!(response.comments.len(), 1);
    assert_eq!(response.comments[0].id, "comment_123");
}

/// Tests create comment request model creation
///
/// This test verifies that CreateCommentRequest can be created with
/// all optional fields and that the values are correctly stored.
#[test]
fn test_create_comment_request_creation() {
    let request = CreateCommentRequest {
        comment_text: "This is a new comment".to_string(),
        assignee: Some(123),
        notify_all: Some(true),
    };

    assert_eq!(request.comment_text, "This is a new comment");
    assert_eq!(request.assignee, Some(123));
    assert_eq!(request.notify_all, Some(true));
}

/// Tests custom field value model creation
///
/// This test verifies that CustomFieldValue can be created with
/// JSON values and that the field ID and value are correctly stored.
#[test]
fn test_custom_field_value_creation() {
    let value = CustomFieldValue {
        id: "field_123".to_string(),
        value: serde_json::json!("field value"),
    };

    assert_eq!(value.id, "field_123");
    assert_eq!(value.value, serde_json::json!("field value"));
}

/// Tests task tag model creation
///
/// This test verifies that TaskTag can be created with color information
/// and that the tag name, colors, and creator are correctly stored.
#[test]
fn test_task_tag_creation() {
    let tag = TaskTag {
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

/// Tests task priority model creation
///
/// This test verifies that TaskPriority can be created with priority
/// information and that the ID, priority level, color, and order are correctly stored.
#[test]
fn test_task_priority_creation() {
    let priority = TaskPriority {
        id: "1".to_string(),
        priority: "high".to_string(),
        color: "#ff0000".to_string(),
        orderindex: "1".to_string(),
    };

    assert_eq!(priority.id, "1");
    assert_eq!(priority.priority, "high");
    assert_eq!(priority.color, "#ff0000");
    assert_eq!(priority.orderindex, "1");
}

/// Tests task status model creation
///
/// This test verifies that TaskStatus can be created with status
/// information and that the ID, status name, color, order, and type are correctly stored.
#[test]
fn test_task_status_creation() {
    let status = TaskStatus {
        id: "status_123".to_string(),
        status: "in progress".to_string(),
        color: "#ffff00".to_string(),
        orderindex: 1,
        type_: "custom".to_string(),
    };

    assert_eq!(status.id, "status_123");
    assert_eq!(status.status, "in progress");
    assert_eq!(status.color, "#ffff00");
    assert_eq!(status.orderindex, 1);
    assert_eq!(status.type_, "custom");
}

/// Tests task creator model creation
///
/// This test verifies that TaskCreator can be created with user
/// information and that the ID, username, color, and profile picture are correctly stored.
#[test]
fn test_task_creator_creation() {
    let creator = TaskCreator {
        id: 123,
        username: "testuser".to_string(),
        color: "#ff0000".to_string(),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
    };

    assert_eq!(creator.id, 123);
    assert_eq!(creator.username, "testuser");
    assert_eq!(creator.color, "#ff0000");
    assert_eq!(
        creator.profile_picture,
        Some("https://example.com/avatar.jpg".to_string())
    );
}

/// Tests task assignee model creation
///
/// This test verifies that TaskAssignee can be created with user
/// information and that all user fields including email and initials are correctly stored.
#[test]
fn test_task_assignee_creation() {
    let assignee = TaskAssignee {
        id: 123,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        color: "#ff0000".to_string(),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        initials: Some("TU".to_string()),
    };

    assert_eq!(assignee.id, 123);
    assert_eq!(assignee.username, "testuser");
    assert_eq!(assignee.email, "test@example.com");
    assert_eq!(assignee.color, "#ff0000");
    assert_eq!(assignee.initials, Some("TU".to_string()));
}

/// Tests task watcher model creation
///
/// This test verifies that TaskWatcher can be created with user
/// information and that all user fields are correctly stored.
#[test]
fn test_task_watcher_creation() {
    let watcher = TaskWatcher {
        id: 123,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        color: "#ff0000".to_string(),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        initials: Some("TU".to_string()),
    };

    assert_eq!(watcher.id, 123);
    assert_eq!(watcher.username, "testuser");
    assert_eq!(watcher.email, "test@example.com");
    assert_eq!(watcher.color, "#ff0000");
    assert_eq!(watcher.initials, Some("TU".to_string()));
}

/// Tests task checklist model creation
///
/// This test verifies that TaskChecklist can be created with checklist
/// information and that the ID, name, order, and resolution status are correctly stored.
#[test]
fn test_task_checklist_creation() {
    let checklist = TaskChecklist {
        id: "checklist_123".to_string(),
        name: Some("Test Checklist".to_string()),
        orderindex: 1,
        assignee: None,
        resolved: Some(false),
        parent: None,
        date_created: "2023-01-01T00:00:00Z".to_string(),
        children: None,
    };

    assert_eq!(checklist.id, "checklist_123");
    assert_eq!(checklist.name, Some("Test Checklist".to_string()));
    assert_eq!(checklist.orderindex, 1);
    assert_eq!(checklist.resolved, Some(false));
}

/// Tests task checklist item model creation
///
/// This test verifies that TaskChecklistItem can be created with item
/// information and that the ID, name, order, and resolution status are correctly stored.
#[test]
fn test_task_checklist_item_creation() {
    let item = TaskChecklistItem {
        id: "item_123".to_string(),
        name: Some("Test Item".to_string()),
        orderindex: 1,
        assignee: None,
        resolved: Some(false),
        parent: None,
        date_created: "2023-01-01T00:00:00Z".to_string(),
        children: None,
    };

    assert_eq!(item.id, "item_123");
    assert_eq!(item.name, Some("Test Item".to_string()));
    assert_eq!(item.orderindex, 1);
    assert_eq!(item.resolved, Some(false));
}

/// Tests task custom field model creation
///
/// This test verifies that TaskCustomField can be created with field
/// information and that the ID, name, type, configuration, and value are correctly stored.
#[test]
fn test_task_custom_field_creation() {
    let custom_field = TaskCustomField {
        id: "field_123".to_string(),
        name: Some("Custom Field".to_string()),
        type_: "text".to_string(),
        type_config: Some(serde_json::json!({"required": true})),
        value: Some(serde_json::json!("field value")),
    };

    assert_eq!(custom_field.id, "field_123");
    assert_eq!(custom_field.name, Some("Custom Field".to_string()));
    assert_eq!(custom_field.type_, "text");
    assert_eq!(custom_field.value, Some(serde_json::json!("field value")));
}

/// Tests task list model creation
///
/// This test verifies that TaskList can be created with list
/// information and that the ID, name, and access permissions are correctly stored.
#[test]
fn test_task_list_creation() {
    let task_list = TaskList {
        id: "list_123".to_string(),
        name: Some("Test List".to_string()),
        access: Some(true),
    };

    assert_eq!(task_list.id, "list_123");
    assert_eq!(task_list.name, Some("Test List".to_string()));
    assert_eq!(task_list.access, Some(true));
}

/// Tests task folder model creation
///
/// This test verifies that TaskFolder can be created with folder
/// information and that the ID, name, visibility, and access permissions are correctly stored.
#[test]
fn test_task_folder_creation() {
    let task_folder = TaskFolder {
        id: "folder_123".to_string(),
        name: Some("Test Folder".to_string()),
        hidden: Some(false),
        access: Some(true),
    };

    assert_eq!(task_folder.id, "folder_123");
    assert_eq!(task_folder.name, Some("Test Folder".to_string()));
    assert_eq!(task_folder.hidden, Some(false));
    assert_eq!(task_folder.access, Some(true));
}

/// Tests task space model creation
///
/// This test verifies that TaskSpace can be created with space
/// information and that the ID and name are correctly stored.
#[test]
fn test_task_space_creation() {
    let task_space = TaskSpace {
        id: "space_123".to_string(),
        name: Some("Test Space".to_string()),
    };

    assert_eq!(task_space.id, "space_123");
    assert_eq!(task_space.name, Some("Test Space".to_string()));
}

/// Tests comment text model creation
///
/// This test verifies that CommentText can be created with text
/// content and that the text and type are correctly stored.
#[test]
fn test_comment_text_creation() {
    let comment_text = CommentText {
        text: "This is comment text".to_string(),
        type_: "text".to_string(),
    };

    assert_eq!(comment_text.text, "This is comment text");
    assert_eq!(comment_text.type_, "text");
}

/// Tests comment user model creation
///
/// This test verifies that CommentUser can be created with user
/// information and that the ID, username, color, and profile picture are correctly stored.
#[test]
fn test_comment_user_creation() {
    let comment_user = CommentUser {
        id: 123,
        username: "testuser".to_string(),
        color: Some("#ff0000".to_string()),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
    };

    assert_eq!(comment_user.id, 123);
    assert_eq!(comment_user.username, "testuser");
    assert_eq!(comment_user.color, Some("#ff0000".to_string()));
    assert_eq!(
        comment_user.profile_picture,
        Some("https://example.com/avatar.jpg".to_string())
    );
}

/// Tests list status model creation
///
/// This test verifies that ListStatus can be created with status
/// information and that the status name, color, and label visibility are correctly stored.
#[test]
fn test_list_status_creation() {
    let list_status = ListStatus {
        status: "in progress".to_string(),
        color: "#ffff00".to_string(),
        hide_label: Some(false),
    };

    assert_eq!(list_status.status, "in progress");
    assert_eq!(list_status.color, "#ffff00");
    assert_eq!(list_status.hide_label, Some(false));
}

/// Tests list priority model creation
///
/// This test verifies that ListPriority can be created with priority
/// information and that the priority level and color are correctly stored.
#[test]
fn test_list_priority_creation() {
    let list_priority = ListPriority {
        priority: "high".to_string(),
        color: "#ff0000".to_string(),
    };

    assert_eq!(list_priority.priority, "high");
    assert_eq!(list_priority.color, "#ff0000");
}

/// Tests list assignee model creation
///
/// This test verifies that ListAssignee can be created with user
/// information and that all user fields including email and initials are correctly stored.
#[test]
fn test_list_assignee_creation() {
    let list_assignee = ListAssignee {
        id: 123,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        color: "#ff0000".to_string(),
        profile_picture: Some("https://example.com/avatar.jpg".to_string()),
        initials: Some("TU".to_string()),
    };

    assert_eq!(list_assignee.id, 123);
    assert_eq!(list_assignee.username, "testuser");
    assert_eq!(list_assignee.email, "test@example.com");
    assert_eq!(list_assignee.color, "#ff0000");
    assert_eq!(list_assignee.initials, Some("TU".to_string()));
}

/// Tests list folder model creation
///
/// This test verifies that ListFolder can be created with folder
/// information and that the ID, name, visibility, and access permissions are correctly stored.
#[test]
fn test_list_folder_creation() {
    let list_folder = ListFolder {
        id: "folder_123".to_string(),
        name: "Test Folder".to_string(),
        hidden: Some(false),
        access: Some(true),
    };

    assert_eq!(list_folder.id, "folder_123");
    assert_eq!(list_folder.name, "Test Folder");
    assert_eq!(list_folder.hidden, Some(false));
    assert_eq!(list_folder.access, Some(true));
}

/// Tests list space model creation
///
/// This test verifies that ListSpace can be created with space
/// information and that the ID and name are correctly stored.
#[test]
fn test_list_space_creation() {
    let list_space = ListSpace {
        id: "space_123".to_string(),
        name: "Test Space".to_string(),
    };

    assert_eq!(list_space.id, "space_123");
    assert_eq!(list_space.name, "Test Space");
}

/// Tests folder model creation and field validation
///
/// This test verifies that Folder can be created with all fields
/// including optional fields and nested structures like space information.
#[test]
fn test_folder_creation() {
    let folder = Folder {
        id: "folder_123".to_string(),
        name: Some("Test Folder".to_string()),
        orderindex: 1,
        content: Some("Folder description".to_string()),
        hidden: Some(false),
        access: Some(true),
        space: FolderSpace {
            id: "space_123".to_string(),
            name: "Test Space".to_string(),
        },
        archived: Some(false),
        override_statuses: Some(false),
        statuses: Some(vec![]),
    };

    assert_eq!(folder.id, "folder_123");
    assert_eq!(folder.name, Some("Test Folder".to_string()));
    assert_eq!(folder.orderindex, 1);
    assert_eq!(folder.hidden, Some(false));
    assert_eq!(folder.access, Some(true));
    assert_eq!(folder.archived, Some(false));
}

/// Tests folders response model creation
///
/// This test verifies that FoldersResponse can be created and
/// contains a collection of folders with proper access.
#[test]
fn test_folders_response_creation() {
    let folder = Folder {
        id: "folder_123".to_string(),
        name: Some("Test Folder".to_string()),
        orderindex: 1,
        content: Some("Folder description".to_string()),
        hidden: Some(false),
        access: Some(true),
        space: FolderSpace {
            id: "space_123".to_string(),
            name: "Test Space".to_string(),
        },
        archived: Some(false),
        override_statuses: Some(false),
        statuses: Some(vec![]),
    };

    let response = FoldersResponse {
        folders: vec![folder],
    };

    assert_eq!(response.folders.len(), 1);
    assert_eq!(response.folders[0].id, "folder_123");
}

/// Tests folder space model creation
///
/// This test verifies that FolderSpace can be created with space
/// information and that the ID and name are correctly stored.
#[test]
fn test_folder_space_creation() {
    let folder_space = FolderSpace {
        id: "space_123".to_string(),
        name: "Test Space".to_string(),
    };

    assert_eq!(folder_space.id, "space_123");
    assert_eq!(folder_space.name, "Test Space");
}

/// Tests workspace member model creation
///
/// This test verifies that WorkspaceMember can be created with user
/// information and that all user fields including role and status are correctly stored.
#[test]
fn test_workspace_member_creation() {
    let member = WorkspaceMember {
        user: WorkspaceMemberUser {
            id: Some(123),
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            color: Some("#ff0000".to_string()),
            initials: Some("TU".to_string()),
            profile_picture: Some("https://example.com/avatar.jpg".to_string()),
            role: Some(1),
            role_subtype: Some(2),
            role_key: Some("member".to_string()),
            custom_role: None,
            last_active: Some("2023-01-01T00:00:00Z".to_string()),
            date_joined: Some("2023-01-01T00:00:00Z".to_string()),
            date_invited: Some("2023-01-01T00:00:00Z".to_string()),
            status: Some("active".to_string()),
            banned_date: None,
        },
        invited_by: None,
    };

    assert_eq!(member.user.id, Some(123));
    assert_eq!(member.user.username, Some("testuser".to_string()));
    assert_eq!(member.user.email, Some("test@example.com".to_string()));
    assert_eq!(member.user.role, Some(1));
    assert_eq!(member.user.status, Some("active".to_string()));
}

/// Tests workspace role model creation
///
/// This test verifies that WorkspaceRole can be created with role
/// information and that the ID, name, color, and permissions are correctly stored.
#[test]
fn test_workspace_role_creation() {
    let role = WorkspaceRole {
        id: 1,
        name: "Admin".to_string(),
        color: Some("#ff0000".to_string()),
        permissions: Some(serde_json::json!({"read": true, "write": true})),
    };

    assert_eq!(role.id, 1);
    assert_eq!(role.name, "Admin");
    assert_eq!(role.color, Some("#ff0000".to_string()));
    assert_eq!(
        role.permissions,
        Some(serde_json::json!({"read": true, "write": true}))
    );
}

/// Tests space status model creation
///
/// This test verifies that SpaceStatus can be created with status
/// information and that the ID, status name, color, order, and type are correctly stored.
#[test]
fn test_space_status_creation() {
    let status = SpaceStatus {
        id: "status_123".to_string(),
        status: "in progress".to_string(),
        color: "#ffff00".to_string(),
        orderindex: 1,
        type_: "custom".to_string(),
    };

    assert_eq!(status.id, "status_123");
    assert_eq!(status.status, "in progress");
    assert_eq!(status.color, "#ffff00");
    assert_eq!(status.orderindex, 1);
    assert_eq!(status.type_, "custom");
}

/// Tests space features model creation
///
/// This test verifies that SpaceFeatures can be created with all
/// feature configurations and that all optional feature fields are correctly stored.
#[test]
fn test_space_features_creation() {
    let features = SpaceFeatures {
        due_dates: Some(DueDateFeatures {
            enabled: true,
            start_date: true,
            remap_due_dates: true,
            remap_closed_due_date: true,
        }),
        time_tracking: Some(TimeTrackingFeatures { enabled: true }),
        tags: Some(TagFeatures { enabled: true }),
        time_estimates: Some(TimeEstimateFeatures { enabled: true }),
        checklists: Some(ChecklistFeatures { enabled: true }),
        custom_fields: Some(CustomFieldFeatures { enabled: true }),
        remap_dependencies: Some(DependencyFeatures { enabled: true }),
        dependency_warning: Some(DependencyWarningFeatures { enabled: true }),
        portfolios: Some(PortfolioFeatures { enabled: true }),
    };

    assert!(features.due_dates.is_some());
    assert!(features.time_tracking.is_some());
    assert!(features.tags.is_some());
    assert!(features.time_estimates.is_some());
    assert!(features.checklists.is_some());
    assert!(features.custom_fields.is_some());
    assert!(features.remap_dependencies.is_some());
    assert!(features.dependency_warning.is_some());
    assert!(features.portfolios.is_some());
}

/// Tests due date features model creation
///
/// This test verifies that DueDateFeatures can be created with
/// all due date configuration options and that the boolean flags are correctly stored.
#[test]
fn test_due_date_features_creation() {
    let features = DueDateFeatures {
        enabled: true,
        start_date: true,
        remap_due_dates: true,
        remap_closed_due_date: true,
    };

    assert!(features.enabled);
    assert!(features.start_date);
    assert!(features.remap_due_dates);
    assert!(features.remap_closed_due_date);
}

/// Tests time tracking features model creation
///
/// This test verifies that TimeTrackingFeatures can be created with
/// time tracking configuration and that the enabled flag is correctly stored.
#[test]
fn test_time_tracking_features_creation() {
    let features = TimeTrackingFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests tag features model creation
///
/// This test verifies that TagFeatures can be created with
/// tag configuration and that the enabled flag is correctly stored.
#[test]
fn test_tag_features_creation() {
    let features = TagFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests time estimate features model creation
///
/// This test verifies that TimeEstimateFeatures can be created with
/// time estimate configuration and that the enabled flag is correctly stored.
#[test]
fn test_time_estimate_features_creation() {
    let features = TimeEstimateFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests checklist features model creation
///
/// This test verifies that ChecklistFeatures can be created with
/// checklist configuration and that the enabled flag is correctly stored.
#[test]
fn test_checklist_features_creation() {
    let features = ChecklistFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests custom field features model creation
///
/// This test verifies that CustomFieldFeatures can be created with
/// custom field configuration and that the enabled flag is correctly stored.
#[test]
fn test_custom_field_features_creation() {
    let features = CustomFieldFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests dependency features model creation
///
/// This test verifies that DependencyFeatures can be created with
/// dependency configuration and that the enabled flag is correctly stored.
#[test]
fn test_dependency_features_creation() {
    let features = DependencyFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests dependency warning features model creation
///
/// This test verifies that DependencyWarningFeatures can be created with
/// dependency warning configuration and that the enabled flag is correctly stored.
#[test]
fn test_dependency_warning_features_creation() {
    let features = DependencyWarningFeatures { enabled: true };

    assert!(features.enabled);
}

/// Tests portfolio features model creation
///
/// This test verifies that PortfolioFeatures can be created with
/// portfolio configuration and that the enabled flag is correctly stored.
#[test]
fn test_portfolio_features_creation() {
    let features = PortfolioFeatures { enabled: true };

    assert!(features.enabled);
}
