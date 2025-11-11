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
        // Handle empty case
        if data.is_empty() {
            return MerkleTree {
                root: String::new(),
                leaves: Vec::new(),
            };
        }

        // ====================================================================
        // STEP 1: CREATE LEAF HASHES
        // ====================================================================

        // Hash each data item to create leaves
        // `.iter()` = iterate over data
        // `.map(|item| hash_leaf(item))` = hash each item
        // `.collect()` = collect into Vec<String>

        let mut current_level: Vec<String> = data
            .iter()
            .map(|item| hash_leaf(item))
            .collect();

        // Save leaf hashes (for proof generation later)
        let leaves = current_level.clone();

        // ====================================================================
        // STEP 2: BUILD TREE BOTTOM-UP
        // ====================================================================

        // Keep pairing and hashing until we have one hash (root)
        // `while current_level.len() > 1` = continue until single hash

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            // Process pairs of hashes
            // `.chunks(2)` = iterate in pairs
            //   - Returns chunks of 2 elements
            //   - Last chunk might have 1 element (odd number)

            for chunk in current_level.chunks(2) {
                // Get left hash (always exists)
                let left = &chunk[0];

                // Get right hash (duplicate left if odd number)
                // `chunk.get(1)` = try to get second element
                //   - Returns Option<&String>
                //   - Some(&right) if exists
                //   - None if chunk has only 1 element
                // `.unwrap_or(left)` = use left if no right

                let right = chunk.get(1).unwrap_or(left);

                // Hash the pair to create parent
                let parent = hash_pair(left, right);

                // Add parent to next level
                next_level.push(parent);
            }

            // Move up one level
            current_level = next_level;
        }

        // Root is the final remaining hash
        let root = current_level[0].clone();

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
fn hash_leaf(data: &str) -> String {
    let hash_bytes = simple_hash(data.as_bytes());
    bytes_to_hex(&hash_bytes)
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
pub fn hash_pair(left: &str, right: &str) -> String {
    // Concatenate hashes
    // `format!("{}{}", left, right)` = combine strings

    let combined = format!("{}{}", left, right);

    // Hash the combination
    let hash_bytes = simple_hash(combined.as_bytes());
    bytes_to_hex(&hash_bytes)
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
