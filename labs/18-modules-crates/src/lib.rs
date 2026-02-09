//! # Lab 18: Modules and Crates
//!
//! Student-facing crate root. Public API functions are stubs to implement.

pub mod models;
pub mod services;
mod utils;

pub use models::user::User;
pub use services::auth::{authenticate, AuthToken};

pub mod prelude {
    pub use crate::models::user::User;
    pub use crate::services::auth::{authenticate, AuthToken};
}

/// Initialize the library.
pub fn init(app_name: &str) {
    // TODO: Initialize library state for an app.
    let _ = app_name;
    todo!("Initialize the modules-crates library")
}

/// Return current crate version.
pub fn version() -> &'static str {
    // TODO: Return version from Cargo metadata.
    todo!("Return crate version")
}

#[doc(hidden)]
pub mod solution;
