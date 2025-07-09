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

use serde::{Deserialize, Serialize, Deserializer};

// Custom deserializers for handling ClickUp API type inconsistencies

/// Deserializes a field that can be either a string or a number into a String
fn string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;
    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = String;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a number")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }
        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
    }
    deserializer.deserialize_any(StringOrNumberVisitor)
}

/// Deserializes a field that can be either a string or a number into an Option<i64>
fn opt_i64_from_string_or_number<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptI64Visitor;
    impl<'de> serde::de::Visitor<'de> for OptI64Visitor {
        type Value = Option<i64>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer, a string containing an integer, or null")
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_some<D2>(self, deserializer: D2) -> Result<Self::Value, D2::Error>
        where
            D2: serde::Deserializer<'de>,
        {
            let val: serde_json::Value = Deserialize::deserialize(deserializer)?;
            match val {
                serde_json::Value::Number(n) => n.as_i64().map(Some).ok_or_else(|| serde::de::Error::custom("invalid number for i64")),
                serde_json::Value::String(s) => s.parse::<i64>().map(Some).map_err(|_| serde::de::Error::custom("invalid string for i64")),
                _ => Err(serde::de::Error::custom("unexpected type for Option<i64>")),
            }
        }
    }
    deserializer.deserialize_option(OptI64Visitor)
}

/// Deserializes a field that can be either a string or a number into an i64
fn i64_from_string_or_number<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct I64Visitor;
    impl<'de> serde::de::Visitor<'de> for I64Visitor {
        type Value = i64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer or a string containing an integer")
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value as i64)
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse::<i64>().map_err(|_| serde::de::Error::custom("invalid string for i64"))
        }
        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse::<i64>().map_err(|_| serde::de::Error::custom("invalid string for i64"))
        }
    }
    deserializer.deserialize_any(I64Visitor)
}

/// Deserializes a field that can be either a string or a number into an Option<String>
fn opt_string_from_string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptStringVisitor;
    impl<'de> serde::de::Visitor<'de> for OptStringVisitor {
        type Value = Option<String>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string, a number, or null")
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_some<D2>(self, deserializer: D2) -> Result<Self::Value, D2::Error>
        where
            D2: serde::Deserializer<'de>,
        {
            let val: serde_json::Value = Deserialize::deserialize(deserializer)?;
            match val {
                serde_json::Value::String(s) => Ok(Some(s)),
                serde_json::Value::Number(n) => Ok(Some(n.to_string())),
                _ => Err(serde::de::Error::custom("unexpected type for Option<String>")),
            }
        }
    }
    deserializer.deserialize_option(OptStringVisitor)
}

/// Deserializes a field that can be either a bool or an integer (0/1) into a bool
fn bool_from_bool_or_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolOrIntVisitor;
    impl<'de> serde::de::Visitor<'de> for BoolOrIntVisitor {
        type Value = bool;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or 0/1 integer")
        }
        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(serde::de::Error::custom("invalid integer for bool")),
            }
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(serde::de::Error::custom("invalid integer for bool")),
            }
        }
    }
    deserializer.deserialize_any(BoolOrIntVisitor)
}

/// Deserializes a field that can be either a bool or an integer (0/1) into an Option<bool>
fn opt_bool_from_bool_or_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptBoolOrIntVisitor;
    impl<'de> serde::de::Visitor<'de> for OptBoolOrIntVisitor {
        type Value = Option<bool>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean, 0/1 integer, or null")
        }
        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                0 => Ok(Some(false)),
                1 => Ok(Some(true)),
                _ => Ok(None), // Invalid integer becomes None
            }
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                0 => Ok(Some(false)),
                1 => Ok(Some(true)),
                _ => Ok(None), // Invalid integer becomes None
            }
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }
    deserializer.deserialize_any(OptBoolOrIntVisitor)
}

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
    pub username: Option<String>,
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
    #[serde(deserialize_with = "string_or_number")]
    /// Unique workspace identifier
    pub id: String,
    /// Workspace display name
    pub name: Option<String>,
    /// Workspace color for UI display
    pub color: Option<String>,
    /// URL to workspace avatar
    pub avatar: Option<String>,
    /// List of workspace members
    pub members: Vec<WorkspaceMember>,
    /// List of available roles in the workspace
    pub roles: Option<Vec<WorkspaceRole>>,
}

