# ClickUp CLI - User Guide

A powerful command-line interface for managing your ClickUp workspaces, tasks, and projects directly from your terminal.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Authentication](#authentication)
4. [Basic Commands](#basic-commands)
5. [Task Management](#task-management)
6. [Advanced Features](#advanced-features)
7. [Configuration](#configuration)
8. [Troubleshooting](#troubleshooting)
9. [Examples](#examples)

## Installation

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **ClickUp API token** - Get from your ClickUp account settings

### Install the CLI

#### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Build and install
cargo install --path .

# Verify installation
clickup-cli --help
```

#### Option 2: Build Locally

```bash
# Clone and build
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils
cargo build --release

# The binary will be at target/release/clickup-cli
./target/release/clickup-cli --help
```

## Quick Start

### 1. Get Your ClickUp API Token

1. Log in to your ClickUp account
2. Go to **Settings** → **Apps**
3. Click **Generate API Token** (for personal use) or **Create New App** (for team use)
4. Copy the generated token

### 2. Set Up Authentication

```bash
# Set your API token (interactive mode)
clickup-cli auth set

# Or set it directly
clickup-cli auth set --token "your-api-token-here"

# Test your authentication
clickup-cli auth test
```

### 3. Explore Your Workspaces

```bash
# List all your workspaces
clickup-cli workspaces list

# Show details of a specific workspace
clickup-cli workspaces show --id "workspace_id_here"
```

### 4. Navigate Your Projects

```bash
# List spaces in a workspace
clickup-cli spaces list --workspace-id "workspace_id_here"

# List lists in a space
clickup-cli lists list --space-id "space_id_here"

# List tasks in a list
clickup-cli tasks list --list-id "list_id_here"
```

## Authentication

The CLI securely stores your API token and provides several authentication commands.

### Authentication Commands

```bash
# Set your API token
clickup-cli auth set

# Test your authentication
clickup-cli auth test

# Check authentication status
clickup-cli auth status

# Clear stored token
clickup-cli auth clear
```

### Rate Limiting Configuration

The CLI includes intelligent rate limiting to handle ClickUp's API limits:

```bash
# Show current rate limiting settings
clickup-cli auth rate-limit --show

# Configure for paid account with higher limits
clickup-cli auth rate-limit --requests-per-minute 500

# Disable auto-retry
clickup-cli auth rate-limit --auto-retry false

# Set custom retry behavior
clickup-cli auth rate-limit --max-retries 5 --buffer-seconds 10
```

**Default Settings:**
- 100 requests per minute (ClickUp free account limit)
- Auto-retry enabled with 3 max attempts
- 5-second buffer for safety

## Basic Commands

### Workspace Management

```bash
# List all workspaces
clickup-cli workspaces list

# Show workspace details
clickup-cli workspaces show --id "workspace_id"
```

### Team Management

```bash
# List all teams (same as workspaces)
clickup-cli teams list

# Show team details
clickup-cli teams show --id "team_id"
```

### Space Management

```bash
# List spaces in a workspace
clickup-cli spaces list --workspace-id "workspace_id"

# Show space details
clickup-cli spaces show --id "space_id"
```

### List Management

```bash
# List lists in a space
clickup-cli lists list --space-id "space_id"

# Show list details
clickup-cli lists show --id "list_id"
```

## Task Management

### Basic Task Operations

```bash
# List tasks in a list
clickup-cli tasks list --list-id "list_id"

# Show task details
clickup-cli tasks show --id "task_id"

# Create a new task
clickup-cli tasks create \
  --list-id "list_id" \
  --name "Task Name" \
  --description "Task description" \
  --status "to do" \
  --priority 1

# Update a task
clickup-cli tasks update \
  --id "task_id" \
  --name "Updated Task Name" \
  --status "in progress"

# Delete a task
clickup-cli tasks delete --id "task_id"
```

### Task Creation Options

When creating tasks, you can specify:

- `--name`: Task name (required)
- `--description`: Task description
- `--status`: Task status ("to do", "in progress", "done", etc.)
- `--priority`: Priority level (1-4, where 1 is highest)
- `--due-date`: Due date in milliseconds since epoch
- `--assignees`: Comma-separated list of user IDs
- `--tags`: Comma-separated list of tags

```bash
# Create a complex task
clickup-cli tasks create \
  --list-id "list_id" \
  --name "Complete project documentation" \
  --description "Write comprehensive documentation for the new feature" \
  --status "to do" \
  --priority 1 \
  --due-date 1640995200000 \
  --assignees "123,456" \
  --tags "urgent,frontend"
```

### Task Filtering and Search

```bash
# List tasks with a specific tag (client-side filtering)
clickup-cli tasks list-by-tag --list-id "list_id" --tag "urgent"

# Search for tasks with a specific tag across all lists in a space
clickup-cli tasks search-by-tag --tag "urgent" --space-id "space_id"

# Search across all spaces in a workspace
clickup-cli tasks search-by-tag --tag "urgent" --workspace-id "workspace_id"
```

### Overdue Task Management

```bash
# Update overdue tasks with a specific tag to today's date
clickup-cli tasks update-overdue-by-tag --tag "urgent" --dry-run

# Actually update the overdue tasks (remove --dry-run)
clickup-cli tasks update-overdue-by-tag --tag "urgent"
```

## Tag Management

Tags help organize tasks with visual labels and colors. You can list existing tags and create new ones.

### Tag Commands

```bash
# List all tags in a space
clickup-cli tags list --space-id <space-id>

# Create a new tag
clickup-cli tags create \
  --space-id <space-id> \
  --name "urgent" \
  --tag-fg "#ffffff" \
  --tag-bg "#ff0000"
```

### Tag Creation Options

When creating tags, you can specify:

- `--space-id`: Space ID where the tag will be created (required)
- `--name`: Tag name (required)
- `--tag-fg`: Foreground color in hex format (default: "#ffffff")
- `--tag-bg`: Background color in hex format (default: "#ff0000")

```bash
# Create a tag with custom colors
clickup-cli tags create \
  --space-id "space_id" \
  --name "bug" \
  --tag-fg "#000000" \
  --tag-bg "#ffff00"

# Create a tag with default colors
clickup-cli tags create \
  --space-id "space_id" \
  --name "feature"
```

## Comment Management

```bash
# List comments for a task
clickup-cli comments list --task-id "task_id"

# Show comment details
clickup-cli comments show --id "comment_id"

# Create a new comment
clickup-cli comments create \
  --task-id "task_id" \
  --text "This is a comment" \
  --notify-all true

# Update a comment
clickup-cli comments update \
  --id "comment_id" \
  --text "Updated comment text" \
  --notify-all true

# Delete a comment
clickup-cli comments delete --id "comment_id"
```

## Advanced Features

### Dry-Run Mode

Test destructive operations without actually performing them:

```bash
# Test updating overdue tasks without making changes
clickup-cli tasks update-overdue-by-tag --tag "urgent" --dry-run

# Test deleting a task
clickup-cli tasks delete --id "task_id" --dry-run
```

### Debug Mode

Enable detailed logging for troubleshooting:

```bash
# Run any command with debug output
clickup-cli --debug auth test
clickup-cli --debug tasks list --list-id "list_id"
```

### Interactive Prompts

The CLI provides interactive prompts for missing parameters:

```bash
# If you don't provide required parameters, the CLI will prompt for them
clickup-cli tasks create
# CLI will prompt for: list-id, name, description, etc.
```

## Configuration

### Configuration Sources

The CLI supports multiple configuration sources with this precedence:

1. **Environment variables** (highest priority)
2. **`.env` file** (loaded automatically if present)
3. **Configuration file** (`config.toml`)
4. **Default values** (lowest priority)

### Configuration File

The CLI stores configuration in your system's config directory:

- **Windows**: `%APPDATA%\clickup-cli\config.toml`
- **macOS**: `~/Library/Application Support/clickup-cli/config.toml`
- **Linux**: `~/.config/clickup-cli/config.toml`

Example configuration file:

```toml
# API token for authentication
api_token = "your-api-token-here"

# Default workspace ID (optional)
workspace_id = "workspace-id"

# Default list ID (optional)
default_list_id = "list-id"

# API base URL (defaults to ClickUp v2 API)
api_base_url = "https://api.clickup.com/api/v2"
```

### Environment Variables

You can set configuration via environment variables:

```bash
# Required: API token
export CLICKUP_API_TOKEN="your-api-token"

# Optional: Default workspace and list IDs
export CLICKUP_WORKSPACE_ID="workspace-id"
export CLICKUP_DEFAULT_LIST_ID="list-id"

# Optional: API configuration
export CLICKUP_API_TIMEOUT="30"
export CLICKUP_DEFAULT_PAGE_SIZE="100"
```

### .env Files

For local development, create a `.env` file in your project root:

```bash
# Copy template (if available)
cp .env.example .env

# Edit .env file
CLICKUP_API_TOKEN=your_api_token_here
CLICKUP_WORKSPACE_ID=your_workspace_id_here
CLICKUP_DEFAULT_LIST_ID=your_list_id_here
```

**Note**: `.env` files are ignored by git for security. Never commit your `.env` file.

## Troubleshooting

### Common Issues

#### Authentication Problems

```bash
# Test your authentication
clickup-cli auth test

# Check authentication status
clickup-cli auth status

# Clear and reset your token
clickup-cli auth clear
clickup-cli auth set
```

#### Rate Limiting Issues

If you're hitting rate limits:

```bash
# Check current rate limiting settings
clickup-cli auth rate-limit --show

# Adjust for your account type
clickup-cli auth rate-limit --requests-per-minute 200  # For paid accounts

# Disable auto-retry if needed
clickup-cli auth rate-limit --auto-retry false
```

#### Debug Mode

Enable debug logging to see what's happening:

```bash
# Run with debug output
clickup-cli --debug auth test
clickup-cli --debug tasks list --list-id "list_id"
```

### Error Messages

The CLI provides detailed error messages to help you troubleshoot:

- **Authentication errors**: Check your API token
- **Rate limiting errors**: Adjust rate limiting settings
- **Not found errors**: Verify IDs are correct
- **Permission errors**: Check your ClickUp permissions

## Examples

### Complete Workflow Example

```bash
# 1. Set up authentication
clickup-cli auth set

# 2. List your workspaces
clickup-cli workspaces list

# 3. List spaces in a workspace
clickup-cli spaces list --workspace-id "workspace_123"

# 4. List lists in a space
clickup-cli lists list --space-id "space_456"

# 5. Create a task
clickup-cli tasks create \
  --list-id "list_789" \
  --name "Complete project documentation" \
  --description "Write comprehensive documentation for the new feature" \
  --status "to do" \
  --priority 1

# 6. List tasks to verify creation
clickup-cli tasks list --list-id "list_789"

# 7. Add a comment to the task
clickup-cli comments create \
  --task-id "task_abc123" \
  --text "Starting work on this task"
```

### Task Management Workflow

```bash
# Create a task with all options
clickup-cli tasks create \
  --list-id "list_789" \
  --name "Complex Task" \
  --description "This is a detailed task description" \
  --status "in progress" \
  --priority 2 \
  --due-date 1640995200000 \
  --assignees "123,456" \
  --tags "urgent,frontend"

# Update task status
clickup-cli tasks update \
  --id "task_abc123" \
  --status "done"

# List tasks with specific tag
clickup-cli tasks list-by-tag \
  --list-id "list_789" \
  --tag "urgent"

# Search for tasks with specific tag across all lists
clickup-cli tasks search-by-tag \
  --tag "urgent"

# Update overdue tasks with a specific tag
clickup-cli tasks update-overdue-by-tag \
  --tag "urgent" \
  --dry-run  # Test first, then remove --dry-run to actually update
```

### Batch Operations

```bash
# Find all tasks with "urgent" tag across workspace
clickup-cli tasks search-by-tag --tag "urgent" --workspace-id "workspace_123"

# Update all overdue urgent tasks
clickup-cli tasks update-overdue-by-tag --tag "urgent" --workspace-id "workspace_123"

# List all comments for a task
clickup-cli comments list --task-id "task_abc123"
```

### Configuration Examples

```bash
# Set default workspace for all commands
export CLICKUP_WORKSPACE_ID="workspace_123"

# Set default list for task operations
export CLICKUP_DEFAULT_LIST_ID="list_789"

# Configure for paid account with higher rate limits
clickup-cli auth rate-limit --requests-per-minute 500

# Set custom retry behavior
clickup-cli auth rate-limit --max-retries 5 --buffer-seconds 10
```

## Getting Help

```bash
# General help
clickup-cli --help

# Command-specific help
clickup-cli auth --help
clickup-cli tasks --help
clickup-cli tasks create --help

# List all available commands
clickup-cli --help
```

## Support

- **Issues**: [GitHub Issues](https://github.com/davidshq/clickup-utils/issues)
- **Documentation**: This user guide and the main [README.md](README.md)
- **Development Status**: See [dev-docs/PROJECT_STATUS.md](dev-docs/PROJECT_STATUS.md) for current development status

---

*This user guide covers all the features available in the ClickUp CLI. For development information, see the main [README.md](README.md) and [dev-docs/](dev-docs/) directory.* 