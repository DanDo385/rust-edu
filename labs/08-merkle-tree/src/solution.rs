//! # Merkle Tree - Complete Solution with Blockchain Context
//!
//! ## What is a Merkle Tree?
//!
//! A Merkle tree (hash tree) is a data structure where:
//! - Leaves = hashes of data (transactions in Bitcoin)
//! - Internal nodes = hash of their children
//! - Root = single hash representing all data
//!
//! ```text
//!         Root Hash
//!        /         \
//!      H(AB)       H(CD)
//!     /    \      /    \
//!   H(A)  H(B)  H(C)  H(D)
//!    |     |     |     |
//!    A     B     C     D    <- Actual data (transactions)
//! ```
//!
//! ## Blockchain Use
//!
//! Bitcoin blocks contain:
//! - Block header (80 bytes) with Merkle root
//! - Thousands of transactions
//!
//! Merkle root in header allows:
//! - Verify transaction without full block (SPV)
//! - Proof size: O(log n) instead of O(n)
//! - Efficient: only need log₂(n) hashes to prove inclusion
//!
//! NOTE: This implementation uses a simple hash function for educational purposes.
//! In production, use a proper cryptographic hash library like sha2.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher};

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

/// Merkle tree structure.
pub struct MerkleTree {
    pub root: String,      // Root hash (goes in block header)
    pub leaves: Vec<String>, // Leaf hashes (transaction hashes)
}

impl MerkleTree {
    /// Build a Merkle tree from data items.
    ///
    /// ## Blockchain Context
    /// When a Bitcoin miner creates a block:
    /// 1. Collect transactions to include
    /// 2. Hash each transaction (leaf hashes)
    /// 3. Build Merkle tree bottom-up
    /// 4. Put root hash in block header
    /// 5. Broadcast block (header + transactions)
    ///
    /// ## Algorithm
    /// 1. Hash each data item (create leaves)
    /// 2. While more than one hash remains:
    ///    - Pair up hashes
    ///    - Hash each pair to create parent
    ///    - If odd number, duplicate last hash
    /// 3. Final hash is root
    ///
    /// ## Parameters
    /// - `data`: Vec of strings to include (transactions)
    ///
    /// ## Returns
    /// MerkleTree with root and leaves
    pub fn new(data: Vec<String>) -> Self {
        if data.is_empty() {
            return MerkleTree {
                root: String::new(),
                leaves: Vec::new(),
            };
        }

        let mut current_level: Vec<Vec<u8>> = data
            .iter()
            .map(|item| hash_leaf(item))
            .collect();

        let leaves_as_bytes = current_level.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let left = &chunk[0];
                let right = if chunk.len() > 1 { &chunk[1] } else { left };
                let parent = hash_pair(left, right);
                next_level.push(parent);
            }
            current_level = next_level;
        }

        let root = bytes_to_hex(&current_level[0]);
        let leaves = leaves_as_bytes
            .iter()
            .map(|hash| bytes_to_hex(hash))
            .collect();

        MerkleTree { root, leaves }
    }

    /// Get the root hash.
    ///
    /// ## Blockchain Context
    /// This root hash goes in the Bitcoin block header.
    /// It represents ALL transactions in the block.
    /// Changing any transaction changes the root!
    pub fn root_hash(&self) -> &str {
        &self.root
    }
}

/// Hash a single leaf (transaction).
///
/// In Bitcoin, this is the transaction ID (txid).
fn hash_leaf(data: &str) -> Vec<u8> {
    simple_hash(data.as_bytes())
}

/// Hash a pair of nodes.
///
/// ## How it works
/// - Concatenate left and right hashes
/// - Hash the result
/// - This creates the parent node
///
/// ## Why this order matters
/// - Always left + right (not right + left)
/// - Ensures deterministic tree structure
/// - Bitcoin does: SHA256(SHA256(left || right))
pub fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut combined = Vec::with_capacity(left.len() + right.len());
    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    simple_hash(&combined)
}

// ============================================================================
// MERKLE PROOF VERIFICATION (Simplified)
// ============================================================================
//
// To verify a transaction is in a block without downloading the block:
//
// 1. Get Merkle proof from a full node:
//    - Transaction hash
//    - Sibling hashes along path to root
//    - Only log₂(n) hashes needed!
//
// 2. Verify proof:
//    - Start with transaction hash
//    - Combine with sibling at each level
//    - Hash each combination
//    - Check if final result matches root
//
// Example with 8 transactions:
// - Full block: 8 transactions (~2 KB each) = ~16 KB
// - Proof: log₂(8) = 3 sibling hashes (~96 bytes) + transaction
// - Savings: 99.4% reduction!
//
// This is how mobile Bitcoin wallets work (SPV wallets)