/// Workspace member information
/// 
/// This struct represents a user who is a member of a workspace.
/// The API response nests user data under a 'user' field.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceMember {
    /// User information
    pub user: WorkspaceMemberUser,
    /// Information about who invited this user
    pub invited_by: Option<WorkspaceMemberUser>,
}

/// Workspace member user information
/// 
/// This struct contains the actual user data within a workspace member.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceMemberUser {
    /// Unique user identifier
    pub id: Option<i64>,
    /// User's display name
    pub username: Option<String>,
    /// User's email address
    pub email: Option<String>,
    pub color: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
    /// User's role in the workspace - can be string or number
    #[serde(deserialize_with = "opt_i64_from_string_or_number", default)]
    pub role: Option<i64>,
    /// User's role subtype - can be string or number
    #[serde(deserialize_with = "opt_i64_from_string_or_number", default)]
    pub role_subtype: Option<i64>,
    /// User's role key - can be string or number
    #[serde(deserialize_with = "opt_string_from_string_or_number", default)]
    pub role_key: Option<String>,
    /// Custom role configuration
    pub custom_role: Option<serde_json::Value>,
    /// Timestamp of user's last activity
    pub last_active: Option<String>,
    /// Timestamp when user joined
    pub date_joined: Option<String>,
    /// Timestamp when user was invited
    pub date_invited: Option<String>,
    /// User's status (e.g., "active", "banned")
    pub status: Option<String>,
    /// Date when user was banned (if applicable)
    pub banned_date: Option<String>,
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
    /// Role color for UI display
    pub color: Option<String>,
    /// Role permissions
    pub permissions: Option<serde_json::Value>,
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
/// A space represents a project or team within a workspace.
/// It contains lists, folders, and task organization.
#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique space identifier
    pub id: String,
    /// Space display name
    pub name: Option<String>,
    /// Whether the space is private
    #[serde(deserialize_with = "bool_from_bool_or_int")]
    pub private: bool,
    /// Space color for UI display
    pub color: Option<String>,
    /// URL to space avatar
    pub avatar: Option<String>,
    /// Whether the space allows multiple assignees
    pub multiple_assignees: bool,
    /// Space features configuration
    pub features: Option<SpaceFeatures>,
    /// Whether the space is archived
    pub archived: bool,
    /// Space statuses
    pub statuses: Vec<SpaceStatus>,
}

/// Space features configuration
/// 
/// This struct defines the features enabled for a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceFeatures {
    /// Due date features
    pub due_dates: Option<DueDateFeatures>,
    /// Time tracking features
    pub time_tracking: Option<TimeTrackingFeatures>,
    /// Tag features
    pub tags: Option<TagFeatures>,
    /// Time estimate features
    pub time_estimates: Option<TimeEstimateFeatures>,
    /// Checklist features
    pub checklists: Option<ChecklistFeatures>,
    /// Custom field features
    pub custom_fields: Option<CustomFieldFeatures>,
    /// Dependency features
    pub remap_dependencies: Option<DependencyFeatures>,
    /// Dependency warning features
    pub dependency_warning: Option<DependencyWarningFeatures>,
    /// Portfolio features
    pub portfolios: Option<PortfolioFeatures>,
}

/// Due date features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct DueDateFeatures {
    /// Whether due dates are enabled
    pub enabled: bool,
    /// Whether start dates are enabled
    pub start_date: bool,
    /// Whether due date remapping is enabled
    pub remap_due_dates: bool,
    /// Whether closed due date remapping is enabled
    pub remap_closed_due_date: bool,
}

/// Time tracking features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTrackingFeatures {
    /// Whether time tracking is enabled
    pub enabled: bool,
}

