//! Models module
//!
//! This module contains data models used throughout the application.
//!
//! # Available Models
//!
//! - [`User`]: Represents a user in the system

// models/mod.rs - Module declaration file
//
// This file declares submodules within the 'models' module.
// It acts as the "index" for the models module.
//
// Directory structure:
//   src/models/
//   ├── mod.rs       <- You are here
//   └── user.rs      <- User model

// Declare the user submodule
// This tells Rust to look for src/models/user.rs
pub mod user;

// We can re-export items from submodules to make them easier to access
// Instead of: use my_library::models::user::User;
// Users can do: use my_library::models::User;
pub use user::User;

// ============================================================================
// MODULE-LEVEL FUNCTIONALITY
// ============================================================================
// You can also add module-level functions, types, or constants here

/// Model validation result
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Errors that can occur during model validation
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidEmail(String),
    InvalidUsername(String),
    TooShort(String),
    TooLong(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::InvalidEmail(msg) => write!(f, "Invalid email: {}", msg),
            ValidationError::InvalidUsername(msg) => write!(f, "Invalid username: {}", msg),
            ValidationError::TooShort(msg) => write!(f, "Too short: {}", msg),
            ValidationError::TooLong(msg) => write!(f, "Too long: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

// ============================================================================
// KEY TAKEAWAYS FOR MOD.RS
// ============================================================================
// 1. mod.rs is the "index file" for a module directory
// 2. Use 'pub mod' to declare public submodules
// 3. Use 'pub use' to re-export items from submodules
// 4. Can contain shared types, constants, or functions for the module
// 5. Acts as the entry point for the module
