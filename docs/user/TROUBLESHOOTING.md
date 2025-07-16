# ClickUp CLI - Troubleshooting Guide

This guide helps you resolve common issues when using the ClickUp CLI.

## Table of Contents

1. [Authentication Issues](#authentication-issues)
2. [Rate Limiting Problems](#rate-limiting-problems)
3. [Configuration Issues](#configuration-issues)
4. [Command Errors](#command-errors)
5. [Network and Connection Issues](#network-and-connection-issues)
6. [Debug Mode](#debug-mode)
7. [Getting Help](#getting-help)

## Authentication Issues

### Problem: "Authentication failed" or "Invalid API token"

**Symptoms:**
- Commands return authentication errors
- `clickup-cli auth test` fails
- "Invalid API token" error messages

**Solutions:**

1. **Verify your API token:**
   ```bash
   # Check if token is set
   clickup-cli auth status
   
   # Clear and reset token
   clickup-cli auth clear
   clickup-cli auth set
   ```

2. **Get a new API token:**
   - Log in to your ClickUp account
   - Go to **Settings** → **Apps**
   - Click **Generate API Token** (for personal use)
   - Copy the new token and set it:
   ```bash
   clickup-cli auth set --token "your-new-token"
   ```

3. **Check token permissions:**
   - Ensure your API token has the necessary permissions
   - For team apps, verify the app has access to the required workspaces

4. **Verify account status:**
   - Ensure your ClickUp account is active
   - Check if you have access to the workspaces you're trying to use

### Problem: "Token not found" or "No authentication configured"

**Symptoms:**
- Commands fail with "No authentication configured"
- `clickup-cli auth status` shows no token

**Solutions:**

1. **Set up authentication:**
   ```bash
   clickup-cli auth set
   ```

2. **Check configuration file:**
   ```bash
   # On Windows
   type %APPDATA%\clickup-cli\config.toml
   
   # On macOS/Linux
   cat ~/.config/clickup-cli/config.toml
   ```

3. **Set via environment variable:**
   ```bash
   export CLICKUP_API_TOKEN="your-api-token"
   clickup-cli auth test
   ```

## Rate Limiting Problems

### Problem: "Rate limit exceeded" or "Too many requests"

**Symptoms:**
- Commands fail with rate limiting errors
- Requests are being throttled
- Slow response times

**Solutions:**

1. **Check current rate limiting settings:**
   ```bash
   clickup-cli auth rate-limit --show
   ```

2. **Adjust for your account type:**
   ```bash
   # For paid accounts with higher limits
   clickup-cli auth rate-limit --requests-per-minute 500
   
   # For free accounts (default)
   clickup-cli auth rate-limit --requests-per-minute 100
   ```

3. **Disable auto-retry if needed:**
   ```bash
   clickup-cli auth rate-limit --auto-retry false
   ```

4. **Set custom retry behavior:**
   ```bash
   clickup-cli auth rate-limit --max-retries 5 --buffer-seconds 10
   ```

5. **Wait and retry:**
   - Rate limits reset every minute
   - Wait 60 seconds and try again

### Problem: Slow performance or timeouts

**Symptoms:**
- Commands take a long time to complete
- Timeout errors
- Inconsistent response times

**Solutions:**

1. **Increase timeout settings:**
   ```bash
   export CLICKUP_API_TIMEOUT="60"
   ```

2. **Reduce batch sizes:**
   ```bash
   export CLICKUP_DEFAULT_PAGE_SIZE="50"
   ```

3. **Check network connectivity:**
   ```bash
   # Test API connectivity
   curl -I https://api.clickup.com/api/v2/user
   ```

## Configuration Issues

### Problem: Configuration not loading or wrong values

**Symptoms:**
- Commands use wrong workspace/list IDs
- Configuration changes not taking effect
- Inconsistent behavior

**Solutions:**

1. **Check configuration precedence:**
   ```bash
   # Environment variables (highest priority)
   echo $CLICKUP_API_TOKEN
   echo $CLICKUP_WORKSPACE_ID
   
   # Configuration file
   cat ~/.config/clickup-cli/config.toml
   
   # .env file (if present)
   cat .env
   ```

2. **Clear conflicting configurations:**
   ```bash
   # Remove environment variables
   unset CLICKUP_API_TOKEN
   unset CLICKUP_WORKSPACE_ID
   
   # Remove .env file
   rm .env
   
   # Use only configuration file
   clickup-cli auth test
   ```

3. **Verify configuration file location:**
   ```bash
   # Windows
   echo %APPDATA%\clickup-cli\config.toml
   
   # macOS
   echo ~/Library/Application\ Support/clickup-cli/config.toml
   
   # Linux
   echo ~/.config/clickup-cli/config.toml
   ```

### Problem: Environment variables not working

**Symptoms:**
- Environment variables are ignored
- Configuration not updating
- Wrong values being used

**Solutions:**

1. **Check variable names:**
   ```bash
   # Correct variable names
   export CLICKUP_API_TOKEN="your-token"
   export CLICKUP_WORKSPACE_ID="workspace-id"
   export CLICKUP_DEFAULT_LIST_ID="list-id"
   ```

2. **Verify in current shell:**
   ```bash
   # Check if variables are set
   env | grep CLICKUP
   
   # Test in same shell
   clickup-cli auth test
   ```

3. **Restart shell session:**
   ```bash
   # Start new shell or reload profile
   source ~/.bashrc  # or ~/.zshrc
   ```

## Command Errors

### Problem: "Invalid ID" or "Not found" errors

**Symptoms:**
- Commands fail with "Invalid ID" errors
- Resources not found
- Wrong IDs being used

**Solutions:**

1. **Verify resource IDs:**
   ```bash
   # List workspaces to get correct IDs
   clickup-cli workspaces list
   
   # List spaces in workspace
   clickup-cli spaces list --workspace-id "workspace_id"
   
   # List lists in space
   clickup-cli lists list --space-id "space_id"
   ```

2. **Check ID format:**
   - Workspace IDs: Usually numeric or alphanumeric
   - Space IDs: Usually alphanumeric with underscores
   - List IDs: Usually alphanumeric with underscores
   - Task IDs: Usually alphanumeric with underscores

3. **Use interactive mode:**
   ```bash
   # CLI will prompt for correct IDs
   clickup-cli tasks create
   ```

### Problem: "Permission denied" or "Access denied"

**Symptoms:**
- Permission errors when accessing resources
- Cannot create/update tasks
- Access denied to workspaces

**Solutions:**

1. **Check your permissions:**
   - Verify you have access to the workspace
   - Check if you can create tasks in the list
   - Ensure you have comment permissions

2. **Use different API token:**
   ```bash
   # Generate new token with proper permissions
   clickup-cli auth clear
   clickup-cli auth set
   ```

3. **Check workspace access:**
   ```bash
   # List accessible workspaces
   clickup-cli workspaces list
   
   # Test with specific workspace
   clickup-cli spaces list --workspace-id "workspace_id"
   ```

### Problem: "Invalid status" or "Invalid priority"

**Symptoms:**
- Task creation/update fails with invalid status
- Priority values rejected
- Status not recognized

**Solutions:**

1. **Use correct status values:**
   ```bash
   # Common status values
   clickup-cli tasks create --status "to do"
   clickup-cli tasks create --status "in progress"
   clickup-cli tasks create --status "done"
   ```

2. **Use correct priority values:**
   ```bash
   # Priority levels (1-4, where 1 is highest)
   clickup-cli tasks create --priority 1  # Highest
   clickup-cli tasks create --priority 2  # High
   clickup-cli tasks create --priority 3  # Normal
   clickup-cli tasks create --priority 4  # Low
   ```

3. **Check custom statuses:**
   - Your ClickUp workspace may have custom statuses
   - Use the exact status name from your workspace

## Network and Connection Issues

### Problem: "Connection timeout" or "Network error"

**Symptoms:**
- Connection timeouts
- Network errors
- Cannot reach ClickUp API

**Solutions:**

1. **Check internet connectivity:**
   ```bash
   # Test basic connectivity
   ping api.clickup.com
   
   # Test API endpoint
   curl -I https://api.clickup.com/api/v2/user
   ```

2. **Increase timeout settings:**
   ```bash
   export CLICKUP_API_TIMEOUT="60"
   clickup-cli auth test
   ```

3. **Check proxy settings:**
   ```bash
   # If behind corporate proxy
   export HTTP_PROXY="http://proxy.company.com:8080"
   export HTTPS_PROXY="http://proxy.company.com:8080"
   ```

4. **Try different network:**
   - Test from different network
   - Check if corporate firewall is blocking

### Problem: SSL/TLS errors

**Symptoms:**
- SSL certificate errors
- TLS handshake failures
- Certificate validation errors

**Solutions:**

1. **Update system certificates:**
   ```bash
   # On Ubuntu/Debian
   sudo apt-get update && sudo apt-get install ca-certificates
   
   # On macOS
   sudo security find-certificate -a -p /System/Library/Keychains/SystemRootCertificates.keychain > /tmp/certs.pem
   ```

2. **Check system time:**
   ```bash
   # Ensure system time is correct
   date
   sudo ntpdate -s time.nist.gov  # Linux
   sudo sntp -sS time.apple.com   # macOS
   ```

## Debug Mode

### Enable Debug Logging

Debug mode provides detailed information about what the CLI is doing:

```bash
# Enable debug mode for any command
clickup-cli --debug auth test
clickup-cli --debug tasks list --list-id "list_id"
clickup-cli --debug workspaces list
```

### What Debug Mode Shows

- **API requests and responses**
- **Configuration loading**
- **Rate limiting information**
- **Error details**
- **Network connection details**

### Using Debug Output

1. **Run command with debug:**
   ```bash
   clickup-cli --debug auth test
   ```

2. **Look for error patterns:**
   - Authentication failures
   - Rate limiting issues
   - Network problems
   - Configuration issues

3. **Share debug output:**
   - When reporting issues, include debug output
   - Remove sensitive information (API tokens) before sharing

## Getting Help

### Self-Help Resources

1. **Command help:**
   ```bash
   clickup-cli --help
   clickup-cli auth --help
   clickup-cli tasks --help
   ```

2. **Documentation:**
   - [USER_GUIDE.md](USER_GUIDE.md) - Comprehensive user guide
   - [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick command reference
   - [README.md](README.md) - Main project documentation

3. **Examples:**
   - See the [Examples](#examples) section in USER_GUIDE.md
   - Check the main README.md for workflow examples

### Reporting Issues

When reporting issues, include:

1. **Error message and command:**
   ```bash
   clickup-cli --debug <command>
   ```

2. **System information:**
   ```bash
   # Operating system
   uname -a
   
   # Rust version
   rustc --version
   
   # CLI version
   clickup-cli --version
   ```

3. **Configuration (remove sensitive data):**
   ```bash
   # Show configuration (remove API token)
   clickup-cli auth status
   ```

4. **Steps to reproduce:**
   - Exact commands run
   - Expected vs actual behavior
   - Any relevant context

### Community Support

- **GitHub Issues**: [Report issues here](https://github.com/davidshq/clickup-utils/issues)
- **Documentation**: Check the [dev-docs/](dev-docs/) directory for development information
- **Project Status**: See [dev-docs/PROJECT_STATUS.md](dev-docs/PROJECT_STATUS.md) for current development status

## Common Error Messages and Solutions

| Error Message | Likely Cause | Solution |
|---------------|---------------|----------|
| "Authentication failed" | Invalid or expired API token | Get new token and set it |
| "Rate limit exceeded" | Too many API requests | Adjust rate limiting settings |
| "Invalid ID" | Wrong resource ID | Verify ID with list commands |
| "Permission denied" | Insufficient permissions | Check ClickUp permissions |
| "Connection timeout" | Network issues | Check connectivity and timeouts |
| "Configuration not found" | Missing config file | Set up authentication |
| "Invalid status" | Wrong status value | Use correct status names |
| "Not found" | Resource doesn't exist | Verify resource exists and is accessible |

---

*For more detailed information, see [USER_GUIDE.md](USER_GUIDE.md) and [README.md](README.md).* 