/// Tag features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct TagFeatures {
    /// Whether tags are enabled
    pub enabled: bool,
}

/// Time estimate features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEstimateFeatures {
    /// Whether time estimates are enabled
    pub enabled: bool,
}

/// Checklist features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ChecklistFeatures {
    /// Whether checklists are enabled
    pub enabled: bool,
}

/// Custom field features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFieldFeatures {
    /// Whether custom fields are enabled
    pub enabled: bool,
}

/// Dependency features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyFeatures {
    /// Whether dependency remapping is enabled
    pub enabled: bool,
}

/// Dependency warning features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyWarningFeatures {
    /// Whether dependency warnings are enabled
    pub enabled: bool,
}

/// Portfolio features configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioFeatures {
    /// Whether portfolios are enabled
    pub enabled: bool,
}

/// Space status information
/// 
/// This struct represents a status that can be assigned to tasks in a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceStatus {
    /// Unique status identifier
    pub id: String,
    /// Status display name
    pub status: String,
    /// Status color for UI display
    pub color: String,
    /// Status display order
    #[serde(deserialize_with = "i64_from_string_or_number")]
    pub orderindex: i64,
    /// Status type
    #[serde(rename = "type")]
    pub type_: String,
}

// Folder models

/// Response containing a list of folders
/// 
/// This struct represents the API response when fetching folders within a space.
#[derive(Debug, Serialize, Deserialize)]
pub struct FoldersResponse {
    /// List of folders in the space
    pub folders: Vec<Folder>,
}

/// Folder information
/// 
/// A folder represents a collection of lists within a space.
/// It provides organization for lists and can contain multiple lists.
#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique folder identifier
    pub id: String,
    /// Folder display name
    pub name: Option<String>,
    /// Folder display order
    #[serde(deserialize_with = "i64_from_string_or_number")]
    pub orderindex: i64,
    /// Folder content/description
    pub content: Option<String>,
    /// Whether the folder is hidden
    pub hidden: Option<bool>,
    /// Whether the user has access to the folder
    pub access: Option<bool>,
    /// Folder space information
    pub space: FolderSpace,
    /// Whether the folder is archived
    pub archived: Option<bool>,
    /// Whether the folder overrides statuses
    pub override_statuses: Option<bool>,
    /// Folder statuses
    pub statuses: Option<Vec<SpaceStatus>>,
}

/// Folder space information
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderSpace {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique space identifier
    pub id: String,
    /// Space display name
    pub name: String,
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
/// It can be organized in folders or directly in the space.
#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique list identifier
    pub id: String,
    /// List display name
    pub name: Option<String>,
    /// List display order
    #[serde(deserialize_with = "i64_from_string_or_number")]
    pub orderindex: i64,
    /// List content/description
    pub content: Option<String>,
    /// List status configuration
    pub status: Option<ListStatus>,
    /// List priority configuration
    pub priority: Option<ListPriority>,
    /// List assignee
    pub assignee: Option<ListAssignee>,
    /// Number of tasks in the list
    pub task_count: Option<i64>,
    /// List due date
    pub due_date: Option<String>,
    /// List start date
    pub start_date: Option<String>,
    /// List folder information
    pub folder: Option<ListFolder>,
    /// List space information
    pub space: ListSpace,
    /// Whether the list is archived
    pub archived: Option<bool>,
    /// Whether the list overrides statuses
    pub override_statuses: Option<bool>,
    /// List statuses
    pub statuses: Option<Vec<SpaceStatus>>,
}

/// List status configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ListStatus {
    /// Status display name
    pub status: String,
    /// Status color for UI display
    pub color: String,
    /// Whether to hide the status label
    pub hide_label: Option<bool>,
}

/// List priority configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ListPriority {
    /// Priority display name
    pub priority: String,
    /// Priority color for UI display
    pub color: String,
}

/// List assignee information
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAssignee {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's preferred color
    pub color: String,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
}

