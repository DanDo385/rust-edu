//! Integration tests for Lab 18: Modules & Crates
//!
//! These tests demonstrate how EXTERNAL USERS would use the library.
//! Note that:
//! - Only PUBLIC items are accessible
//! - Private modules (like utils) are invisible
//! - We use pub use re-exports for cleaner imports

use modules_crates::models::User;
use modules_crates::services::auth::authenticate;
use modules_crates::prelude::*;

// ============================================================================
// BASIC MODULE ACCESS TESTS
// ============================================================================

#[test]
fn test_user_creation_from_models_module() {
    // Demonstrates accessing User through models module
    let user = modules_crates::models::User::new(
        "alice".to_string(),
        "alice@example.com".to_string(),
    );
    assert_eq!(user.username(), "alice");
}

#[test]
fn test_user_creation_via_reexport() {
    // Demonstrates using re-exported User at crate root
    let user = User::new("bob".to_string(), "bob@example.com".to_string());
    assert_eq!(user.username(), "bob");
    assert_eq!(user.email(), "bob@example.com");
}

#[test]
fn test_user_creation_via_prelude() {
    // Demonstrates prelude import (most convenient)
    let user = User::new("charlie".to_string(), "charlie@example.com".to_string());
    assert!(user.is_active());
}

// ============================================================================
// USER MODEL TESTS
// ============================================================================

#[test]
fn test_user_display_name() {
    let user = User::new("dave".to_string(), "dave@example.com".to_string());
    assert_eq!(user.display_name(), "dave <dave@example.com>");
}

#[test]
fn test_user_matches_username() {
    let user = User::new("eve".to_string(), "eve@example.com".to_string());
    assert!(user.matches_username("eve"));
    assert!(!user.matches_username("frank"));
}

#[test]
fn test_user_deactivation_state() {
    let mut user = User::new("frank".to_string(), "frank@example.com".to_string());

    // Initially active
    assert!(user.is_active());

    // Deactivate
    user.deactivate();
    assert!(!user.is_active());

    // Reactivate
    user.activate();
    assert!(user.is_active());
}

#[test]
fn test_user_email_update_valid() {
    let mut user = User::new("grace".to_string(), "grace@example.com".to_string());

    // Valid email update
    let result = user.set_email("newemail@example.com".to_string());
    assert!(result.is_ok());
    assert_eq!(user.email(), "newemail@example.com");
}

#[test]
fn test_user_email_update_invalid() {
    let mut user = User::new("henry".to_string(), "henry@example.com".to_string());

    // Invalid email (no @)
    let result = user.set_email("invalid-email".to_string());
    assert!(result.is_err());

    // Email should remain unchanged
    assert_eq!(user.email(), "henry@example.com");
}

#[test]
fn test_user_validation_success() {
    let user = User::validated("iris".to_string(), "iris@example.com".to_string());
    assert!(user.is_ok());

    let user = user.unwrap();
    assert_eq!(user.username(), "iris");
}

#[test]
fn test_user_validation_username_too_short() {
    let user = User::validated("ab".to_string(), "ab@example.com".to_string());
    assert!(user.is_err());
}

#[test]
fn test_user_validation_username_too_long() {
    let long_name = "a".repeat(21);
    let user = User::validated(long_name, "test@example.com".to_string());
    assert!(user.is_err());
}

#[test]
fn test_user_validation_invalid_email() {
    let user = User::validated("jack".to_string(), "invalid-email".to_string());
    assert!(user.is_err());
}

#[test]
fn test_user_validation_valid_lengths() {
    // Minimum valid username (3 chars)
    let user = User::validated("abc".to_string(), "a@b.c".to_string());
    assert!(user.is_ok());

    // Maximum valid username (20 chars)
    let max_name = "a".repeat(20);
    let user = User::validated(max_name, "test@example.com".to_string());
    assert!(user.is_ok());
}

#[test]
fn test_user_clone_and_equality() {
    let user1 = User::new("kevin".to_string(), "kevin@example.com".to_string());
    let user2 = user1.clone();

    assert_eq!(user1, user2);
}

#[test]
fn test_user_debug_output() {
    let user = User::new("laura".to_string(), "laura@example.com".to_string());
    let debug_str = format!("{:?}", user);
    assert!(debug_str.contains("laura"));
}

#[test]
fn test_user_display_format() {
    let user = User::new("mike".to_string(), "mike@example.com".to_string());
    let display_str = format!("{}", user);
    assert!(display_str.contains("mike"));
    assert!(display_str.contains("mike@example.com"));
}

