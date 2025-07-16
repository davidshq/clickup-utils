# ClickUp CLI

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-200%2B%20passing-brightgreen.svg)](https://github.com/davidshq/clickup-utils)
[![Clippy](https://img.shields.io/badge/clippy-clean-brightgreen.svg)](https://github.com/davidshq/clickup-utils)

A powerful command-line interface for the ClickUp API that allows you to manage your ClickUp workspaces, spaces, lists, tasks, and comments directly from the terminal.

## 🏗️ Architecture

This project follows modern Rust best practices with a clean separation between library and binary concerns:

- **Library/Binary Separation**: Clean separation between reusable library code and CLI-specific logic
- **Configuration-Driven Design**: All constants centralized and environment-overridable
- **Application Layer**: Proper lifecycle management with startup/shutdown procedures
- **Type Safety**: Comprehensive error handling with custom error types
- **Async/Await**: Modern async patterns throughout the codebase

### Recent Architectural Improvements
- **✅ Library/Binary Separation**: Eliminated module duplication and created clean architecture
- **✅ Centralized Constants**: All magic values moved to `src/constants.rs`
- **✅ Application Layer**: Created `src/app.rs` for proper lifecycle management
- **✅ Configuration-Driven**: Environment variables can override all settings
- **✅ Enhanced Error Handling**: Comprehensive error context and logging

## 📊 Project Status

For current project status, quality metrics, and implementation details, see:
**[dev-docs/PROJECT_STATUS.md](dev-docs/PROJECT_STATUS.md)**

**Quick Overview:**
- **✅ Zero Clippy Warnings**: All code quality issues resolved
- **✅ All Documentation Tests Passing**: 22/22 doc tests successful
- **✅ Comprehensive Test Coverage**: 200+ unit tests, 9 integration tests

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
- **🏷️ Tag-based Operations**: Search and filter tasks by tags
- **📅 Overdue Task Management**: Update overdue tasks with specific tags
- **⏱️ Rate Limiting**: Intelligent API rate limit handling with automatic retry
- **🔄 Advanced Search**: Cross-space task search and filtering
- **📊 Interactive Prompts**: User-friendly prompts for missing parameters
- **🛡️ Dry-run Support**: Safe testing of destructive operations
- **🧪 Integration Testing**: Comprehensive test suite with real API testing

## 📦 Installation

### Prerequisites

- **Rust 1.70+** (install from [rustup.rs](https://rustup.rs/))
- **ClickUp API token** (get from your ClickUp settings)

### Library Usage

The ClickUp CLI can also be used as a library in other Rust projects:

```rust
use clickup_cli::{ClickUpApi, Config, ClickUpError};
use clickup_cli::constants::api::DEFAULT_TIMEOUT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let api = ClickUpApi::new(config)?;
    
    // Use the API client
    let workspaces = api.get_workspaces().await?;
    println!("Found {} workspaces", workspaces.teams.len());
    
    Ok(())
}
```

### Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

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

### Install from Crates.io (Coming Soon)

```bash
# Install the latest version from crates.io (when published)
cargo install clickup-cli
```

**Note**: This package is not yet published to crates.io. Use the local installation method above.

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

# Configure rate limiting settings
clickup-cli auth rate-limit --show

# Set custom rate limit (e.g., for paid accounts with higher limits)
clickup-cli auth rate-limit --requests-per-minute 200

# Disable auto-retry for rate-limited requests
clickup-cli auth rate-limit --auto-retry false

# Set custom retry settings
clickup-cli auth rate-limit --max-retries 5 --buffer-seconds 10
```

### Rate Limiting

The CLI includes intelligent rate limiting to handle ClickUp's API limits:

- **Automatic Throttling**: Prevents hitting rate limits by tracking requests per minute
- **Smart Retry Logic**: Automatically retries rate-limited requests with exponential backoff
- **Configurable Limits**: Adjust settings for different account types (free vs paid)
- **Request Queuing**: Ensures no requests are lost when rate limits are hit

**Default Settings**:
- 100 requests per minute (ClickUp free account limit)
- Auto-retry enabled with 3 max attempts
- 5-second buffer for safety

**Configuration**:
```bash
# Show current rate limiting settings
clickup-cli auth rate-limit --show

# Configure for paid account with higher limits
clickup-cli auth rate-limit --requests-per-minute 500

# Disable auto-retry (manual handling)
clickup-cli auth rate-limit --auto-retry false

# Set custom retry behavior
clickup-cli auth rate-limit --max-retries 5 --buffer-seconds 10
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

# Update overdue tasks with a specific tag to today's date
clickup-cli tasks update-overdue-by-tag --tag <tag-name> [--workspace-id <workspace-id>] [--space-id <space-id>] [--dry-run]

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

The CLI supports multiple configuration sources with the following precedence (highest to lowest):

1. **Environment variables** (highest priority)
2. **`.env` file** (loaded automatically if present)
3. **Configuration file** (`config.toml`)
4. **Default values** (lowest priority)

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
# Required: API token for authentication
export CLICKUP_API_TOKEN="your-api-token"

# Optional: Default workspace and list IDs
export CLICKUP_WORKSPACE_ID="workspace-id"
export CLICKUP_DEFAULT_LIST_ID="list-id"

# Optional: API configuration
export CLICKUP_API_BASE_URL="https://api.clickup.com/api/v2"
export CLICKUP_API_TIMEOUT="30"  # API request timeout in seconds
export CLICKUP_MAX_REQUEST_SIZE="10485760"  # Maximum request size in bytes (10MB)

# Optional: Pagination settings
export CLICKUP_DEFAULT_PAGE_SIZE="100"
export CLICKUP_MAX_PAGE_SIZE="1000"

# Optional: Connection pooling
export CLICKUP_MAX_IDLE_CONNECTIONS="10"
export CLICKUP_IDLE_TIMEOUT="90"  # Idle connection timeout in seconds
export CLICKUP_MAX_CONNECTIONS="100"

# Optional: Batch operation settings
export CLICKUP_MAX_BATCH_SIZE="10"
export CLICKUP_TASK_BATCH_SIZE="5"
export CLICKUP_COMMENT_BATCH_SIZE="10"
```

### .env Files (Recommended for Development)

For local development, you can use a `.env` file in your project root:

1. **Copy the template**:
   ```bash
   cp .env.example .env
   ```

2. **Edit the `.env` file** with your settings:
   ```bash
   # Required: Your ClickUp API token
   CLICKUP_API_TOKEN=your_api_token_here
   
   # Optional: Test API token for running tests
   CLICKUP_API_TOKEN_TEST=your_test_api_token_here
   
   # Optional: Default workspace ID
   CLICKUP_WORKSPACE_ID=your_workspace_id_here
   
   # Optional: Default list ID
   CLICKUP_DEFAULT_LIST_ID=your_list_id_here
   
   # Optional: Rate limiting settings
   CLICKUP_RATE_LIMIT__REQUESTS_PER_MINUTE=100
   CLICKUP_RATE_LIMIT__AUTO_RETRY=true
   CLICKUP_RATE_LIMIT__MAX_RETRIES=3
   CLICKUP_RATE_LIMIT__BUFFER_SECONDS=5
   ```

3. **The `.env` file will be loaded automatically** when you run the CLI

**Note**: `.env` files are ignored by git for security. Never commit your `.env` file.

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

# Update overdue tasks with a specific tag to today's date
clickup-cli tasks update-overdue-by-tag \
  --tag "urgent" \
  --dry-run

# Delete completed tasks
clickup-cli tasks delete --id "task_abc123"
```

## 🤝 Contributing

We welcome contributions! This project is production-ready and actively maintained.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Install dependencies
cargo build

# Run tests (all should pass)
cargo test

# Check for issues (should be clean)
cargo clippy --all-targets --all-features -- -D warnings

# Run with debug logging
cargo run -- --debug auth test
```


### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests (requires API token)
cargo test -- --ignored

# Check for issues
cargo clippy --all-targets --all-features -- -D warnings
```

#### Integration Tests

The project includes comprehensive integration tests that verify functionality against the real ClickUp API:

```bash
# Set up test environment
cp env.test.example .env.test
# Edit .env.test with your test API token

# Run integration tests
./scripts/run_integration_tests.sh

# Or run directly with cargo
cargo test -- --ignored
```

**Note**: Integration tests require a valid ClickUp API token and will create/delete test data in your ClickUp workspace.

See [INTEGRATION_TESTS_README.md](INTEGRATION_TESTS_README.md) for detailed setup instructions.

#### Test Configuration

When running tests, the CLI will automatically use the `CLICKUP_API_TOKEN_TEST` environment variable if available:

```bash
# Set test token for running tests
export CLICKUP_API_TOKEN_TEST=your_test_api_token

# Or add to your .env file
echo "CLICKUP_API_TOKEN_TEST=your_test_api_token" >> .env

# Run tests
cargo test
```

This allows you to use a separate API token for testing without affecting your regular configuration.

### Documentation

```bash
# Generate API documentation
cargo doc --no-deps --open

# Check documentation
cargo doc --no-deps
```

## 📚 Development Documentation

For comprehensive development documentation, see:
**[dev-docs/PROJECT_STATUS.md](dev-docs/PROJECT_STATUS.md)** - Single source of truth for project status

**Key Documentation:**
- **`dev-docs/ARCHITECTURAL_ANALYSIS_AND_RECOMMENDATIONS.md`**: Comprehensive architectural analysis
- **`dev-docs/CLICKUP_API_COMPARISON.md`**: Detailed API implementation status
- **`dev-docs/CODE_REVIEW_RECOMMENDATIONS.md`**: Code quality recommendations
- **`dev-docs/adr/`**: Architecture Decision Records (13 documents)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ClickUp API](https://clickup.com/api) for providing the API
- [clap](https://github.com/clap-rs/clap) for excellent CLI argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) for HTTP client functionality
- [serde](https://github.com/serde-rs/serde) for serialization support

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/davidshq/clickup-utils/issues)
- **Documentation**: [API Documentation](https://docs.rs/clickup-cli) (when published)
- **Development Status**: See [dev-docs/PROJECT_STATUS.md](dev-docs/PROJECT_STATUS.md) for current development status

## 🔄 Changelog

This project is actively maintained. See [dev-docs/PROJECT_STATUS.md](dev-docs/PROJECT_STATUS.md) for current development status and recent improvements.

### Recent Changes
- Added comprehensive integration test suite with real API testing
- Improved error handling and user experience
- Enhanced documentation and code quality
- Added support for advanced task management features

For detailed change history, check the [GitHub commits](https://github.com/davidshq/clickup-utils/commits/main).