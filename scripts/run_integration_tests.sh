#!/bin/bash

# ClickUp CLI Integration Test Runner
# 
# This script runs the integration tests for the ClickUp CLI with proper
# environment setup and cleanup.
# 
# Usage:
#   ./scripts/run_integration_tests.sh [test_name]
# 
# Examples:
#   ./scripts/run_integration_tests.sh                    # Run all integration tests
#   ./scripts/run_integration_tests.sh test_authentication # Run specific test

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    print_error "This script must be run from the project root directory"
    exit 1
fi

# Check if .env.test exists
if [ ! -f ".env.test" ]; then
    print_warning ".env.test file not found"
    print_status "Copying env.test.example to .env.test"
    cp env.test.example .env.test
    print_warning "Please edit .env.test and add your test credentials"
    print_warning "Then run this script again"
    exit 1
fi

# Check if CLICKUP_API_TOKEN_TEST is set
if [ -z "$CLICKUP_API_TOKEN_TEST" ]; then
    # Try to load from .env.test
    if [ -f ".env.test" ]; then
        export $(grep -v '^#' .env.test | xargs)
    fi
fi

if [ -z "$CLICKUP_API_TOKEN_TEST" ]; then
    print_error "CLICKUP_API_TOKEN_TEST environment variable is not set"
    print_status "Please set it in your .env.test file or export it directly"
    exit 1
fi

print_status "Running integration tests with token: ${CLICKUP_API_TOKEN_TEST:0:8}..."

# Build the project first
print_status "Building project..."
cargo build --release

# Run the tests
if [ -n "$1" ]; then
    # Run specific test
    print_status "Running test: $1"
    cargo test --release -- --ignored "$1"
else
    # Run all integration tests except test_invalid_authentication
    print_status "Running all integration tests except test_invalid_authentication..."
    cargo test --release -- --ignored --skip test_invalid_authentication
    status=$?
    # Now run test_invalid_authentication last
    print_status "Running test_invalid_authentication (last)..."
    cargo test --release -- --ignored test_invalid_authentication
    status2=$?
    if [ $status -eq 0 ] && [ $status2 -eq 0 ]; then
        print_success "All integration tests passed!"
        exit 0
    else
        print_error "Some integration tests failed"
        exit 1
    fi
fi 