// ============================================================================
// AUTHENTICATION SERVICE TESTS
// ============================================================================

#[test]
fn test_authenticate_generates_token() {
    let user = User::new("nancy".to_string(), "nancy@example.com".to_string());
    let token = authenticate(&user);

    assert!(token.is_valid());
    assert_eq!(token.user_id(), "nancy");
}

#[test]
fn test_auth_token_accessors() {
    let user = User::new("oscar".to_string(), "oscar@example.com".to_string());
    let token = authenticate(&user);

    // Token has methods to access value and user_id
    let _value = token.value();
    let user_id = token.user_id();
    assert_eq!(user_id, "oscar");
}

#[test]
fn test_auth_token_validity() {
    let user = User::new("paul".to_string(), "paul@example.com".to_string());
    let token = authenticate(&user);

    assert!(token.is_valid());
}

#[test]
fn test_auth_service_creation() {
    let service = modules_crates::services::auth::AuthService::new(
        "MyAuthService".to_string(),
    );
    let user = User::new("quinn".to_string(), "quinn@example.com".to_string());

    let token = service.authenticate(&user);
    assert!(token.is_valid());
}

#[test]
fn test_auth_service_default() {
    let service = modules_crates::services::auth::AuthService::default();
    let user = User::new("rachel".to_string(), "rachel@example.com".to_string());

    let token = service.authenticate(&user);
    assert!(token.is_valid());
}

#[test]
fn test_auth_service_verify_token() {
    let service = modules_crates::services::auth::AuthService::default();
    let user = User::new("sam".to_string(), "sam@example.com".to_string());

    let token = service.authenticate(&user);
    assert!(service.verify(&token, &user));
}

#[test]
fn test_auth_service_verify_wrong_user() {
    let service = modules_crates::services::auth::AuthService::default();
    let user1 = User::new("tom".to_string(), "tom@example.com".to_string());
    let user2 = User::new("uma".to_string(), "uma@example.com".to_string());

    let token = service.authenticate(&user1);
    // Token is for tom, but we're verifying with uma
    assert!(!service.verify(&token, &user2));
}

#[test]
fn test_auth_service_logout() {
    let service = modules_crates::services::auth::AuthService::default();
    let user = User::new("victor".to_string(), "victor@example.com".to_string());

    let token = service.authenticate(&user);
    service.logout(&token);  // Should not panic
}

#[test]
fn test_auth_token_clone() {
    let user = User::new("wendy".to_string(), "wendy@example.com".to_string());
    let token1 = authenticate(&user);
    let token2 = token1.clone();

    assert_eq!(token1.value(), token2.value());
    assert_eq!(token1.user_id(), token2.user_id());
}

#[test]
fn test_auth_token_display() {
    let user = User::new("xavier".to_string(), "xavier@example.com".to_string());
    let token = authenticate(&user);
    let display_str = format!("{}", token);
    assert!(display_str.contains("AuthToken"));
}

// ============================================================================
// RE-EXPORT AND PRELUDE TESTS
// ============================================================================

#[test]
fn test_user_accessible_via_root_reexport() {
    // This works because lib.rs does: pub use models::user::User;
    let user = modules_crates::User::new(
        "yara".to_string(),
        "yara@example.com".to_string(),
    );
    assert_eq!(user.username(), "yara");
}

#[test]
fn test_auth_token_accessible_via_root_reexport() {
    // This works because lib.rs does: pub use services::auth::AuthToken;
    let user = User::new("zack".to_string(), "zack@example.com".to_string());
    let token = modules_crates::authenticate(&user);
    let _: modules_crates::AuthToken = token;  // Type annotation verifies accessible
}

#[test]
fn test_prelude_imports_user() {
    use modules_crates::prelude::*;

    let user = User::new("alex".to_string(), "alex@example.com".to_string());
    assert_eq!(user.username(), "alex");
}

#[test]
fn test_prelude_imports_auth() {
    use modules_crates::prelude::*;

    let user = User::new("ben".to_string(), "ben@example.com".to_string());
    let token = authenticate(&user);
    assert!(token.is_valid());
}

// ============================================================================
// CRATE-LEVEL FUNCTION TESTS
// ============================================================================

#[test]
fn test_init_function() {
    modules_crates::init("TestApp v1.0");
    // Should not panic
}

#[test]
fn test_version_function() {
    let v = modules_crates::version();
    assert!(!v.is_empty());
    // Version should be "0.1.0" as defined in Cargo.toml
    assert_eq!(v, "0.1.0");
}

// ============================================================================
// MODULE VISIBILITY TESTS
// ============================================================================

