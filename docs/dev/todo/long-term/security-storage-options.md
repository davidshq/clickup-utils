# Security Storage Options for ClickUp CLI

## Overview
This document explains the various options for storing secrets (like API tokens) securely in the ClickUp CLI, comparing their security benefits and implementation complexity.

---

## Current State: Moderate Security Risk

**The current implementation has some security concerns but isn't catastrophic:**

### ✅ **Current Security Measures in Place:**
1. **Git ignore protection**: `.env` files are properly excluded from version control
2. **Multi-source configuration**: Tokens can be stored in system config directories (more secure than project-local `.env`)
3. **Environment variable precedence**: System environment variables take priority over `.env` files
4. **Test separation**: Separate `.env.test` files prevent test tokens from mixing with production

### ⚠️ **Security Issues with Current `.env` Approach:**
1. **Plain text storage**: Tokens are stored in plain text in `.env` files
2. **File system exposure**: Anyone with file system access can read the tokens
3. **No encryption**: No protection against local file system attacks
4. **No token rotation**: No automatic token expiration or rotation
5. **No secure deletion**: Tokens remain in file system even after "deletion"

---

## Security Storage Options Comparison

### 1. System Config Directory (`~/.config/clickup-cli/config.toml`)

**Current Implementation Status:** ✅ **Implemented**

#### File System Isolation & Permissions

**.env files (Project-local):**
- Located in project directory: `./project/.env`
- Accessible to anyone with project directory access
- Often shared across development environments
- Can be accidentally committed to version control
- Inherits project directory permissions

**System config directory (`~/.config/clickup-cli/config.toml`):**
- Located in user's private config directory
- Standard location: `~/.config/clickup-cli/config.toml` (Linux/macOS) or `%APPDATA%\clickup-cli\config.toml` (Windows)
- Only accessible to the user who owns the directory
- Automatically created with proper user permissions
- Isolated from project files and version control

#### Cross-Platform Standardization

The `dirs::config_dir()` function provides platform-specific secure locations:

- **Linux:** `~/.config/clickup-cli/config.toml`
- **macOS:** `~/Library/Application Support/clickup-cli/config.toml`
- **Windows:** `%APPDATA%\clickup-cli\config.toml`

#### User-Level Isolation

**.env files:**
- Shared across all users on the same machine
- Project-specific, not user-specific
- Can be accessed by other developers working on the same project

**System config:**
- User-specific storage
- Each user has their own isolated configuration
- No cross-user contamination

#### Backup & System Integration

**.env files:**
- Often excluded from backups
- Project-specific backup strategies
- May be lost when projects are moved/deleted

**System config:**
- Automatically included in user profile backups
- Survives project changes and deletions
- Integrated with system backup tools

#### Configuration Precedence

The system implements a smart precedence order:

1. Environment variables (highest priority - most secure)
2. .env file (loaded automatically if present)
3. Configuration file (system config directory)
4. Default values (lowest priority)

This means you can:
- Use environment variables for the most sensitive data
- Use system config for persistent, less sensitive settings
- Use `.env` only for development convenience

#### Automatic Directory Creation & Permissions

The system automatically creates the config directory with proper permissions:

```rust
let config_dir = dirs::config_dir()?.join("clickup-cli");
std::fs::create_dir_all(&config_dir)?; // Creates with user permissions
```

**Pros:**
- ✅ Already implemented
- ✅ User isolation
- ✅ File system security
- ✅ System integration
- ✅ Backup resilience
- ✅ Cross-platform standardization

**Cons:**
- ❌ Still plain text storage
- ❌ No encryption
- ❌ No token rotation

---

### 2. System Keyring Storage

**Implementation Status:** ⚠️ **PENDING**

#### What It Is
System keyring integration using the `keyring` crate to store tokens in the OS's secure credential storage:

- **Windows:** Windows Credential Manager
- **macOS:** Keychain
- **Linux:** Secret Service API (GNOME Keyring, KWallet)

#### Security Benefits
- **Encrypted storage**: Tokens are encrypted by the OS
- **Access control**: OS-level permission management
- **No file system exposure**: Tokens not stored in plain text files
- **Automatic cleanup**: OS handles secure deletion
- **Integration**: Works with existing OS security features

#### Implementation Complexity
**Medium complexity** - requires:
- Adding `keyring` dependency
- Implementing keyring wrapper functions
- Error handling for keyring failures
- Fallback to existing methods

