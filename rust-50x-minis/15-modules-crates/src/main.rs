// Project 15: Modules and Crates - Binary Crate
//
// This is the BINARY crate (main.rs). It uses the library crate (lib.rs)
// to demonstrate how libraries and binaries work together.
//
// Key Concepts:
// - A binary uses the library by importing it
// - The binary can only access PUBLIC items from the library
// - Private items in the library are hidden
// - This separation allows for clean API design

// Import items from our library crate
// The crate name comes from Cargo.toml's [package] name
// But since we're in the same crate, we use the special name of the project
use modules_crates::prelude::*;  // Use the prelude for common items
use modules_crates::{init, version};  // Import crate-level functions
use modules_crates::models;  // Import modules module
use modules_crates::services;  // Import services module

fn main() {
    println!("=== Modules and Crates Demo ===\n");

    // ========================================================================
    // PART 1: Library Initialization
    // ========================================================================
    println!("--- Part 1: Library Initialization ---\n");

    init("MyApplication v1.0");
    println!("Library version: {}", version());

    println!();

    // ========================================================================
    // PART 2: Using Models
    // ========================================================================
    println!("--- Part 2: Using Models ---\n");

    // Create users using the public API
    // Note: We can't access private fields directly!

    let user1 = User::new("alice".to_string(), "alice@example.com".to_string());
    println!("Created user: {}", user1);
    println!("  Username: {}", user1.username());
    println!("  Email: {}", user1.email());
    println!("  Active: {}", user1.is_active());

    // Create user with validation
    match User::validated("bob".to_string(), "bob@example.com".to_string()) {
        Ok(user) => println!("Validated user: {}", user),
        Err(err) => eprintln!("Validation error: {}", err),
    }

    // Try to create invalid user
    println!("\nAttempting to create invalid user:");
    match User::validated("ab".to_string(), "invalid-email".to_string()) {
        Ok(user) => println!("User created: {}", user),
        Err(err) => println!("  Validation failed: {}", err),
    }

    // We can also access through the full path
    let user2 = models::User::new("charlie".to_string(), "charlie@example.com".to_string());
    println!("\nUser via full path: {}", user2.display_name());

    println!();

    // ========================================================================
    // PART 3: Using Services
    // ========================================================================
    println!("--- Part 3: Using Services ---\n");

    // Authenticate a user
    println!("Authenticating user1...");
    let token = authenticate(&user1);
    println!("  Generated token: {}", token);
    println!("  Token value: {}", token.value());
    println!("  User ID: {}", token.user_id());
    println!("  Valid: {}", token.is_valid());

    // Verify token
    println!("\nVerifying token...");
    let is_valid = services::auth::verify_token(&token, &user1);
    println!("  Token valid for user1: {}", is_valid);

    // Create an auth service
    println!("\nUsing AuthService:");
    let auth_service = services::AuthService::new("MyAuthService".to_string());
    let token2 = auth_service.authenticate(&user2);
    println!("  Token for user2: {}", token2);

    let verified = auth_service.verify(&token2, &user2);
    println!("  Verified: {}", verified);

    auth_service.logout(&token2);

    println!();

    // ========================================================================
    // PART 4: Module Privacy Demonstration
    // ========================================================================
    println!("--- Part 4: Module Privacy ---\n");

    println!("What we CAN access:");
    println!("  ✓ User::new() - public constructor");
    println!("  ✓ user.username() - public getter");
    println!("  ✓ authenticate() - public function");
    println!("  ✓ AuthToken::value() - public method");

    println!("\nWhat we CANNOT access (would cause compile errors):");
    println!("  ✗ User {{ username: ... }} - fields are private");
    println!("  ✗ utils::generate_random_string() - utils module is private");
    println!("  ✗ AuthToken::new() - constructor is pub(crate)");

    // Try to uncomment these to see errors:
    // let user = User { username: "test".to_string(), email: "test@example.com".to_string(), active: true };  // ❌ ERROR
    // let random = modules_crates::utils::generate_random_string(10);  // ❌ ERROR: utils is private
    // let token = AuthToken::new("token".to_string(), "user".to_string());  // ❌ ERROR: pub(crate)

    println!();

    // ========================================================================
    // PART 5: Different Import Styles
    // ========================================================================
    println!("--- Part 5: Different Import Styles ---\n");

    println!("1. Using prelude:");
    println!("   use modules_crates::prelude::*;");
    let u1 = User::new("user1".to_string(), "user1@example.com".to_string());
    println!("   Created: {}", u1.username());

    println!("\n2. Importing specific items:");
    println!("   use modules_crates::{{User, authenticate}};");
    let u2 = User::new("user2".to_string(), "user2@example.com".to_string());
    let t2 = authenticate(&u2);
    println!("   User: {}, Token: {}", u2.username(), t2);

    println!("\n3. Using full paths:");
    println!("   modules_crates::models::User::new(...)");
    let u3 = modules_crates::models::User::new(
        "user3".to_string(),
        "user3@example.com".to_string(),
    );
    println!("   Created: {}", u3.username());

    println!();

    // ========================================================================
    // PART 6: Real-World Example
    // ========================================================================
    println!("--- Part 6: Real-World Example ---\n");

    println!("Simulating user registration and authentication flow:");

    // Step 1: Create and validate user
    let username = "newuser".to_string();
    let email = "newuser@example.com".to_string();

    let user = match User::validated(username, email) {
        Ok(u) => {
            println!("  1. User validation: ✓");
            u
        }
        Err(e) => {
            eprintln!("  1. User validation: ✗ {}", e);
            return;
        }
    };

    // Step 2: Authenticate user
    println!("  2. Authenticating user...");
    let auth_service = services::AuthService::default();
    let token = auth_service.authenticate(&user);
    println!("     Token generated: {}", token);

    // Step 3: Verify token
    println!("  3. Verifying token...");
    if auth_service.verify(&token, &user) {
        println!("     ✓ Token valid - user authenticated");
    } else {
        println!("     ✗ Token invalid");
    }

    // Step 4: Perform authenticated actions
    println!("  4. User info:");
    println!("     Display name: {}", user.display_name());
    println!("     Active: {}", user.is_active());

    // Step 5: Logout
    println!("  5. Logging out...");
    auth_service.logout(&token);
    println!("     ✓ User logged out");

    println!();

    println!("=== Program Complete ===");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Binary crate (main.rs) uses library crate (lib.rs)
// 2. Can only access PUBLIC items from the library
// 3. Private items are completely hidden
// 4. Use prelude for convenient imports
// 5. Different import styles: prelude, specific, full path
// 6. Library provides clean, intentional API
// 7. Implementation details stay hidden
// 8. Binary demonstrates how to use the library
// 9. Separation of library (reusable) and binary (application)
// 10. This structure scales to large projects

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Trying to access private fields: user.username (use getter instead)
// ❌ Trying to import private modules: use mylib::utils (utils is private)
// ❌ Trying to use pub(crate) items from binary (only visible in lib)
// ❌ Not understanding lib.rs vs main.rs separation
// ❌ Forgetting to mark modules as pub in lib.rs
// ❌ Making everything pub instead of designing a clean API
// ❌ Not using re-exports for convenience
// ❌ Not creating a prelude module for common imports

// ============================================================================
// PROJECT STRUCTURE BEST PRACTICES
// ============================================================================
//
// Small Projects:
//   src/lib.rs       - Library root
//   src/main.rs      - Binary (if needed)
//   src/module1.rs
//   src/module2.rs
//
// Medium Projects:
//   src/lib.rs
//   src/main.rs
//   src/models/
//       mod.rs
//       user.rs
//       post.rs
//   src/services/
//       mod.rs
//       auth.rs
//
// Large Projects:
//   Cargo.toml       - Workspace root
//   core/            - Core library
//   api/             - API server
//   cli/             - CLI tool
//   common/          - Shared utilities
//
// Key Principles:
// - Separate library (reusable) from binary (application)
// - Use modules for logical grouping
// - Keep implementation private, expose minimal API
// - Use re-exports for convenience
// - Document public API
// - Test each module
