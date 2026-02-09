//! Integration tests for Lab 15: Lifetimes & Borrow Checker

use lifetimes_borrow_checker::solution::*;

#[test]
fn test_longest_basic() {
    let s1 = "short";
    let s2 = "much longer";
    assert_eq!(longest(s1, s2), "much longer");
}

#[test]
fn test_longest_equal_length() {
    // When equal length, function returns second argument (else branch)
    assert_eq!(longest("hello", "world"), "world");
}

#[test]
fn test_longest_with_owned_strings() {
    let s1 = String::from("Rust");
    let s2 = String::from("Programming");
    // Convert String to &str for the function
    assert_eq!(longest(&s1, &s2), "Programming");
}

#[test]
fn test_text_metadata_creation() {
    let text = "Rust lifetimes";
    let meta = TextMetadata::new(text, 14);
    assert_eq!(meta.text(), "Rust lifetimes");
    assert_eq!(meta.count(), 14);
}

#[test]
fn test_text_metadata_debug() {
    let meta = TextMetadata::new("test", 4);
    let debug_str = format!("{:?}", meta);
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("4"));
}

#[test]
fn test_combine_creates_owned_string() {
    let result = combine("Hello", " Rust");
    assert_eq!(result, "Hello Rust");
    assert!(result.capacity() >= 11); // It's an owned String with capacity
}

#[test]
fn test_combine_empty_strings() {
    assert_eq!(combine("", ""), "");
}

#[test]
fn test_first_element_with_items() {
    let s1 = "first";
    let s2 = "second";
    let s3 = "third";
    let items = vec![s1, s2, s3];
    assert_eq!(first_element(&items), Some("first"));
}

#[test]
fn test_first_element_empty() {
    let items: Vec<&str> = vec![];
    assert_eq!(first_element(&items), None);
}

#[test]
fn test_validate_refs_same_length() {
    assert!(validate_refs("hello", "world")); // Both 5 chars
}

#[test]
fn test_validate_refs_different_length() {
    assert!(!validate_refs("hi", "hello")); // 2 vs 5 chars
}

#[test]
fn test_describe_text() {
    let meta = TextMetadata::new("Rust", 4);
    let desc = describe_text(&meta);
    assert!(desc.contains("Rust"));
    assert!(desc.contains("4"));
}

#[test]
fn test_lifetime_borrow_safety() {
    // This test demonstrates that lifetimes prevent use-after-free bugs
    let text = "borrowed text";
    let meta = TextMetadata::new(text, 13);
    // meta can't outlive text - enforced by Rust's type system
    assert_eq!(meta.text().len(), text.len());
}