#### Example Implementation
```rust
use keyring::Keyring;

pub struct SecureTokenStorage {
    keyring: Keyring,
}

impl SecureTokenStorage {
    pub fn new() -> Self {
        Self {
            keyring: Keyring::new("clickup-cli", "api-token"),
        }
    }

    pub fn store_token(&self, token: &str) -> Result<(), ClickUpError> {
        self.keyring.set_password(token)
            .map_err(|e| ClickUpError::ConfigError(format!("Failed to store token: {e}")))
    }

    pub fn retrieve_token(&self) -> Result<String, ClickUpError> {
        self.keyring.get_password()
            .map_err(|e| ClickUpError::AuthError(format!("Failed to retrieve token: {e}")))
    }

    pub fn delete_token(&self) -> Result<(), ClickUpError> {
        self.keyring.delete_password()
            .map_err(|e| ClickUpError::ConfigError(format!("Failed to delete token: {e}")))
    }
}
```

**Pros:**
- ✅ OS-level encryption
- ✅ No plain text storage
- ✅ Automatic secure deletion
- ✅ OS integration

**Cons:**
- ❌ Platform-specific implementation
- ❌ Requires additional dependency
- ❌ More complex error handling
- ❌ May not work in all environments (CI/CD, containers)

---

### 3. OAuth 2.0 Flow

**Implementation Status:** ⚠️ **PENDING**

#### What It Is
Proper OAuth 2.0 authentication flow instead of API token storage:

1. User initiates OAuth flow
2. Redirect to ClickUp authorization
3. User grants permissions
4. Receive access token and refresh token
5. Store refresh token securely
6. Automatically refresh access tokens

#### Security Benefits
- **No long-lived tokens**: Access tokens expire quickly
- **Automatic refresh**: Seamless token renewal
- **Granular permissions**: User controls what the app can access
- **Revocable access**: Users can revoke access anytime
- **Standard protocol**: Industry-standard security

#### Implementation Complexity
**High complexity** - requires:
- OAuth 2.0 client implementation
- Web server for callback handling
- Token refresh logic
- Secure refresh token storage
- Error handling for expired/revoked tokens

#### Example Flow
```rust
pub struct OAuthFlow {
    client_id: String,
    redirect_uri: String,
    scopes: Vec<String>,
}

impl OAuthFlow {
    pub fn authorize_url(&self) -> String {
        format!(
            "https://app.clickup.com/api/v2/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
            self.client_id,
            self.redirect_uri,
            self.scopes.join(",")
        )
    }

    pub async fn exchange_code_for_tokens(&self, code: &str) -> Result<Tokens, ClickUpError> {
        // Exchange authorization code for access and refresh tokens
    }

    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<Tokens, ClickUpError> {
        // Use refresh token to get new access token
    }
}
```

**Pros:**
- ✅ Industry-standard security
- ✅ Automatic token refresh
- ✅ Granular permissions
- ✅ User-controlled access
- ✅ No long-lived secrets

**Cons:**
- ❌ Very high implementation complexity
- ❌ Requires web server for callbacks
- ❌ More complex user experience
- ❌ ClickUp may not support full OAuth 2.0

---

### 4. Token Rotation & Expiration

**Implementation Status:** ⚠️ **PENDING**

#### What It Is
Automatic token validation, expiration detection, and rotation:

- **Token validation**: Check if token is still valid
- **Expiration detection**: Detect when tokens expire
- **Automatic rotation**: Generate new tokens when needed
- **Graceful degradation**: Handle expired tokens gracefully

#### Security Benefits
- **Reduced exposure window**: Tokens expire automatically
- **Automatic cleanup**: Expired tokens are invalid
- **Proactive security**: Detect compromised tokens
- **User notification**: Alert users to token issues

#### Implementation Complexity
**Low to Medium complexity** - requires:
- Token validation API calls
- Expiration tracking
- User notification system
- Graceful error handling

#### Example Implementation
```rust
pub struct TokenManager {
    config: Config,
}

impl TokenManager {
    pub async fn validate_token(&self) -> Result<bool, ClickUpError> {
        // Make API call to validate token
        let response = self.api.get_user_info().await?;
        Ok(response.is_ok())
    }

    pub async fn check_token_expiration(&self) -> Result<TokenStatus, ClickUpError> {
        // Check if token is expired or will expire soon
    }

    pub async fn rotate_token_if_needed(&self) -> Result<(), ClickUpError> {
        // Automatically rotate token if expired or expiring soon
    }
}
```

**Pros:**
- ✅ Reduces token exposure time
- ✅ Automatic security improvements
- ✅ Better user experience
- ✅ Proactive security

**Cons:**
- ❌ Requires API calls for validation
- ❌ May not work with all token types
- ❌ Additional complexity

---

### 5. Secure Deletion

**Implementation Status:** ⚠️ **PENDING**

#### What It Is
Secure token deletion that overwrites data before deletion:

- **Memory overwrite**: Clear tokens from memory
- **File overwrite**: Overwrite file contents before deletion
- **Secure cleanup**: Ensure no traces remain

