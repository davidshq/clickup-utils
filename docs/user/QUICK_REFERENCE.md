# ClickUp CLI - Quick Reference

A quick reference guide for all available commands and their usage.

## Installation

```bash
# Build and install
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils
cargo install --path .

# Verify installation
clickup-cli --help
```

## Authentication

```bash
# Set API token
clickup-cli auth set

# Test authentication
clickup-cli auth test

# Check status
clickup-cli auth status

# Clear token
clickup-cli auth clear

# Rate limiting configuration
clickup-cli auth rate-limit --show
clickup-cli auth rate-limit --requests-per-minute 200
```

## Workspace & Team Management

```bash
# List workspaces
clickup-cli workspaces list

# Show workspace details
clickup-cli workspaces show --id "workspace_id"

# List teams (same as workspaces)
clickup-cli teams list

# Show team details
clickup-cli teams show --id "team_id"
```

## Space Management

```bash
# List spaces in workspace
clickup-cli spaces list --workspace-id "workspace_id"

# Show space details
clickup-cli spaces show --id "space_id"
```

## List Management

```bash
# List lists in space
clickup-cli lists list --space-id "space_id"

# Show list details
clickup-cli lists show --id "list_id"
```

## Task Management

### Basic Operations

```bash
# List tasks
clickup-cli tasks list --list-id "list_id"

# Show task details
clickup-cli tasks show --id "task_id"

# Create task
clickup-cli tasks create \
  --list-id "list_id" \
  --name "Task Name" \
  --description "Description" \
  --status "to do" \
  --priority 1

# Update task
clickup-cli tasks update \
  --id "task_id" \
  --name "New Name" \
  --status "in progress"

# Delete task
clickup-cli tasks delete --id "task_id"
```

### Advanced Task Features

```bash
# Create complex task
clickup-cli tasks create \
  --list-id "list_id" \
  --name "Task Name" \
  --description "Description" \
  --status "to do" \
  --priority 1 \
  --due-date 1640995200000 \
  --assignees "123,456" \
  --tags "urgent,frontend"

# Filter tasks by tag
clickup-cli tasks list-by-tag --list-id "list_id" --tag "urgent"

# Search tasks by tag across space
clickup-cli tasks search-by-tag --tag "urgent" --space-id "space_id"

# Search tasks by tag across workspace
clickup-cli tasks search-by-tag --tag "urgent" --workspace-id "workspace_id"

# Update overdue tasks with tag
clickup-cli tasks update-overdue-by-tag --tag "urgent" --dry-run
```

## Comment Management

```bash
# List comments
clickup-cli comments list --task-id "task_id"

# Show comment details
clickup-cli comments show --id "comment_id"

# Create comment
clickup-cli comments create \
  --task-id "task_id" \
  --text "Comment text" \
  --notify-all true

# Update comment
clickup-cli comments update \
  --id "comment_id" \
  --text "Updated text" \
  --notify-all true

# Delete comment
clickup-cli comments delete --id "comment_id"
```

## Advanced Features

### Debug Mode

```bash
# Enable debug logging
clickup-cli --debug <command>

# Examples
clickup-cli --debug auth test
clickup-cli --debug tasks list --list-id "list_id"
```

### Dry-Run Mode

```bash
# Test operations without making changes
clickup-cli tasks update-overdue-by-tag --tag "urgent" --dry-run
clickup-cli tasks delete --id "task_id" --dry-run
```

### Interactive Prompts

```bash
# CLI will prompt for missing parameters
clickup-cli tasks create
clickup-cli comments create
```

## Configuration

### Environment Variables

```bash
# Required
export CLICKUP_API_TOKEN="your-api-token"

# Optional
export CLICKUP_WORKSPACE_ID="workspace-id"
export CLICKUP_DEFAULT_LIST_ID="list-id"
export CLICKUP_API_TIMEOUT="30"
export CLICKUP_DEFAULT_PAGE_SIZE="100"
```

### Configuration File