#[test]
fn test_models_module_is_public() {
    // We can access the models module
    let _: &str = "this proves models module is accessible";
    modules_crates::models::User::new("a".to_string(), "a@b".to_string());
}

#[test]
fn test_services_module_is_public() {
    // We can access the services module
    let _: &str = "this proves services module is accessible";
    modules_crates::services::AuthService::default();
}

#[test]
fn test_private_utils_module_not_accessible() {
    // This test verifies that utils module is NOT accessible
    // The following would not compile:
    // modules_crates::utils::generate_random_string(16);

    // Instead, we verify it's used internally by services
    let user = User::new("cat".to_string(), "cat@example.com".to_string());
    let token = authenticate(&user);

    // The token was generated by utils::generate_random_string (internal)
    assert!(token.value().len() > 0);
}

// ============================================================================
// INTEGRATION WORKFLOW TESTS
// ============================================================================

#[test]
fn test_complete_auth_workflow() {
    // This is a realistic workflow using multiple public APIs

    // Step 1: Create a user
    let user = User::validated(
        "david".to_string(),
        "david@example.com".to_string(),
    ).expect("valid user");

    // Step 2: Authenticate
    let service = modules_crates::services::auth::AuthService::new(
        "MyService".to_string(),
    );
    let token = service.authenticate(&user);

    // Step 3: Verify token
    assert!(service.verify(&token, &user));

    // Step 4: Update user email
    let mut user = user;
    user.set_email("newemail@example.com".to_string()).expect("valid email");
    assert_eq!(user.email(), "newemail@example.com");

    // Step 5: Logout
    service.logout(&token);
}

#[test]
fn test_multiple_users_same_service() {
    let service = modules_crates::services::auth::AuthService::default();

    let user1 = User::new("eve".to_string(), "eve@example.com".to_string());
    let user2 = User::new("frank".to_string(), "frank@example.com".to_string());

    let token1 = service.authenticate(&user1);
    let token2 = service.authenticate(&user2);

    // Each token is valid for its user
    assert!(service.verify(&token1, &user1));
    assert!(service.verify(&token2, &user2));

    // But not for the other
    assert!(!service.verify(&token1, &user2));
    assert!(!service.verify(&token2, &user1));
}

#[test]
fn test_user_state_modifications() {
    let mut user = User::new("grace".to_string(), "grace@example.com".to_string());

    // Initial state
    assert_eq!(user.username(), "grace");
    assert!(user.is_active());

    // Modify email
    user.set_email("grace2@example.com".to_string()).unwrap();
    assert_eq!(user.email(), "grace2@example.com");

    // Deactivate
    user.deactivate();
    assert!(!user.is_active());

    // Still accessible after modifications
    assert_eq!(user.username(), "grace");
    assert_eq!(user.email(), "grace2@example.com");
}

#[test]
fn test_error_handling_with_validation() {
    // Invalid username (too short)
    let result = User::validated(
        "ab".to_string(),
        "test@example.com".to_string(),
    );
    assert!(result.is_err());

    // Error message should be descriptive
    match result {
        Err(e) => {
            let msg = format!("{}", e);
            assert!(msg.contains("Username"));
        }
        _ => panic!("Expected error"),
    }
}

#[test]
fn test_chaining_operations() {
    // Demonstrates that we can chain operations using the public API
    let user = User::validated(
        "helen".to_string(),
        "helen@example.com".to_string(),
    ).expect("valid");

    let token = authenticate(&user);
    let display = user.display_name();

    assert!(token.is_valid());
    assert!(display.contains("helen"));
}

// ============================================================================
// PRIVACY BOUNDARY TESTS
// ============================================================================

#[test]
fn test_user_fields_are_private() {
    let user = User::new("ivan".to_string(), "ivan@example.com".to_string());

    // We cannot access user.username directly (it's private)
    // We must use the public getter method
    let username = user.username();
    assert_eq!(username, "ivan");
}

#[test]
fn test_auth_token_value_is_private() {
    let user = User::new("julia".to_string(), "julia@example.com".to_string());
    let token = authenticate(&user);

    // We cannot create AuthToken directly (new is pub(crate))
    // We must use authenticate() function

    // We cannot modify token.value directly
    // We can only read it via public getter
    let _value = token.value();
}

#[test]
fn test_validation_error_is_public() {
    // ValidationError is part of public API
    let result = User::validated(
        "xy".to_string(),
        "test@example.com".to_string(),
    );

    if let Err(_e) = result {
        // We can destructure and work with ValidationError
        // This proves it's part of the public API
    }
}
