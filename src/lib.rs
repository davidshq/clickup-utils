//! # ClickUp CLI Library
//!
//! This library provides a comprehensive command-line interface for the ClickUp API.
//! It includes modules for API communication, configuration management, error handling,
//! data models, rate limiting, and command implementations.
//!
//! ## Modules
//!
//! - **api**: ClickUp API client for making authenticated requests
//! - **commands**: CLI command implementations for all operations
//! - **config**: Configuration management and settings
//! - **error**: Error handling and custom error types
//! - **models**: Data structures for API communication
//! - **rate_limiter**: Rate limiting functionality for API requests
//!
//! ## Usage
//!
//! This library is primarily used by the `clickup-cli` binary, but can also be
//! used as a dependency in other Rust projects that need ClickUp API functionality.

pub mod api;
pub mod commands;
pub mod config;
pub mod constants;
pub mod error;
pub mod models;
pub mod rate_limiter;
