# ClickUp CLI - Installation Guide

This guide provides step-by-step installation instructions for the ClickUp CLI on different platforms.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation Methods](#installation-methods)
3. [Platform-Specific Instructions](#platform-specific-instructions)
4. [Post-Installation Setup](#post-installation-setup)
5. [Verification](#verification)
6. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Software

- **Rust 1.70+** - The CLI is written in Rust
- **Git** - For cloning the repository
- **ClickUp API token** - For authentication

### Getting Your ClickUp API Token

1. **Log in to your ClickUp account**
2. **Go to Settings** → **Apps**
3. **Choose one of the following options:**

   **Option A: Personal Token (Recommended for individual use)**
   - Click **Generate API Token**
   - Copy the generated token

   **Option B: App Token (For team/organization use)**
   - Click **Create New App**
   - Configure your app settings
   - Copy the API token from your app

## Installation Methods

### Method 1: Build from Source (Recommended)

This method builds the CLI from source code, ensuring you get the latest version with all features.

#### Step 1: Install Rust

**On Windows:**
```bash
# Download and run rustup-init.exe from https://rustup.rs/
# Or use winget
winget install Rust.Rust
```

**On macOS:**
```bash
# Using Homebrew
brew install rust

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Linux (Ubuntu/Debian):**
```bash
# Using apt
sudo apt update
sudo apt install rust-all

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Linux (CentOS/RHEL/Fedora):**
```bash
# Using dnf (Fedora)
sudo dnf install rust

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Step 2: Verify Rust Installation

```bash
# Check Rust version
rustc --version
cargo --version

# Should show version 1.70 or higher
```

#### Step 3: Clone and Build

```bash
# Clone the repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Build the application
cargo build --release

# The binary will be at target/release/clickup-cli
```

#### Step 4: Install to System Path

**Option A: Install to Cargo bin directory (Recommended)**
```bash
# Install to your cargo bin directory
cargo install --path .

# Now you can use clickup-cli from anywhere
clickup-cli --help
```

**Option B: Manual installation**
```bash
# Copy to a directory in your PATH
# On Linux/macOS
sudo cp target/release/clickup-cli /usr/local/bin/

# On Windows (PowerShell)
# Copy target/release/clickup-cli.exe to a directory in your PATH
# Or add target/release to your PATH environment variable
```

### Method 2: Download Pre-built Binary

**Note**: Pre-built binaries are not currently available. Use Method 1 to build from source.

### Method 3: Using Package Managers

**Note**: The CLI is not yet available in package managers. Use Method 1 to build from source.

## Platform-Specific Instructions

### Windows

#### Prerequisites

1. **Install Rust:**
   ```powershell
   # Using winget
   winget install Rust.Rust
   
   # Or download from https://rustup.rs/
   ```

2. **Install Git:**
   ```powershell
   # Using winget
   winget install Git.Git
   
   # Or download from https://git-scm.com/
   ```

#### Installation

```powershell
# Clone repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Build
cargo build --release

# Install
cargo install --path .

# Verify installation
clickup-cli --help
```

#### PATH Configuration

If `clickup-cli` is not found, add the cargo bin directory to your PATH:

1. **Find cargo bin directory:**
   ```powershell
   cargo --version
   # Look for the path in the output
   ```

2. **Add to PATH:**
   - Open System Properties → Advanced → Environment Variables
   - Add `%USERPROFILE%\.cargo\bin` to your PATH
   - Restart your terminal

### macOS

#### Prerequisites

1. **Install Rust:**
   ```bash
   # Using Homebrew
   brew install rust
   
   # Or using rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Git:**
   ```bash
   # Using Homebrew
   brew install git
   
   # Or using Xcode Command Line Tools
   xcode-select --install
   ```

#### Installation

```bash
# Clone repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Build
cargo build --release

# Install
cargo install --path .

# Verify installation
clickup-cli --help
```

### Linux (Ubuntu/Debian)

#### Prerequisites

1. **Install Rust:**
   ```bash
   # Using apt
   sudo apt update
   sudo apt install rust-all
   
   # Or using rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Git:**
   ```bash
   sudo apt update
   sudo apt install git
   ```

3. **Install build dependencies:**
   ```bash
   sudo apt install build-essential pkg-config libssl-dev
   ```

#### Installation

```bash
# Clone repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Build
cargo build --release

# Install
cargo install --path .

# Verify installation
clickup-cli --help
```

### Linux (CentOS/RHEL/Fedora)

#### Prerequisites

1. **Install Rust:**
   ```bash
   # Using dnf (Fedora)
   sudo dnf install rust
   
   # Or using rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Git:**
   ```bash
   # Using dnf
   sudo dnf install git
   ```

3. **Install build dependencies:**
   ```bash
   sudo dnf groupinstall "Development Tools"
   sudo dnf install openssl-devel
   ```

#### Installation

```bash
# Clone repository
git clone https://github.com/davidshq/clickup-utils.git
cd clickup-utils

# Build
cargo build --release

# Install
cargo install --path .

# Verify installation
clickup-cli --help
```

## Post-Installation Setup

### Step 1: Verify Installation

```bash
# Check if CLI is available
clickup-cli --help

# Check version
clickup-cli --version
```

### Step 2: Set Up Authentication

```bash
# Set your API token
clickup-cli auth set

# Test authentication
clickup-cli auth test
```

### Step 3: Test Basic Functionality

```bash
# List your workspaces
clickup-cli workspaces list

# Check authentication status
clickup-cli auth status
```

## Verification

### Quick Verification Script

Create a verification script to test your installation:

```bash
#!/bin/bash
# verify_installation.sh

echo "Verifying ClickUp CLI installation..."

# Check if CLI is available
if command -v clickup-cli &> /dev/null; then
    echo "✅ CLI is installed and available in PATH"
else
    echo "❌ CLI not found in PATH"
    exit 1
fi

# Check version
echo "CLI Version:"
clickup-cli --version

# Check help
echo "Help output:"
clickup-cli --help | head -10

# Test authentication (if configured)
echo "Testing authentication:"
if clickup-cli auth status &> /dev/null; then
    echo "✅ Authentication configured"
else
    echo "⚠️  Authentication not configured (run 'clickup-cli auth set')"
fi

echo "Installation verification complete!"
```

### Run Verification

```bash
# Make script executable
chmod +x verify_installation.sh

# Run verification
./verify_installation.sh
```

## Troubleshooting

### Common Installation Issues

#### Problem: "command not found: clickup-cli"

**Solutions:**

1. **Check if installed:**
   ```bash
   # Check cargo bin directory
   ls ~/.cargo/bin/clickup-cli
   
   # Or check target directory
   ls target/release/clickup-cli
   ```

2. **Add to PATH:**
   ```bash
   # Add cargo bin to PATH
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Reinstall:**
   ```bash
   cargo install --path .
   ```

#### Problem: Build fails with SSL errors

**Solutions:**

1. **Install SSL development libraries:**
   ```bash
   # Ubuntu/Debian
   sudo apt install libssl-dev pkg-config
   
   # CentOS/RHEL/Fedora
   sudo dnf install openssl-devel pkg-config
   
   # macOS
   brew install openssl pkg-config
   ```

2. **Set SSL environment variables:**
   ```bash
   export OPENSSL_DIR=/usr/local/opt/openssl
   export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
   ```

#### Problem: "Permission denied" during installation

**Solutions:**

1. **Fix permissions:**
   ```bash
   # Fix cargo directory permissions
   sudo chown -R $USER:$USER ~/.cargo
   ```

2. **Use user installation:**
   ```bash
   # Install to user directory
   cargo install --path . --user
   ```

#### Problem: Rust version too old

**Solutions:**

1. **Update Rust:**
   ```bash
   rustup update
   ```

2. **Check version:**
   ```bash
   rustc --version
   # Should be 1.70 or higher
   ```

### Platform-Specific Issues

#### Windows Issues

**Problem: "cargo: command not found"**
```powershell
# Add Rust to PATH
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

**Problem: Build fails with Visual Studio errors**
```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

#### macOS Issues

**Problem: "xcrun: error: invalid active developer path"**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

**Problem: SSL certificate errors**
```bash
# Install certificates
brew install ca-certificates
```

#### Linux Issues

**Problem: "No such file or directory" during build**
```bash
# Install build essentials
sudo apt install build-essential  # Ubuntu/Debian
sudo dnf groupinstall "Development Tools"  # CentOS/RHEL/Fedora
```

**Problem: "pkg-config: command not found"**
```bash
# Install pkg-config
sudo apt install pkg-config  # Ubuntu/Debian
sudo dnf install pkg-config  # CentOS/RHEL/Fedora
```

### Getting Help

If you encounter issues not covered here:

1. **Check the troubleshooting guide:**
   - See [TROUBLESHOOTING.md](TROUBLESHOOTING.md)

2. **Enable debug mode:**
   ```bash
   clickup-cli --debug auth test
   ```

3. **Report issues:**
   - Create an issue on [GitHub](https://github.com/davidshq/clickup-utils/issues)
   - Include your system information and error messages

4. **Check system requirements:**
   ```bash
   # System information
   uname -a
   
   # Rust version
   rustc --version
   
   # Git version
   git --version
   ```

## Next Steps

After successful installation:

1. **Read the user guide:** [USER_GUIDE.md](USER_GUIDE.md)
2. **Check the quick reference:** [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
3. **Set up authentication:** `clickup-cli auth set`
4. **Test basic commands:** `clickup-cli workspaces list`

---

*For detailed usage information, see [USER_GUIDE.md](USER_GUIDE.md) and [README.md](README.md).* 