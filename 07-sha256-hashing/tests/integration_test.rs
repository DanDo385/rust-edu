//! Integration tests for sha256-hashing

use sha256_hashing::solution::*;

#[test]
fn test_hash_string_deterministic() {
    let hash1 = hash_string("test");
    let hash2 = hash_string("test");
    assert_eq!(hash1, hash2);
}

#[test]
fn test_hash_string_different_inputs() {
    let hash1 = hash_string("test1");
    let hash2 = hash_string("test2");
    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_string_length() {
    let hash = hash_string("test");
    assert_eq!(hash.len(), 64); // 32 bytes × 2 hex digits
}

#[test]
fn test_hash_with_nonce_different() {
    let hash1 = hash_with_nonce("block", 0);
    let hash2 = hash_with_nonce("block", 1);
    assert_ne!(hash1, hash2);
}

#[test]
fn test_find_hash_with_prefix_single_zero() {
    let (nonce, hash) = find_hash_with_prefix("test", "0");
    assert!(hash.starts_with("0"));
    assert_eq!(hash, hash_with_nonce("test", nonce));
}

#[test]
fn test_find_hash_with_prefix_double_zero() {
    let (nonce, hash) = find_hash_with_prefix("test", "00");
    assert!(hash.starts_with("00"));
    assert_eq!(hash, hash_with_nonce("test", nonce));
}

#[test]
fn test_verify_hash_valid() {
    let input = "test data";
    let hash = hash_string(input);
    assert!(verify_hash(input, &hash));
}

#[test]
fn test_verify_hash_invalid() {
    let input = "test data";
    let hash = hash_string(input);
    assert!(!verify_hash("different data", &hash));
}

#[test]
fn test_verify_hash_wrong_hash() {
    let input = "test data";
    let wrong_hash = "0".repeat(64);
    assert!(!verify_hash(input, &wrong_hash));
}
