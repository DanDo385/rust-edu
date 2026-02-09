//! Reference solution facade for Lab 18.
//!
//! Classroom narrative:
//! 1. `models` and `services` modules own the concrete types and auth flows.
//! 2. This facade re-exports items so consumers get a curated API without duplicating ownership.
//! 3. `init` and `version` borrow strings/runtimes immutably—no heap clones.

pub use crate::models;
pub use crate::services;
pub use models::user::User;
pub use services::auth::{authenticate, AuthToken};

pub mod prelude {
    pub use crate::models::user::User;
    pub use crate::services::auth::{authenticate, AuthToken};
}

pub fn init(app_name: &str) {
    println!("Initializing library for: {}", app_name);
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