The CLI stores configuration in:
- **Windows**: `%APPDATA%\clickup-cli\config.toml`
- **macOS**: `~/Library/Application Support/clickup-cli/config.toml`
- **Linux**: `~/.config/clickup-cli/config.toml`

Example `config.toml`:
```toml
api_token = "your-api-token-here"
workspace_id = "workspace-id"
default_list_id = "list-id"
api_base_url = "https://api.clickup.com/api/v2"
```

### .env File

Create `.env` in project root:
```bash
CLICKUP_API_TOKEN=your_api_token_here
CLICKUP_WORKSPACE_ID=your_workspace_id_here
CLICKUP_DEFAULT_LIST_ID=your_list_id_here
```

## Common Workflows

### Complete Setup Workflow

```bash
# 1. Set authentication
clickup-cli auth set

# 2. List workspaces
clickup-cli workspaces list

# 3. List spaces
clickup-cli spaces list --workspace-id "workspace_id"

# 4. List lists
clickup-cli lists list --space-id "space_id"

# 5. Create task
clickup-cli tasks create \
  --list-id "list_id" \
  --name "My Task" \
  --description "Task description" \
  --status "to do" \
  --priority 1

# 6. Add comment
clickup-cli comments create \
  --task-id "task_id" \
  --text "Starting work"
```

### Task Management Workflow

```bash
# Create task with all options
clickup-cli tasks create \
  --list-id "list_id" \
  --name "Complex Task" \
  --description "Description" \
  --status "in progress" \
  --priority 2 \
  --due-date 1640995200000 \
  --assignees "123,456" \
  --tags "urgent,frontend"

# Update status
clickup-cli tasks update --id "task_id" --status "done"

# Filter by tag
clickup-cli tasks list-by-tag --list-id "list_id" --tag "urgent"

# Search across space
clickup-cli tasks search-by-tag --tag "urgent" --space-id "space_id"

# Update overdue tasks
clickup-cli tasks update-overdue-by-tag --tag "urgent" --dry-run
```

### Batch Operations

```bash
# Find all urgent tasks in workspace
clickup-cli tasks search-by-tag --tag "urgent" --workspace-id "workspace_id"

# Update all overdue urgent tasks
clickup-cli tasks update-overdue-by-tag --tag "urgent" --workspace-id "workspace_id"

# List all comments for task
clickup-cli comments list --task-id "task_id"
```

## Troubleshooting

### Common Issues

```bash
# Authentication problems
clickup-cli auth test
clickup-cli auth status
clickup-cli auth clear
clickup-cli auth set

# Rate limiting issues
clickup-cli auth rate-limit --show
clickup-cli auth rate-limit --requests-per-minute 200

# Debug mode
clickup-cli --debug auth test
clickup-cli --debug tasks list --list-id "list_id"
```

### Error Types

- **Authentication errors**: Check API token
- **Rate limiting errors**: Adjust rate limiting settings
- **Not found errors**: Verify IDs are correct
- **Permission errors**: Check ClickUp permissions

## Help Commands

```bash
# General help
clickup-cli --help

# Command-specific help
clickup-cli auth --help
clickup-cli tasks --help
clickup-cli tasks create --help
clickup-cli comments --help
```

## Configuration Precedence

1. **Environment variables** (highest priority)
2. **`.env` file** (loaded automatically if present)
3. **Configuration file** (`config.toml`)
4. **Default values** (lowest priority)

## Rate Limiting Defaults

- **Requests per minute**: 100 (ClickUp free account limit)
- **Auto-retry**: Enabled with 3 max attempts
- **Buffer seconds**: 5 seconds for safety

## Task Status Options

- `to do`
- `in progress`
- `done`
- `closed`
- Custom statuses from your ClickUp workspace

## Priority Levels

- `1`: Highest priority
- `2`: High priority
- `3`: Normal priority
- `4`: Low priority

## Date Format

Due dates should be provided in milliseconds since epoch (Unix timestamp × 1000).

Example: `1640995200000` for January 1, 2022

---

*For detailed documentation, see [USER_GUIDE.md](USER_GUIDE.md) and [README.md](README.md).* 