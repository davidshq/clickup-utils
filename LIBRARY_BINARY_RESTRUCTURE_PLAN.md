# Library/Binary Restructure Plan

## Overview

This document outlines the plan to restructure the ClickUp CLI project to follow Rust best practices for separating library and binary functionality. Based on [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/binaries-vs-libraries.html), we will create separate crates for the library and binary components.

## Current State Analysis

### Problems with Current Structure
- Mixed library/binary in single crate causing import conflicts
- `src/main.rs` and `src/lib.rs` both trying to use same modules
- Import confusion between `crate::` and `clickup_cli::` contexts
- Violates best practices for public crates

### Current Structure
```
clickup-cli/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── main.rs         # Binary entry point (conflicts with lib.rs)
│   ├── api.rs          # Used by both lib and binary
│   ├── rate_limiter.rs # Used by both lib and binary
│   ├── config.rs       # Used by both lib and binary
│   ├── error.rs        # Used by both lib and binary
│   ├── models.rs       # Used by both lib and binary
│   └── commands/       # CLI-specific code
└── Cargo.toml          # Single crate configuration
```

## Target Structure

### Option 1: Separate Crates (Recommended)
```
clickup-core/           # Library crate
├── src/
│   ├── lib.rs
│   ├── api.rs
│   ├── rate_limiter.rs
│   ├── config.rs
│   ├── error.rs
│   └── models.rs
└── Cargo.toml

clickup-cli/            # Binary crate
├── src/
│   ├── main.rs
│   └── commands/       # CLI-specific code
├── Cargo.toml          # Depends on clickup-core
└── README.md
```

### Option 2: Workspace Structure
```
clickup-workspace/
├── Cargo.toml          # Workspace root
├── clickup-core/       # Library crate
│   ├── src/
│   │   ├── lib.rs
│   │   ├── api.rs
│   │   ├── rate_limiter.rs
│   │   ├── config.rs
│   │   ├── error.rs
│   │   └── models.rs
│   └── Cargo.toml
└── clickup-cli/        # Binary crate
    ├── src/
    │   ├── main.rs
    │   └── commands/
    └── Cargo.toml
```

## Implementation Plan

### Phase 1: Create Library Crate (`clickup-core`)

#### Step 1.1: Create New Library Crate
```bash
# Create new library crate
cargo new clickup-core --lib
cd clickup-core
```

#### Step 1.2: Move Core Modules
Move the following files from `clickup-cli/src/` to `clickup-core/src/`:
- `api.rs`
- `rate_limiter.rs`
- `config.rs`
- `error.rs`
- `models.rs`
- `constants.rs` (newly created)

#### Step 1.3: Update Library Crate Dependencies
```toml
# clickup-core/Cargo.toml
[package]
name = "clickup-core"
version = "0.1.0"
edition = "2021"
description = "Core library for ClickUp API functionality"
license = "MIT"

[dependencies]
# HTTP client for API requests
reqwest = { version = "0.12.22", features = ["json"] }

# JSON serialization and deserialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime for concurrent operations
tokio = { version = "1.0", features = ["full"] }

# Configuration management
config = "0.15.13"
dirs = "6.0.0"
dotenvy = "0.15"

# Error handling
thiserror = "2.0.12"

# Date and time handling
chrono = { version = "0.4", features = ["serde"] }

# Async utilities
futures = "0.3"

# Logging
log = "0.4"
```

#### Step 1.4: Update Library Exports
```rust
// clickup-core/src/lib.rs
pub mod api;
pub mod config;
pub mod constants;
pub mod error;
pub mod models;
pub mod rate_limiter;

// Re-export main types for convenience
pub use api::ClickUpApi;
pub use config::{Config, RateLimitConfig};
pub use error::ClickUpError;
pub use models::*;
```

### Phase 2: Update Binary Crate (`clickup-cli`)

#### Step 2.1: Remove Core Modules from Binary
Remove the following files from `clickup-cli/src/`:
- `api.rs`
- `rate_limiter.rs`
- `config.rs`
- `error.rs`
- `models.rs`
- `constants.rs`

#### Step 2.2: Update Binary Dependencies
```toml
# clickup-cli/Cargo.toml
[package]
name = "clickup-cli"
version = "0.1.0"
edition = "2021"
description = "A command-line interface for ClickUp API"
license = "MIT"

[dependencies]
clickup-core = { path = "../clickup-core" }

# CLI and argument parsing
clap = { version = "4.4", features = ["derive"] }

# Terminal output and formatting
colored = "3.0.0"
comfy-table = "7.1"

# TOML serialization
toml = "0.9.0"

# Logging
env_logger = "0.11.8"
log = "0.4"

# Async runtime
tokio = { version = "1.0", features = ["full"] }
```

