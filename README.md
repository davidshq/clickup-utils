# ClickUp CLI

A powerful command-line interface for ClickUp API that allows you to manage your ClickUp workspaces, spaces, lists, tasks, and comments directly from the terminal.

## Features

- **Authentication Management**: Set up and manage your ClickUp API tokens
- **Workspace Management**: List and view workspace details
- **Space Management**: Browse spaces within workspaces
- **List Management**: View lists within spaces
- **Task Management**: Create, read, update, and delete tasks
- **Comment Management**: Add and view comments on tasks
- **Beautiful Output**: Formatted tables and colored output
- **Error Handling**: Comprehensive error messages and logging
- **Configuration**: Persistent configuration storage

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- ClickUp API token (get from your ClickUp settings)

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd clickup-cli

# Build the application
cargo build --release

# The binary will be available at target/release/clickup-cli
```

### Install Locally

```bash
# Install to your cargo bin directory
cargo install --path .

# Now you can use `clickup-cli` from anywhere
```

## Quick Start

1. **Set up your API token**:
   ```bash
   clickup-cli auth set
   # Enter your ClickUp API token when prompted
   ```

2. **Test your authentication**:
   ```bash
   clickup-cli auth test
   ```

3. **List your workspaces**:
   ```bash
   clickup-cli workspaces list
   ```

4. **Start managing your tasks**:
   ```bash
   # List spaces in a workspace
   clickup-cli spaces list --workspace-id <workspace-id>
   
   # List lists in a space
   clickup-cli lists list --space-id <space-id>
   
   # List tasks in a list
   clickup-cli tasks list --list-id <list-id>
   ```

## Usage

### Authentication Commands

```bash
# Set your API token
clickup-cli auth set --token <your-api-token>
clickup-cli auth set  # Interactive mode

# Test your authentication
clickup-cli auth test

# Check authentication status
clickup-cli auth status

# Clear stored token
clickup-cli auth clear
```

### Workspace Commands

```bash
# List all workspaces
clickup-cli workspaces list

# Show details of a specific workspace
clickup-cli workspaces show --id <workspace-id>
```

### Team Commands

```bash
# List all teams (same as workspaces)
clickup-cli teams list

# Show details of a specific team
clickup-cli teams show --id <team-id>
```

### Space Commands

```bash
# List all spaces in a workspace
clickup-cli spaces list --workspace-id <workspace-id>

# Show details of a specific space
clickup-cli spaces show --id <space-id>
```

### List Commands

```bash
# List all lists in a space
clickup-cli lists list --space-id <space-id>

# Show details of a specific list
clickup-cli lists show --id <list-id>
```

### Task Commands

```bash
# List all tasks in a list
clickup-cli tasks list --list-id <list-id>

# Show details of a specific task
clickup-cli tasks show --id <task-id>

# Create a new task
clickup-cli tasks create \
  --list-id <list-id> \
  --name "My Task" \
  --description "Task description" \
  --status "to do" \
  --priority 2 \
  --due-date 1640995200000

# Update an existing task
clickup-cli tasks update \
  --id <task-id> \
  --name "Updated Task Name" \
  --status "in progress"

# Delete a task
clickup-cli tasks delete --id <task-id>
```

### Comment Commands

```bash
# List all comments for a task
clickup-cli comments list --task-id <task-id>

# Show details of a specific comment
clickup-cli comments show --id <comment-id>

# Create a new comment
clickup-cli comments create \
  --task-id <task-id> \
  --text "This is a comment" \
  --notify-all true
```

## Configuration

The CLI stores configuration in your system's config directory:

- **Windows**: `%APPDATA%\clickup-cli\config.toml`
- **macOS**: `~/Library/Application Support/clickup-cli/config.toml`
- **Linux**: `~/.config/clickup-cli/config.toml`

### Environment Variables

You can also set configuration via environment variables:

```bash
export CLICKUP_API_TOKEN="your-api-token"
export CLICKUP_API_BASE_URL="https://api.clickup.com/api/v2"
```

## Getting Your ClickUp API Token

1. Log in to your ClickUp account
2. Go to **Settings** → **Apps**
3. Click **Create New App** or use your personal token
4. Copy the API token

## Examples

### Workflow Examples

**Create a task from command line**:
```bash
# Set up authentication
clickup-cli auth set

# Find your workspace and list IDs
clickup-cli workspaces list
clickup-cli spaces list --workspace-id <workspace-id>
clickup-cli lists list --space-id <space-id>

# Create a task
clickup-cli tasks create \
  --list-id <list-id> \
  --name "Review pull request #123" \
  --description "Please review the changes in PR #123" \
  --priority 3 \
  --status "to do"
```

**Update task status**:
```bash
clickup-cli tasks update --id <task-id> --status "in progress"
```

**Add a comment to a task**:
```bash
clickup-cli comments create \
  --task-id <task-id> \
  --text "Working on this now" \
  --notify-all true
```

## Error Handling

The CLI provides detailed error messages for common issues:

- **Authentication errors**: Invalid or missing API token
- **Permission errors**: Insufficient permissions for the requested action
- **Rate limiting**: When you exceed API rate limits
- **Network errors**: Connection issues
- **Validation errors**: Invalid input parameters

## Development

### Building for Development

```bash
# Build in debug mode
cargo build

# Run with debug logging
cargo run -- --debug workspaces list
```

### Running Tests

```bash
cargo test
```

### Adding New Commands

The CLI is designed to be easily extensible. To add new commands:

1. Create a new module in `src/commands/`
2. Define your command structure using `clap::Subcommand`
3. Implement the `execute` function
4. Add the module to `src/commands/mod.rs`
5. Add the command to the main CLI structure in `src/main.rs`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

For issues and questions:

1. Check the error messages for guidance
2. Use `--debug` flag for detailed logging
3. Verify your API token is correct
4. Check your ClickUp permissions

## Roadmap

- [ ] Webhook support for real-time updates
- [ ] Batch operations for multiple tasks
- [ ] Export/import functionality
- [ ] Custom field support
- [ ] Time tracking integration
- [ ] Advanced filtering and search
- [ ] Shell completion scripts
- [ ] Plugin system for custom commands 