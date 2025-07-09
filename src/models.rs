use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// User models
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub color: Option<String>,
    pub profile_picture: Option<String>,
    pub initials: Option<String>,
    pub role: Option<i64>,
    pub custom_role: Option<serde_json::Value>,
    pub last_active: Option<String>,
    pub date_joined: Option<String>,
    pub date_invited: Option<String>,
}

// Workspace models
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacesResponse {
    pub teams: Vec<Workspace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub avatar: Option<String>,
    pub members: Vec<WorkspaceMember>,
    pub roles: Vec<WorkspaceRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub color: Option<String>,
    pub initials: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceRole {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub color: Option<String>,
    pub orderindex: i64,
}

// Space models
#[derive(Debug, Serialize, Deserialize)]
pub struct SpacesResponse {
    pub spaces: Vec<Space>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub private: bool,
    pub statuses: Vec<SpaceStatus>,
    pub multiple_assignees: bool,
    pub features: SpaceFeatures,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceStatus {
    pub id: String,
    pub status: String,
    pub type_: String,
    pub orderindex: i64,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceFeatures {
    pub due_dates: SpaceFeature,
    pub sprints: SpaceFeature,
    pub time_tracking: SpaceFeature,
    pub points: SpaceFeature,
    pub custom_items: SpaceFeature,
    pub priorities: SpaceFeature,
    pub tags: SpaceFeature,
    pub time_estimates: SpaceFeature,
    pub check_unresolved: SpaceFeature,
    pub zoom: SpaceFeature,
    pub milestones: SpaceFeature,
    pub custom_fields: SpaceFeature,
    pub dependency_warning: SpaceFeature,
    pub multiple_assignees: SpaceFeature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceFeature {
    pub enabled: bool,
}

// List models
#[derive(Debug, Serialize, Deserialize)]
pub struct ListsResponse {
    pub lists: Vec<List>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub orderindex: i64,
    pub content: String,
    pub status: Option<serde_json::Value>,
    pub priority: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    pub task_count: Option<String>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub folder: Option<Folder>,
    pub space: Option<Space>,
    pub inbound_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub access: bool,
}

// Task models
#[derive(Debug, Serialize, Deserialize)]
pub struct TasksResponse {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub text_content: Option<String>,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub orderindex: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_closed: Option<String>,
    pub creator: TaskCreator,
    pub assignees: Vec<TaskAssignee>,
    pub checklists: Vec<TaskChecklist>,
    pub tags: Vec<TaskTag>,
    pub parent: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub points: Option<i64>,
    pub time_estimate: Option<i64>,
    pub time_spent: Option<i64>,
    pub custom_fields: Vec<TaskCustomField>,
    pub list: TaskList,
    pub folder: Option<TaskFolder>,
    pub space: TaskSpace,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStatus {
    pub status: String,
    pub color: String,
    pub orderindex: i64,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreator {
    pub id: i64,
    pub username: String,
    pub color: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignee {
    pub id: i64,
    pub username: String,
    pub color: Option<String>,
    pub initials: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskChecklist {
    pub id: String,
    pub task_id: String,
    pub name: String,
    pub date_created: String,
    pub orderindex: i64,
    pub creator: i64,
    pub resolved: i64,
    pub unresolved: i64,
    pub items: Vec<TaskChecklistItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskChecklistItem {
    pub id: String,
    pub name: String,
    pub orderindex: i64,
    pub assignee: Option<serde_json::Value>,
    pub resolved: bool,
    pub parent: Option<serde_json::Value>,
    pub date_created: String,
    pub children: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskTag {
    pub name: String,
    pub tag_fg: String,
    pub tag_bg: String,
    pub creator: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskPriority {
    pub id: String,
    pub priority: String,
    pub color: String,
    pub orderindex: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCustomField {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub type_config: Option<serde_json::Value>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub id: String,
    pub name: String,
    pub access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFolder {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpace {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub private: bool,
    pub avatar: Option<String>,
    pub admin_can_manage: Option<bool>,
    pub statuses: Vec<SpaceStatus>,
    pub multiple_assignees: bool,
}

// Task request models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i64>,
    pub due_date: Option<i64>,
    pub due_date_time: Option<bool>,
    pub time_estimate: Option<i64>,
    pub assignees: Option<Vec<i64>>,
    pub tags: Option<Vec<String>>,
    pub parent: Option<String>,
    pub custom_fields: Option<Vec<CustomFieldValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i64>,
    pub due_date: Option<i64>,
    pub due_date_time: Option<bool>,
    pub time_estimate: Option<i64>,
    pub assignees: Option<Vec<i64>>,
    pub tags: Option<Vec<String>>,
    pub parent: Option<String>,
    pub custom_fields: Option<Vec<CustomFieldValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFieldValue {
    pub id: String,
    pub value: serde_json::Value,
}

// Comment models
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub comment: Vec<CommentText>,
    pub comment_text: String,
    pub user: CommentUser,
    pub resolved: bool,
    pub assignee: Option<serde_json::Value>,
    pub assignee_by: Option<serde_json::Value>,
    pub reaction: Option<serde_json::Value>,
    pub date_created: String,
    pub date_updated: String,
    pub parent: Option<serde_json::Value>,
    pub children: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentText {
    pub text: String,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentUser {
    pub id: i64,
    pub username: String,
    pub color: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub comment_text: String,
    pub assignee: Option<i64>,
    pub notify_all: Option<bool>,
} 