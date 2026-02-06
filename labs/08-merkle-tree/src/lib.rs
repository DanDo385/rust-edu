//! # Merkle Tree for Blockchain
//!
//! NOTE: This implementation uses a simple hash function for educational purposes.
//! In production, use a proper cryptographic hash library like sha2.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Simple hash function using Rust's standard library hasher.
/// This is NOT cryptographically secure - use only for learning!
fn simple_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    hasher.write(input);
    let hash_value = hasher.finish();

    // Create a 32-byte (256-bit) hash by repeating and mixing the 64-bit hash
    let mut result = Vec::with_capacity(32);
    for i in 0..4 {
        let shifted = hash_value.wrapping_mul(i as u64 + 1);
        result.extend_from_slice(&shifted.to_be_bytes());
    }
    result
}

/// Convert bytes to hexadecimal string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub struct MerkleTree {
    pub root: String,
    pub leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new(data: Vec<String>) -> Self {
        todo!()
    }

    pub fn root_hash(&self) -> &str {
        todo!()
    }
}

pub fn hash_pair(left: &str, right: &str) -> String {
    todo!()
}

#[doc(hidden)]
pub mod solution;