#### Security Benefits
- **No data remnants**: Tokens completely removed
- **Memory security**: Tokens cleared from RAM
- **File system security**: Overwritten before deletion

#### Implementation Complexity
**Low complexity** - requires:
- Memory clearing functions
- File overwrite utilities
- Secure deletion wrappers

#### Example Implementation
```rust
pub trait SecureDeletion {
    fn secure_delete(&mut self) -> Result<(), ClickUpError>;
}

impl SecureDeletion for Config {
    fn secure_delete(&mut self) -> Result<(), ClickUpError> {
        // Overwrite token in memory
        if let Some(token) = &mut self.api_token {
            for byte in token.as_mut() {
                *byte = 0;
            }
        }
        
        // Clear token
        self.api_token = None;
        
        // Save empty config
        self.save()?;
        
        Ok(())
    }
}
```

**Pros:**
- ✅ Complete token removal
- ✅ Memory security
- ✅ No data remnants

**Cons:**
- ❌ Limited effectiveness on modern systems
- ❌ Garbage collection may retain references
- ❌ OS-level caching may persist data

---

## Risk Assessment by Use Case

### **Low Risk Scenarios:**
- **Personal development**: Single-user machine, local development
- **CI/CD with secrets**: When tokens are injected as environment variables
- **Temporary usage**: When tokens are set via environment variables only

### **Medium Risk Scenarios:**
- **Shared development machines**: Multiple developers using same machine
- **Laptops with physical access**: Anyone with physical access can read `.env` files
- **Backup systems**: `.env` files might be backed up insecurely

### **High Risk Scenarios:**
- **Production deployments**: Using `.env` files in production environments
- **Multi-user systems**: Shared servers or workstations
- **Unencrypted storage**: Laptops without full disk encryption

---

## Implementation Recommendations

### **Phase 1: Immediate (Low Effort)**
**Priority: High**
**Timeline: 1-2 weeks**

1. **Use system config directory**: Store tokens in `~/.config/clickup-cli/config.toml` instead of `.env`
2. **Environment variables only**: Use `export CLICKUP_API_TOKEN=...` for sensitive environments
3. **File permissions**: Set restrictive permissions on `.env` files (`chmod 600 .env`)

**Implementation:**
```bash
# Update documentation to recommend system config
# Add file permission checks
# Update auth commands to use system config by default
```

### **Phase 2: Short-term (Medium Effort)**
**Priority: High**
**Timeline: 2-4 weeks**

1. **Implement secure storage**: Use system keyring (keyring crate) for token storage
2. **Token validation**: Add token expiration checking
3. **Secure deletion**: Implement secure token removal

**Implementation:**
```rust
// Add keyring dependency
keyring = "0.12"

// Implement SecureTokenStorage struct
// Add token validation methods
// Update auth commands to use keyring
```

### **Phase 3: Medium-term (Medium Effort)**
**Priority: Medium**
**Timeline: 1-2 months**

1. **Token rotation**: Automatic token refresh and rotation
2. **Audit logging**: Track token usage and access
3. **Enhanced validation**: More comprehensive token validation

**Implementation:**
```rust
// Add token rotation logic
// Implement audit logging
// Add comprehensive validation
```

### **Phase 4: Long-term (High Effort)**
**Priority: Low**
**Timeline: 3-6 months**

1. **OAuth flow**: Implement proper OAuth 2.0 authentication
2. **Advanced security**: Multi-factor authentication support
3. **Enterprise features**: SSO integration, role-based access

**Implementation:**
```rust
// Full OAuth 2.0 implementation
// Web server for callbacks
// Enterprise security features
```

---

## Practical Security Improvements

### **For Production Use:**
```bash
# Use environment variables (most secure)
export CLICKUP_API_TOKEN="your-token"

# Or use system config (persistent, user-isolated)
clickup-cli auth set  # Saves to ~/.config/clickup-cli/config.toml
```

### **For Development:**
```bash
# Still use .env for convenience, but be aware of risks
echo "CLICKUP_API_TOKEN=dev-token" > .env

# Or use system config for better security
clickup-cli auth set  # Saves to system config
```

---

## Bottom Line

The system config directory approach provides **significant security improvements** over `.env` files:

1. **User isolation** - Each user has their own config
2. **File system security** - Proper permissions and isolation
3. **System integration** - Works with OS security features
4. **Backup resilience** - Survives project changes
5. **Cross-platform standardization** - Consistent behavior across OS

**Recommended Implementation Order:**
1. **Phase 1**: System config directory (immediate)
2. **Phase 2**: System keyring storage (short-term)
3. **Phase 3**: Token validation and rotation (medium-term)
4. **Phase 4**: OAuth 2.0 (long-term)

While both approaches still store tokens in plain text, the system config directory is **substantially more secure** than project-local `.env` files, especially in multi-user or production environments. 