/// List folder information
#[derive(Debug, Serialize, Deserialize)]
pub struct ListFolder {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique folder identifier
    pub id: String,
    /// Folder display name
    pub name: String,
    /// Whether the folder is hidden
    pub hidden: Option<bool>,
    /// Whether the user has access to the folder
    pub access: Option<bool>,
}

/// List space information
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSpace {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique space identifier
    pub id: String,
    /// Space display name
    pub name: String,
}

// Task models

/// Response containing a list of tasks
/// 
/// This struct represents the API response when fetching tasks from a list.
#[derive(Debug, Serialize, Deserialize)]
pub struct TasksResponse {
    /// List of tasks
    pub tasks: Vec<Task>,
}

/// Task information
/// 
/// This struct represents a task in ClickUp with all its properties,
/// metadata, and relationships.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier
    pub id: String,
    /// Task display name
    pub name: Option<String>,
    /// Task custom identifier
    pub custom_id: Option<String>,
    /// Task text content
    pub text_content: String,
    /// Task description
    pub description: String,
    /// Task status
    pub status: TaskStatus,
    /// Task display order
    #[serde(deserialize_with = "string_or_number")]
    pub orderindex: String,
    /// Task creation timestamp
    pub date_created: String,
    /// Task last update timestamp
    pub date_updated: String,
    /// Task close timestamp
    pub date_closed: Option<String>,
    /// Task creator information
    pub creator: TaskCreator,
    /// Task assignees
    pub assignees: Vec<TaskAssignee>,
    /// Task watchers
    pub watchers: Vec<TaskWatcher>,
    /// Task checklists
    pub checklists: Vec<TaskChecklist>,
    /// Task tags
    pub tags: Vec<TaskTag>,
    /// Parent task ID (for subtasks)
    pub parent: Option<String>,
    /// Top-level parent task ID
    pub top_level_parent: Option<String>,
    /// Task priority
    pub priority: Option<TaskPriority>,
    /// Task due date
    pub due_date: Option<String>,
    /// Task start date
    pub start_date: Option<String>,
    /// Task time estimate (in milliseconds)
    pub time_estimate: Option<i64>,
    /// Task time spent (in milliseconds)
    pub time_spent: Option<i64>,
    /// Task custom fields
    pub custom_fields: Vec<TaskCustomField>,
    /// Task dependencies
    pub dependencies: Vec<String>,
    /// Task linked tasks
    pub linked_tasks: Vec<String>,
    /// Task team ID
    pub team_id: String,
    /// Task list information
    pub list: TaskList,
    /// Task folder information
    pub folder: Option<TaskFolder>,
    /// Task space information
    pub space: TaskSpace,
    /// Task URL
    pub url: String,
    /// Task subtasks
    pub subtasks: Option<Vec<Task>>,
}

/// Task status information
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Unique status identifier
    pub id: String,
    /// Status display name
    pub status: String,
    /// Status color for UI display
    pub color: String,
    /// Status display order
    #[serde(deserialize_with = "i64_from_string_or_number")]
    pub orderindex: i64,
    /// Status type
    #[serde(rename = "type")]
    pub type_: String,
}

/// Task creator information
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreator {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's preferred color
    pub color: String,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
}

/// Task assignee information
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignee {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's preferred color
    pub color: String,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
}

/// Task watcher information
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskWatcher {
    /// Unique user identifier
    pub id: i64,
    /// User's display name
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's preferred color
    pub color: String,
    /// URL to user's profile picture
    pub profile_picture: Option<String>,
    /// User's initials for avatar display
    pub initials: Option<String>,
}

/// Task checklist information
/// 
/// This struct represents a checklist within a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskChecklist {
    /// Unique checklist identifier
    pub id: String,
    /// Checklist display name
    pub name: Option<String>,
    /// Checklist display order
    #[serde(deserialize_with = "i64_from_string_or_number")]
    pub orderindex: i64,
    /// Checklist assignee
    pub assignee: Option<serde_json::Value>,
    /// Whether the checklist is resolved
    #[serde(deserialize_with = "opt_bool_from_bool_or_int")]
    pub resolved: Option<bool>,
    /// Parent checklist item (for nested items)
    pub parent: Option<serde_json::Value>,
    /// Checklist creation timestamp
    pub date_created: String,
    /// Child checklist items
    pub children: Option<Vec<TaskChecklistItem>>,
}

