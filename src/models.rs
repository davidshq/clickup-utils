//! # Data Models
//! 
//! This module defines all the data structures used for communicating with the ClickUp API.
//! It includes models for users, workspaces, spaces, lists, tasks, comments, and various
//! related entities.
//! 
//! ## Model Categories
//! 
//! The models are organized into logical categories:
//! - **User Models**: User information and authentication data
//! - **Workspace Models**: Workspace and team management
//! - **Space Models**: Space configuration and features
//! - **List Models**: List management and organization
//! - **Task Models**: Task data and metadata
//! - **Comment Models**: Comment system and threading
//! - **Request Models**: Data structures for API requests
//! 
//! ## Serialization
//! 
//! All models implement `Serialize` and `Deserialize` traits for JSON communication
//! with the ClickUp API. The models are designed to match the ClickUp API response
//! format exactly.
//! 
//! ## Usage
//! 
//! ```rust
//! use crate::models::{User, Task, CreateTaskRequest};
//! 
//! // Deserialize API response
//! let user: User = serde_json::from_str(json_data)?;
//! 
//! // Create request data
//! let task_request = CreateTaskRequest {
//!     name: "My Task".to_string(),
//!     description: Some("Task description".to_string()),
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};

// User models

/// Complete user information response from ClickUp API
/// 
/// This struct represents the top-level response when fetching user information.
/// It contains a single `UserData` struct with all user details.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's detailed information
    pub user: UserData,
}

/// Detailed user information
/// 
/// This struct contains all the information about a ClickUp user including
/// their profile, authentication details, and account settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's preferred color for UI display
    pub color: Option<String>,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
    /// User's role ID in the system
    pub role: Option<i64>,
    /// Custom role configuration (JSON object)
    pub custom_role: Option<serde_json::Value>,
    /// Timestamp of user's last activity
    pub last_active: Option<String>,
    /// Timestamp when user joined
    pub date_joined: Option<String>,
    /// Timestamp when user was invited
    pub date_invited: Option<String>,
}

// Workspace models

/// Response containing a list of workspaces
/// 
/// This struct represents the API response when fetching all accessible workspaces.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacesResponse {
    /// List of workspaces accessible to the user
    pub teams: Vec<Workspace>,
}

/// Workspace (team) information
/// 
/// A workspace represents a top-level organizational unit in ClickUp.
/// It contains spaces, members, and role definitions.
#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// Unique workspace identifier
    pub id: String,
    /// Workspace display name
    pub name: String,
    /// Workspace color for UI display
    pub color: Option<String>,
    /// URL to workspace avatar
    pub avatar: Option<String>,
    /// List of workspace members
    pub members: Vec<WorkspaceMember>,
    /// List of available roles in the workspace
    pub roles: Vec<WorkspaceRole>,
}

/// Workspace member information
/// 
/// This struct represents a user who is a member of a workspace.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceMember {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's preferred color
    pub color: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
}

/// Workspace role definition
/// 
/// This struct defines a role that can be assigned to users within a workspace.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceRole {
    /// Unique role identifier
    pub id: i64,
    /// Role display name
    pub name: String,
    /// Role key for programmatic access
    pub key: String,
    /// Role color for UI display
    pub color: Option<String>,
    /// Role display order
    pub orderindex: i64,
}

// Space models

/// Response containing a list of spaces
/// 
/// This struct represents the API response when fetching spaces within a workspace.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpacesResponse {
    /// List of spaces in the workspace
    pub spaces: Vec<Space>,
}

/// Space information
/// 
/// A space represents a project or organizational unit within a workspace.
/// It contains lists, tasks, and has specific features enabled.
#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    /// Unique space identifier
    pub id: String,
    /// Space display name
    pub name: String,
    /// Whether the space is private
    pub private: bool,
    /// Available statuses in the space
    pub statuses: Vec<SpaceStatus>,
    /// Whether multiple assignees are allowed
    pub multiple_assignees: bool,
    /// Space features configuration
    pub features: SpaceFeatures,
}