#### Step 2.3: Update Binary Imports
```rust
// clickup-cli/src/main.rs
use clickup_core::{ClickUpApi, Config};
use clap::{Parser, Subcommand};
use log::{error, info};

mod commands;

use crate::commands::{auth, comments, lists, spaces, tasks, teams, workspaces};
```

#### Step 2.4: Update Command Imports
Update all command files to use `clickup_core::` instead of `crate::`:

```rust
// clickup-cli/src/commands/utils.rs
use clickup_core::{ClickUpApi, ClickUpError, Config};
// ... rest of imports
```

### Phase 3: Update Tests

#### Step 3.1: Update Integration Tests
```rust
// tests/api_tests.rs
use clickup_core::{ClickUpApi, Config};
use test_utils::{TestApiUtils, TestConfig};
```

#### Step 3.2: Update Test Utilities
```rust
// tests/test_utils.rs
use clickup_core::{ClickUpApi, Config};
use clickup_core::commands::utils::ApiUtils;
```

### Phase 4: Documentation and Examples

#### Step 4.1: Update Library Documentation
```rust
// clickup-core/src/lib.rs
//! # ClickUp Core Library
//!
//! This library provides core functionality for interacting with the ClickUp API.
//! It includes API client, configuration management, error handling, and data models.
//!
//! ## Usage
//!
//! ```rust
//! use clickup_core::{ClickUpApi, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::load()?;
//!     let api = ClickUpApi::new(config)?;
//!     
//!     let user = api.get_user().await?;
//!     println!("Hello, {}!", user.user.username);
//!     
//!     Ok(())
//! }
//! ```
```

#### Step 4.2: Update Binary Documentation
```rust
// clickup-cli/src/main.rs
//! # ClickUp CLI
//!
//! A command-line interface for the ClickUp API.
//! This binary provides a comprehensive CLI tool for managing ClickUp workspaces,
//! teams, spaces, lists, tasks, and comments.
```

## Benefits of This Approach

### 1. Clear Separation of Concerns
- **Library**: Core API functionality, reusable across projects
- **Binary**: CLI-specific code, user interface

### 2. Better Versioning
- Library and binary can have independent versioning
- Library can be used by other projects without CLI dependencies

### 3. Reduced Dependencies
- Library doesn't need CLI-specific dependencies (clap, colored, etc.)
- Binary only includes what it needs

### 4. Better Testing
- Library can be tested independently
- Integration tests can focus on CLI behavior

### 5. Easier Maintenance
- Clear boundaries between library and CLI code
- Easier to add new interfaces (web API, GUI, etc.)

## Migration Checklist

### Pre-Migration
- [ ] Create backup of current project
- [ ] Document current functionality
- [ ] Identify all import dependencies

### Phase 1: Library Crate
- [ ] Create `clickup-core` crate
- [ ] Move core modules (`api.rs`, `config.rs`, etc.)
- [ ] Update `Cargo.toml` dependencies
- [ ] Update library exports
- [ ] Test library builds successfully

### Phase 2: Binary Crate
- [ ] Remove core modules from binary
- [ ] Update binary dependencies to use `clickup-core`
- [ ] Update all imports to use `clickup_core::`
- [ ] Test binary builds successfully

### Phase 3: Tests
- [ ] Update integration tests
- [ ] Update test utilities
- [ ] Verify all tests pass

### Phase 4: Documentation
- [ ] Update library documentation
- [ ] Update binary documentation
- [ ] Update README files
- [ ] Update examples

### Post-Migration
- [ ] Test full functionality
- [ ] Update CI/CD pipelines
- [ ] Update deployment scripts
- [ ] Publish to crates.io (if applicable)

## Alternative: Workspace Approach

If you prefer to keep everything in one repository, consider using a workspace:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "clickup-core",
    "clickup-cli"
]
```

This approach maintains the same separation but keeps everything in one repository.

## Timeline Estimate

- **Phase 1**: 2-3 hours
- **Phase 2**: 2-3 hours  
- **Phase 3**: 1-2 hours
- **Phase 4**: 1-2 hours
- **Testing & Polish**: 2-3 hours

**Total**: 8-13 hours

## Risk Mitigation

1. **Create comprehensive tests** before migration
2. **Use feature branches** for each phase
3. **Test thoroughly** after each phase
4. **Keep backup** of working version
5. **Document all changes** for rollback if needed

## References

- [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/binaries-vs-libraries.html)
- [Organizing code in binary crates](https://rust-cli-recommendations.sunshowers.io/organizing-binary.html)
- [Rust workspace documentation](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) 