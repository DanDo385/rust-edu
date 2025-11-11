//! Integration tests for ownership-borrowing project
//!
//! These tests verify that the solution works correctly.
//! Run with: cargo test -p ownership-borrowing

use ownership_borrowing::solution::*;

// ============================================================================
// TESTS FOR: add_exclamation
// ============================================================================

#[test]
fn test_add_exclamation_basic() {
    let s = String::from("Hello");
    let result = add_exclamation(s);
    assert_eq!(result, "Hello!");
}

#[test]
fn test_add_exclamation_already_has_content() {
    let s = String::from("Hello World");
    let result = add_exclamation(s);
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_add_exclamation_empty_string() {
    let s = String::from("");
    let result = add_exclamation(s);
    assert_eq!(result, "!");
}

#[test]
fn test_add_exclamation_single_char() {
    let s = String::from("H");
    let result = add_exclamation(s);
    assert_eq!(result, "H!");
}

#[test]
fn test_add_exclamation_with_unicode() {
    let s = String::from("Hello 世界");
    let result = add_exclamation(s);
    assert_eq!(result, "Hello 世界!");
}

#[test]
fn test_add_exclamation_ownership_transfer() {
    // Test that ownership is properly transferred
    let s = String::from("Test");
    let result = add_exclamation(s);
    // s is no longer valid here - can't use it
    assert_eq!(result, "Test!");
    // If we tried to use s here, it wouldn't compile
}

#[test]
fn test_add_exclamation_multiple_calls() {
    // Test chaining calls (each time ownership transfers)
    let s1 = String::from("Hello");
    let s2 = add_exclamation(s1);
    let s3 = add_exclamation(s2);
    assert_eq!(s3, "Hello!!");
}

// ============================================================================
// TESTS FOR: get_length
// ============================================================================

#[test]
fn test_get_length_basic() {
    let s = String::from("Hello");
    let len = get_length(&s);
    assert_eq!(len, 5);
    // s is still valid!
    assert_eq!(s, "Hello");
}

#[test]
fn test_get_length_empty() {
    let s = String::from("");
    let len = get_length(&s);
    assert_eq!(len, 0);
    assert_eq!(s, "");
}

#[test]
fn test_get_length_unicode() {
    // Note: len() returns BYTE count, not character count
    let s = String::from("Hello 世界");
    let len = get_length(&s);
    // "Hello " = 6 bytes, "世界" = 6 bytes (3 bytes per char)
    assert_eq!(len, 12);
}

#[test]
fn test_get_length_preserves_string() {
    let s = String::from("Test");
    let len1 = get_length(&s);
    let len2 = get_length(&s);
    let len3 = get_length(&s);
    // Can call multiple times, s is still valid
    assert_eq!(len1, 4);
    assert_eq!(len2, 4);
    assert_eq!(len3, 4);
    assert_eq!(s, "Test");
}

#[test]
fn test_get_length_long_string() {
    let s = String::from("This is a longer string with many words");
    let len = get_length(&s);
    assert_eq!(len, 39);  // Actual length of the string
}

// ============================================================================
// TESTS FOR: make_uppercase
// ============================================================================

#[test]
fn test_make_uppercase_basic() {
    let mut s = String::from("hello");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO");
}

#[test]
fn test_make_uppercase_already_uppercase() {
    let mut s = String::from("HELLO");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO");
}

#[test]
fn test_make_uppercase_mixed_case() {
    let mut s = String::from("HeLLo WoRLd");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO WORLD");
}

#[test]
fn test_make_uppercase_empty() {
    let mut s = String::from("");
    make_uppercase(&mut s);
    assert_eq!(s, "");
}

#[test]
fn test_make_uppercase_with_numbers() {
    let mut s = String::from("hello123");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO123");
}

#[test]
fn test_make_uppercase_with_punctuation() {
    let mut s = String::from("hello, world!");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO, WORLD!");
}

#[test]
fn test_make_uppercase_unicode() {
    let mut s = String::from("hello");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO");
}

#[test]
fn test_make_uppercase_german_eszett() {
    // German ß becomes SS when uppercased
    let mut s = String::from("straße");
    make_uppercase(&mut s);
    assert_eq!(s, "STRASSE");
}

#[test]
fn test_make_uppercase_multiple_calls() {
    let mut s = String::from("hello");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO");
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO"); // Unchanged
}

// ============================================================================
// TESTS FOR: demonstrate_copy_vs_move
// ============================================================================

#[test]
fn test_demonstrate_copy_vs_move_contains_key_words() {
    let result = demonstrate_copy_vs_move();

    // Check that the explanation contains key concepts
    assert!(result.contains("Copy"));
    assert!(result.contains("Move"));
    assert!(result.contains("clone"));
    assert!(result.contains("i32"));
    assert!(result.contains("String"));
}

#[test]
fn test_demonstrate_copy_vs_move_shows_values() {
    let result = demonstrate_copy_vs_move();

    // Check that it shows the example values
    assert!(result.contains("5")); // i32 value
    assert!(result.contains("hello")); // String value
    assert!(result.contains("world")); // Cloned String value
}

#[test]
fn test_demonstrate_copy_vs_move_explains_difference() {
    let result = demonstrate_copy_vs_move();

    // Check that it explains the key differences
    assert!(result.contains("Both") || result.contains("valid"));
    assert!(result.contains("INVALID") || result.contains("invalid"));
}

#[test]
fn test_demonstrate_copy_vs_move_returns_string() {
    let result = demonstrate_copy_vs_move();

    // Just verify it returns a non-empty String
    assert!(!result.is_empty());
    assert!(result.len() > 100); // Should be a detailed explanation
}

// ============================================================================
// INTEGRATION TESTS - Testing multiple concepts together
// ============================================================================

#[test]
fn test_integration_borrow_and_own() {
    // Create a string, borrow it, then take ownership
    let s = String::from("test");

    // Borrow immutably
    let len = get_length(&s);
    assert_eq!(len, 4);

    // Still valid, can take ownership
    let result = add_exclamation(s);
    assert_eq!(result, "test!");
}

#[test]
fn test_integration_multiple_borrows() {
    // Test multiple immutable borrows
    let s = String::from("hello");

    let len1 = get_length(&s);
    let len2 = get_length(&s);
    let len3 = get_length(&s);

    assert_eq!(len1, 5);
    assert_eq!(len2, 5);
    assert_eq!(len3, 5);

    // s is still valid
    assert_eq!(s, "hello");
}

#[test]
fn test_integration_mutable_borrow_then_immutable() {
    // Mutable borrow, then immutable borrows
    let mut s = String::from("hello");

    // Mutable borrow (modifies)
    make_uppercase(&mut s);
    assert_eq!(s, "HELLO");

    // Now immutable borrows (mutable borrow ended)
    let len = get_length(&s);
    assert_eq!(len, 5);
}

#[test]
fn test_integration_chain_operations() {
    // Chain multiple operations
    let mut s = String::from("rust");

    // Check initial length
    let len1 = get_length(&s);
    assert_eq!(len1, 4);

    // Make uppercase
    make_uppercase(&mut s);
    assert_eq!(s, "RUST");

    // Check length again
    let len2 = get_length(&s);
    assert_eq!(len2, 4);

    // Add exclamation (takes ownership)
    let result = add_exclamation(s);
    assert_eq!(result, "RUST!");

    // Check final length
    let len3 = get_length(&result);
    assert_eq!(len3, 5);
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_edge_case_very_long_string() {
    let mut s = "a".repeat(10000);
    make_uppercase(&mut s);
    assert_eq!(s.len(), 10000);
    assert!(s.chars().all(|c| c == 'A'));
}

#[test]
fn test_edge_case_special_characters() {
    let mut s = String::from("!@#$%^&*()");
    make_uppercase(&mut s);
    assert_eq!(s, "!@#$%^&*()"); // No change for special chars
}

#[test]
fn test_edge_case_whitespace_only() {
    let mut s = String::from("   \t\n");
    make_uppercase(&mut s);
    assert_eq!(s, "   \t\n"); // Whitespace unchanged
}

#[test]
fn test_edge_case_mixed_scripts() {
    let mut s = String::from("hello мир 世界");
    make_uppercase(&mut s);
    // English and Russian uppercase, Chinese unchanged
    assert!(s.contains("HELLO"));
    assert!(s.contains("МИР"));
    assert!(s.contains("世界"));
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

#[test]
fn test_property_exclamation_increases_length_by_one() {
    let test_strings = vec!["", "a", "hello", "test string", "🦀"];

    for s in test_strings {
        let original_len = s.len();
        let result = add_exclamation(String::from(s));
        assert_eq!(result.len(), original_len + 1);
    }
}

#[test]
fn test_property_uppercase_preserves_length_ascii() {
    // For ASCII strings, length should be preserved
    let test_strings = vec!["hello", "world", "RUST", "Test123", ""];

    for s in test_strings {
        let mut string = String::from(s);
        let original_len = string.len();
        make_uppercase(&mut string);
        assert_eq!(string.len(), original_len);
    }
}

#[test]
fn test_property_get_length_matches_string_len() {
    let test_strings = vec![
        String::from(""),
        String::from("a"),
        String::from("hello"),
        String::from("hello world"),
        String::from("🦀"),
    ];

    for s in test_strings {
        let len1 = get_length(&s);
        let len2 = s.len();
        assert_eq!(len1, len2);
    }
}

#[test]
fn test_property_idempotent_uppercase() {
    // Calling uppercase twice should give same result as once
    let test_strings = vec!["hello", "HeLLo", "HELLO", "test123"];

    for s in test_strings {
        let mut s1 = String::from(s);
        let mut s2 = String::from(s);

        make_uppercase(&mut s1);

        make_uppercase(&mut s2);
        make_uppercase(&mut s2); // Call twice

        assert_eq!(s1, s2);
    }
}