/// Space status definition
/// 
/// This struct defines a status that can be assigned to tasks within a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceStatus {
    /// Unique status identifier
    pub id: String,
    /// Status display name
    pub status: String,
    /// Status type (e.g., "open", "closed")
    pub type_: String,
    /// Status display order
    pub orderindex: i64,
    /// Status color for UI display
    pub color: String,
}

/// Space features configuration
/// 
/// This struct defines which features are enabled for a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceFeatures {
    /// Due date functionality
    pub due_dates: SpaceFeature,
    /// Sprint functionality
    pub sprints: SpaceFeature,
    /// Time tracking functionality
    pub time_tracking: SpaceFeature,
    /// Story points functionality
    pub points: SpaceFeature,
    /// Custom items functionality
    pub custom_items: SpaceFeature,
    /// Priority functionality
    pub priorities: SpaceFeature,
    /// Tag functionality
    pub tags: SpaceFeature,
    /// Time estimates functionality
    pub time_estimates: SpaceFeature,
    /// Check unresolved functionality
    pub check_unresolved: SpaceFeature,
    /// Zoom integration
    pub zoom: SpaceFeature,
    /// Milestone functionality
    pub milestones: SpaceFeature,
    /// Custom fields functionality
    pub custom_fields: SpaceFeature,
    /// Dependency warning functionality
    pub dependency_warning: SpaceFeature,
    /// Multiple assignees functionality
    pub multiple_assignees: SpaceFeature,
}

/// Individual space feature configuration
/// 
/// This struct defines whether a specific feature is enabled in a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
}

// List models

/// Response containing a list of lists
/// 
/// This struct represents the API response when fetching lists within a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListsResponse {
    /// List of lists in the space
    pub lists: Vec<List>,
}

/// List information
/// 
/// A list represents a collection of tasks within a space.
/// It can be organized in folders and has various metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    /// Unique list identifier
    pub id: String,
    /// List display name
    pub name: String,
    /// List display order
    pub orderindex: i64,
    /// List description content
    pub content: String,
    /// Current list status
    pub status: Option<serde_json::Value>,
    /// List priority setting
    pub priority: Option<serde_json::Value>,
    /// List assignee
    pub assignee: Option<serde_json::Value>,
    /// Number of tasks in the list
    pub task_count: Option<String>,
    /// List due date
    pub due_date: Option<String>,
    /// List start date
    pub start_date: Option<String>,
    /// Parent folder information
    pub folder: Option<Folder>,
    /// Parent space information
    pub space: Option<Space>,
    /// Inbound email address for the list
    pub inbound_address: Option<String>,
}

/// Folder information
/// 
/// A folder is a container for lists within a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    /// Unique folder identifier
    pub id: String,
    /// Folder display name
    pub name: String,
    /// Whether the folder is hidden
    pub hidden: bool,
    /// Whether the user has access to the folder
    pub access: bool,
}

// Task models

/// Response containing a list of tasks
/// 
/// This struct represents the API response when fetching tasks within a list.
#[derive(Debug, Serialize, Deserialize)]
pub struct TasksResponse {
    /// List of tasks in the list
    pub tasks: Vec<Task>,
}

/// Task information
/// 
/// A task represents a work item within a list. It contains all the details
/// about the task including status, assignees, dates, and metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier
    pub id: String,
    /// Task display name
    pub name: String,
    /// Task description in text format
    pub text_content: Option<String>,
    /// Task description in rich format
    pub description: Option<String>,
    /// Current task status
    pub status: TaskStatus,
    /// Task display order
    pub orderindex: String,
    /// Task creation timestamp
    pub date_created: String,
    /// Task last update timestamp
    pub date_updated: String,
    /// Task completion timestamp
    pub date_closed: Option<String>,
    /// Task creator information
    pub creator: TaskCreator,
    /// List of task assignees
    pub assignees: Vec<TaskAssignee>,
    /// List of task checklists
    pub checklists: Vec<TaskChecklist>,
    /// List of task tags
    pub tags: Vec<TaskTag>,
    /// Parent task identifier (for subtasks)
    pub parent: Option<String>,
    /// Task priority
    pub priority: Option<TaskPriority>,
    /// Task due date
    pub due_date: Option<String>,
    /// Task start date
    pub start_date: Option<String>,
    /// Task story points
    pub points: Option<i64>,
    /// Task time estimate (in milliseconds)
    pub time_estimate: Option<i64>,
    /// Task time spent (in milliseconds)
    pub time_spent: Option<i64>,
    /// List of task custom fields
    pub custom_fields: Vec<TaskCustomField>,
    /// Parent list information
    pub list: TaskList,
    /// Parent folder information
    pub folder: Option<TaskFolder>,
    /// Parent space information
    pub space: TaskSpace,
    /// Task URL in ClickUp
    pub url: String,
}

