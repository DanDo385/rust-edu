//! # Modules and Crates Library
//!
//! This library demonstrates Rust's module system and crate organization.
//!
//! ## Quick Start
//!
//! ```
//! use modules_crates::prelude::*;
//!
//! // Create a user
//! let user = User::new("alice".to_string(), "alice@example.com".to_string());
//!
//! // Authenticate
//! let token = authenticate(&user);
//! println!("Token: {}", token.value());
//! ```

// Project 15: Modules and Crates - Library Crate
//
// This is the LIBRARY crate root (lib.rs). A library crate provides reusable
// functionality that can be used by binaries or other libraries.
//
// Key Concepts:
// - This file defines the PUBLIC API of our library
// - We use 'mod' to declare modules
// - We use 'pub' to expose items to library users
// - Everything is private by default
// - We can re-export items for convenience

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================
// This tells Rust about our modules. The compiler will look for:
// - src/models.rs OR src/models/mod.rs
// - src/services.rs OR src/services/mod.rs
// - src/utils.rs

pub mod models;      // Public module - users can access models::*
pub mod services;    // Public module - users can access services::*
mod utils;           // Private module - only this crate can use it

// ============================================================================
// RE-EXPORTS (Facade Pattern)
// ============================================================================
// Instead of forcing users to write:
//   use my_library::models::user::User;
//
// We can re-export commonly used items at the crate root:
//   use my_library::User;
//
// This creates a cleaner, more user-friendly API.

pub use models::user::User;
pub use services::auth::{authenticate, AuthToken};

// We can also create a "prelude" module with commonly used items
// This is a common pattern in Rust libraries (like std::prelude)
pub mod prelude {
    //! Prelude module containing commonly used items
    //!
    //! Import everything with: `use my_library::prelude::*;`

    pub use crate::models::user::User;
    pub use crate::services::auth::{authenticate, AuthToken};
}

// ============================================================================
// LIBRARY-LEVEL FUNCTIONS
// ============================================================================
// You can also provide utility functions at the crate root

/// Initialize the library with configuration
///
/// This is a common pattern for libraries that need initialization.
///
/// # Examples
///
/// ```
/// use modules_crates::init;
///
/// init("MyApp v1.0");
/// ```
pub fn init(app_name: &str) {
    println!("Initializing library for: {}", app_name);
    // In a real library, this might:
    // - Set up logging
    // - Initialize database connections
    // - Load configuration
    // - etc.
}

/// Get the library version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// ============================================================================
// PRIVATE HELPER FUNCTIONS
// ============================================================================
// Private functions can only be used within this crate

fn _internal_helper() {
    // This function is not visible to library users
    println!("Internal helper function");
}

// ============================================================================
// TESTS
// ============================================================================
// Unit tests can be included directly in the library modules

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }

    #[test]
    fn test_init() {
        // Just ensure it doesn't panic
        init("Test");
    }
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. lib.rs is the root of a library crate
// 2. Use 'mod' to declare modules (pub for public, private by default)
// 3. Use 'pub use' to re-export items for a cleaner API
// 4. Everything is private by default - use 'pub' intentionally
// 5. Modules provide privacy boundaries (encapsulation)
// 6. Create a prelude module for commonly used imports
// 7. Document your public API with doc comments (///)
// 8. Use #![doc] for crate-level documentation
// 9. Tests can live alongside code with #[cfg(test)]
// 10. Library crates are reusable - binaries use them

// ============================================================================
// VISIBILITY MODIFIERS
// ============================================================================
// pub          - Public to everyone
// pub(crate)   - Public within this crate only
// pub(super)   - Public to parent module only
// pub(in path) - Public to specific path only
// (none)       - Private to this module only

// Example:
pub(crate) fn crate_visible_function() {
    // Only visible within this crate, not to external users
}
