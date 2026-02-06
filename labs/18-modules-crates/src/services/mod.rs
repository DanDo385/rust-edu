//! Services module
//!
//! Contains business logic and service layer functionality.
//!
//! # Available Services
//!
//! - [`auth`]: Authentication and authorization services

// services/mod.rs - Services module declaration
//
// This module contains business logic and services.

pub mod auth;

// Re-export commonly used items
pub use auth::{authenticate, AuthToken, AuthService};
