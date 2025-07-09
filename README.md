# ClickUp CLI

[![Crates.io](https://img.shields.io/crates/v/clickup-cli)](https://crates.io/crates/clickup-cli)
[![Documentation](https://docs.rs/clickup-cli/badge.svg)](https://docs.rs/clickup-cli)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

A powerful command-line interface for the ClickUp API that allows you to manage your ClickUp workspaces, spaces, lists, tasks, and comments directly from the terminal.

## WARNING: Vibe Coded
This has been vibe coded using primarily Cursor. Expect the code quality to be low.

## 🚀 Features

- **🔐 Authentication Management**: Secure API token storage and management
- **🏢 Workspace Management**: List and view workspace details
- **📁 Space Management**: Browse spaces within workspaces
- **📋 List Management**: View lists within spaces
- **✅ Task Management**: Create, read, update, and delete tasks
- **💬 Comment Management**: Add and view comments on tasks
- **🎨 Beautiful Output**: Formatted tables and colored output
- **🛡️ Error Handling**: Comprehensive error messages and logging
- **⚙️ Configuration**: Persistent configuration storage
- **🔍 Debug Mode**: Detailed logging for troubleshooting

## 📦 Installation

### Prerequisites

- **Rust 1.70+** (install from [rustup.rs](https://rustup.rs/))
- **ClickUp API token** (get from your ClickUp settings)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/davidshq/clickup-cli.git
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

### Install from Crates.io

```bash
# Install the latest version from crates.io
cargo install clickup-cli
```

## 🚀 Quick Start

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

## 📖 Usage

### Authentication Commands

```bash
# Set your API token (interactive mode - recommended)
clickup-cli auth set

# Set your API token (command-line mode)
clickup-cli auth set --token <your-api-token>

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

# List tasks filtered by tag (client-side filtering)
clickup-cli tasks list-by-tag --list-id <list-id> --tag <tag-name>

# Search for tasks with a specific tag across all lists in a space
clickup-cli tasks search-by-tag --tag <tag-name> [--workspace-id <workspace-id>] [--space-id <space-id>]

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

# Update an existing comment
clickup-cli comments update \
  --id <comment-id> \
  --text "Updated comment text" \
  --notify-all true

# Delete a comment
clickup-cli comments delete --id <comment-id>
```

## ⚙️ Configuration

The CLI stores configuration in your system's config directory:

- **Windows**: `%APPDATA%\clickup-cli\config.toml`
- **macOS**: `~/Library/Application Support/clickup-cli/config.toml`
- **Linux**: `~/.config/clickup-cli/config.toml`

### Configuration File Format

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

You can also set configuration via environment variables:

```bash
export CLICKUP_API_TOKEN="your-api-token"
export CLICKUP_WORKSPACE_ID="workspace-id"
export CLICKUP_DEFAULT_LIST_ID="list-id"
export CLICKUP_API_BASE_URL="https://api.clickup.com/api/v2"
```

## 🔑 Getting Your ClickUp API Token

1. **Log in to your ClickUp account**
2. **Go to Settings** → **Apps**
3. **Click Create New App** or use your personal token
4. **Copy the API token** and use it with the CLI

### Personal Token (Recommended)

1. Go to **Settings** → **Apps**
2. Click **Generate API Token**
3. Copy the generated token

### App Token

1. Go to **Settings** → **Apps**
2. Click **Create New App**
3. Configure your app settings
4. Copy the API token from your app

## 🐛 Debug Mode

Enable debug logging for troubleshooting:

```bash
# Enable debug mode
clickup-cli --debug <command>

# Example with debug output
clickup-cli --debug auth test
```

## 📝 Examples

### Complete Workflow Example

```bash
# 1. Set up authentication
clickup-cli auth set

# 2. List workspaces
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

### Advanced Task Management

```bash
# Create a task with all options
clickup-cli tasks create \
  --list-id "list_789" \
  --name "Complex Task" \
  --description "This is a detailed task description" \
  --status "in progress" \
  --priority 2 \
  --due-date 1640995200000 \
  --assignees 123,456 \
  --tags "urgent", "frontend"

# Update task status
clickup-cli tasks update \
  --id "task_abc123" \
  --status "done"

# List tasks with specific tag
clickup-cli tasks list-by-tag \
  --list-id "list_789" \
  --tag "update-auto"

# Search for tasks with specific tag across all lists in a space
clickup-cli tasks search-by-tag \
  --tag "update-auto"

# Delete completed tasks
clickup-cli tasks delete --id "task_abc123"
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/clickup-cli.git
cd clickup-cli

# Install dependencies
cargo build

# Run tests
cargo test

# Run with debug logging
cargo run -- --debug auth test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ClickUp API](https://clickup.com/api) for providing the API
- [clap](https://github.com/clap-rs/clap) for excellent CLI argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) for HTTP client functionality
- [serde](https://github.com/serde-rs/serde) for serialization support

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/davidshq/clickup-cli/issues)
- **Documentation**: [API Documentation](https://docs.rs/clickup-cli)

## 🔄 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.