/// Task checklist item information
/// 
/// This struct represents an individual item within a task checklist.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskChecklistItem {
    /// Unique checklist item identifier
    pub id: String,
    /// Checklist item display name
    pub name: Option<String>,
    /// Checklist item display order
    #[serde(deserialize_with = "i64_from_string_or_number")]
    pub orderindex: i64,
    /// Checklist item assignee
    pub assignee: Option<serde_json::Value>,
    /// Whether the checklist item is resolved
    #[serde(deserialize_with = "opt_bool_from_bool_or_int")]
    pub resolved: Option<bool>,
    /// Parent checklist item (for nested items)
    pub parent: Option<serde_json::Value>,
    /// Checklist item creation timestamp
    pub date_created: String,
    /// Child checklist items
    pub children: Option<Vec<serde_json::Value>>,
}

/// Task tag information
/// 
/// This struct represents a tag assigned to a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskTag {
    /// Tag display name
    pub name: Option<String>,
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
    #[serde(deserialize_with = "string_or_number")]
    pub id: String,
    /// Priority display name
    pub priority: String,
    /// Priority color for UI display
    pub color: String,
    /// Priority display order
    #[serde(deserialize_with = "string_or_number")]
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
    pub name: Option<String>,
    /// Custom field type
    #[serde(rename = "type")]
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
    #[serde(deserialize_with = "string_or_number")]
    /// Unique list identifier
    pub id: String,
    /// List display name
    pub name: Option<String>,
    /// Whether the user has access to the list
    #[serde(deserialize_with = "opt_bool_from_bool_or_int")]
    pub access: Option<bool>,
}

/// Task folder information
/// 
/// This struct contains information about the folder containing the task's list.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFolder {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique folder identifier
    pub id: String,
    /// Folder display name
    pub name: Option<String>,
    /// Whether the folder is hidden
    #[serde(deserialize_with = "opt_bool_from_bool_or_int")]
    pub hidden: Option<bool>,
    /// Whether the user has access to the folder
    #[serde(deserialize_with = "opt_bool_from_bool_or_int")]
    pub access: Option<bool>,
}

/// Task space information
/// 
/// This struct contains information about the space containing the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpace {
    #[serde(deserialize_with = "string_or_number")]
    /// Unique space identifier
    pub id: String,
    /// Space display name
    pub name: Option<String>,
}

// Request models

/// Request data for creating a new task
/// 
/// This struct contains the data needed to create a new task in ClickUp.
/// All fields except `name` are optional.
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
    /// Task start date (Unix timestamp in milliseconds)
    pub start_date: Option<i64>,
    /// Whether the start date includes time
    pub start_date_time: Option<bool>,
    /// Task story points
    pub points: Option<i64>,
    /// Whether to notify assignees
    pub notify_all: Option<bool>,
}

impl Default for CreateTaskRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            status: None,
            priority: None,
            due_date: None,
            due_date_time: None,
            time_estimate: None,
            assignees: None,
            tags: None,
            parent: None,
            custom_fields: None,
            start_date: None,
            start_date_time: None,
            points: None,
            notify_all: None,
        }
    }
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
    /// Task start date (Unix timestamp in milliseconds)
    pub start_date: Option<i64>,
    /// Whether the start date includes time
    pub start_date_time: Option<bool>,
    /// Task story points
    pub points: Option<i64>,
    /// Whether to notify assignees
    pub notify_all: Option<bool>,
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
    #[serde(rename = "type")]
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
    /// Comment content
    pub comment_text: String,
    /// Comment assignee (if any)
    pub assignee: Option<i64>,
    /// Whether to notify assignee
    pub notify_all: Option<bool>,
} 