/// Task status information
/// 
/// This struct defines the current status of a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Status display name
    pub status: String,
    /// Status color for UI display
    pub color: String,
    /// Status display order
    pub orderindex: i64,
    /// Status type
    pub type_: String,
}

/// Task creator information
/// 
/// This struct contains information about the user who created the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreator {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's preferred color
    pub color: Option<String>,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
}

/// Task assignee information
/// 
/// This struct contains information about a user assigned to the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignee {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's preferred color
    pub color: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
}

/// Task checklist information
/// 
/// This struct represents a checklist within a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskChecklist {
    /// Unique checklist identifier
    pub task_id: String,
    /// Checklist display name
    pub name: String,
    /// Checklist creation timestamp
    pub date_created: String,
    /// Checklist display order
    pub orderindex: i64,
    /// Checklist creator user ID
    pub creator: i64,
    /// Number of resolved checklist items
    pub resolved: i64,
    /// Number of unresolved checklist items
    pub unresolved: i64,
    /// List of checklist items
    pub items: Vec<TaskChecklistItem>,
}

/// Task checklist item information
/// 
/// This struct represents an individual item within a task checklist.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskChecklistItem {
    /// Unique checklist item identifier
    pub id: String,
    /// Checklist item display name
    pub name: String,
    /// Checklist item display order
    pub orderindex: i64,
    /// Checklist item assignee
    pub assignee: Option<serde_json::Value>,
    /// Whether the checklist item is resolved
    pub resolved: bool,
    /// Parent checklist item (for nested items)
    pub parent: Option<serde_json::Value>,
    /// Checklist item creation timestamp
    pub date_created: String,
    /// Child checklist items
    pub children: Vec<serde_json::Value>,
}

/// Task tag information
/// 
/// This struct represents a tag assigned to a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskTag {
    /// Tag display name
    pub name: String,
    /// Tag foreground color
    pub tag_fg: String,
    /// Tag background color
    pub tag_bg: String,
    /// Tag creator user ID
    pub creator: i64,
}

/// Task priority information
/// 
/// This struct defines the priority level of a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskPriority {
    /// Priority identifier
    pub id: String,
    /// Priority display name
    pub priority: String,
    /// Priority color for UI display
    pub color: String,
    /// Priority display order
    pub orderindex: String,
}

/// Task custom field information
/// 
/// This struct represents a custom field value on a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCustomField {
    /// Custom field identifier
    pub id: String,
    /// Custom field display name
    pub name: String,
    /// Custom field type
    pub type_: String,
    /// Custom field type configuration
    pub type_config: Option<serde_json::Value>,
    /// Custom field value
    pub value: Option<serde_json::Value>,
}

/// Task list information
/// 
/// This struct contains information about the list containing the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    /// Unique list identifier
    pub id: String,
    /// List display name
    pub name: String,
    /// Whether the user has access to the list
    pub access: bool,
}

/// Task folder information
/// 
/// This struct contains information about the folder containing the task's list.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFolder {
    /// Unique folder identifier
    pub id: String,
    /// Folder display name
    pub name: String,
    /// Whether the folder is hidden
    pub hidden: bool,
    /// Whether the user has access to the folder
    pub access: bool,
}

/// Task space information
/// 
/// This struct contains information about the space containing the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpace {
    /// Unique space identifier
    pub id: String,
    /// Space display name
    pub name: String,
    /// Space color for UI display
    pub color: Option<String>,
    /// Whether the space is private
    pub private: bool,
    /// URL to space avatar
    pub avatar: Option<String>,
    /// Whether admins can manage the space
    pub admin_can_manage: Option<bool>,
    /// Available statuses in the space
    pub statuses: Vec<SpaceStatus>,
    /// Whether multiple assignees are allowed
    pub multiple_assignees: bool,
}

// Request models

/// Request data for creating a new task
/// 
/// This struct contains all the data needed to create a new task in ClickUp.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    /// Task display name (required)
    pub name: String,
    /// Task description
    pub description: Option<String>,
    /// Task status
    pub status: Option<String>,
    /// Task priority (1-4, where 1 is highest)
    pub priority: Option<i64>,
    /// Task due date (Unix timestamp in milliseconds)
    pub due_date: Option<i64>,
    /// Whether the due date includes time
    pub due_date_time: Option<bool>,
    /// Task time estimate (in milliseconds)
    pub time_estimate: Option<i64>,
    /// List of assignee user IDs
    pub assignees: Option<Vec<i64>>,
    /// List of tag names
    pub tags: Option<Vec<String>>,
    /// Parent task ID (for subtasks)
    pub parent: Option<String>,
    /// List of custom field values
    pub custom_fields: Option<Vec<CustomFieldValue>>,
}

/// Request data for updating an existing task
/// 
/// This struct contains the data needed to update an existing task in ClickUp.
/// All fields are optional, allowing partial updates.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    /// Task display name
    pub name: Option<String>,
    /// Task description
    pub description: Option<String>,
    /// Task status
    pub status: Option<String>,
    /// Task priority (1-4, where 1 is highest)
    pub priority: Option<i64>,
    /// Task due date (Unix timestamp in milliseconds)
    pub due_date: Option<i64>,
    /// Whether the due date includes time
    pub due_date_time: Option<bool>,
    /// Task time estimate (in milliseconds)
    pub time_estimate: Option<i64>,
    /// List of assignee user IDs
    pub assignees: Option<Vec<i64>>,
    /// List of tag names
    pub tags: Option<Vec<String>>,
    /// Parent task ID (for subtasks)
    pub parent: Option<String>,
    /// List of custom field values
    pub custom_fields: Option<Vec<CustomFieldValue>>,
}

/// Custom field value for task requests
/// 
/// This struct represents a custom field value when creating or updating tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFieldValue {
    /// Custom field identifier
    pub id: String,
    /// Custom field value (can be various types)
    pub value: serde_json::Value,
}

// Comment models

/// Response containing a list of comments
/// 
/// This struct represents the API response when fetching comments for a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsResponse {
    /// List of comments on the task
    pub comments: Vec<Comment>,
}

/// Comment information
/// 
/// This struct represents a comment on a task, including its content,
/// author, and metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    /// Unique comment identifier
    pub id: String,
    /// Comment content in rich format
    pub comment: Vec<CommentText>,
    /// Comment content in plain text
    pub comment_text: String,
    /// Comment author information
    pub user: CommentUser,
    /// Whether the comment is resolved
    pub resolved: bool,
    /// Comment assignee (if any)
    pub assignee: Option<serde_json::Value>,
    /// User who assigned the comment
    pub assignee_by: Option<serde_json::Value>,
    /// Comment reactions
    pub reaction: Option<serde_json::Value>,
    /// Comment creation timestamp
    pub date_created: String,
    /// Comment last update timestamp
    pub date_updated: String,
    /// Parent comment (for threaded comments)
    pub parent: Option<serde_json::Value>,
    /// Child comments (for threaded comments)
    pub children: Vec<serde_json::Value>,
}

/// Comment text segment
/// 
/// This struct represents a segment of rich text within a comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentText {
    /// Text content
    pub text: String,
    /// Text type (e.g., "text", "link", "mention")
    pub type_: String,
}

/// Comment user information
/// 
/// This struct contains information about the user who wrote the comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentUser {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's preferred color
    pub color: Option<String>,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
}

/// Request data for creating a new comment
/// 
/// This struct contains the data needed to create a new comment on a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// Comment text content (required)
    pub comment_text: String,
    /// User ID to assign the comment to
    pub assignee: Option<i64>,
    /// Whether to notify all users
    pub notify_all: Option<bool>,
} 