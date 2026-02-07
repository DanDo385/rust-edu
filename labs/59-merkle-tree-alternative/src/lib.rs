//! # Lab 59: Merkle Tree (Alternative)
//!
//! Alternative implementation of a Merkle tree data structure used in blockchains.
//! Uses a simple hash function from std (NOT cryptographically secure) for
//! educational purposes, keeping the workspace dependency-free.
//!
//! ## Ownership Commentary
//! - `MerkleTree` owns all its node data via `Vec<String>`
//! - Cloning is used when building tree levels (each level references the previous)
//! - `&str` returns are borrowed from the struct's owned data
//!
//! ## What is a Merkle Tree?
//! A binary tree where leaf nodes are hashes of data blocks, and internal nodes
//! are hashes of their children. The root hash represents the entire dataset.
//! Changing any leaf changes the root -- this provides tamper detection.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

// ============================================================================
// HASHING UTILITIES
// ============================================================================

/// Computes a simple hash of arbitrary bytes, returning a hex string.
///
/// # Memory Model
/// `DefaultHasher` is stack-allocated (no heap). The result is a u64 (8 bytes)
/// which we extend to a 32-byte hash for pedagogical similarity to SHA-256.
/// The returned String is heap-allocated and owned by the caller.
///
/// NOTE: This is NOT cryptographically secure. In production, use sha2 crate.
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    let hash_value = hasher.finish();

    // Create a 32-byte hash by mixing the 64-bit value
    let mut result = Vec::with_capacity(32);
    for i in 0u64..4 {
        let shifted = hash_value.wrapping_mul(i + 1);
        result.extend_from_slice(&shifted.to_be_bytes());
    }

    bytes_to_hex(&result)
}

/// Hashes a string (convenience wrapper).
pub fn hash_string(data: &str) -> String {
    hash_bytes(data.as_bytes())
}

/// Converts a byte slice to a hexadecimal string.
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Hashes two hash strings together (combines left and right child hashes).
pub fn hash_pair(left: &str, right: &str) -> String {
    let combined = format!("{}{}", left, right);
    hash_bytes(combined.as_bytes())
}

// ============================================================================
// MERKLE TREE
// ============================================================================

/// A Merkle tree that stores hashes at each level.
///
/// # Memory Model
/// - `root`: Owned String on the heap (the root hash)
/// - `leaves`: Vec<String> owning all leaf hashes
/// - `nodes`: Vec<String> owning ALL nodes (leaves + internal + root)
///
/// When the MerkleTree is dropped, all Strings and Vecs are freed automatically.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: String,
    leaves: Vec<String>,
    nodes: Vec<String>,
}

impl MerkleTree {
    /// Creates a new Merkle tree from a slice of string data.
    ///
    /// # Algorithm
    /// 1. Hash each data item to create leaf nodes
    /// 2. Pair adjacent nodes and hash them together
    /// 3. Repeat until only one node remains (the root)
    /// 4. If a level has an odd number of nodes, the last node is promoted
    pub fn new(data: &[&str]) -> Self {
        if data.is_empty() {
            return MerkleTree {
                root: String::new(),
                leaves: vec![],
                nodes: vec![],
            };
        }

        // Step 1: Hash all data items (leaf nodes)
        let leaves: Vec<String> = data.iter().map(|d| hash_string(d)).collect();
        let mut nodes = leaves.clone();

        // Step 2: Build tree bottom-up
        let mut current_level = leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for i in (0..current_level.len()).step_by(2) {
                if i + 1 < current_level.len() {
                    // Hash pair of nodes
                    let parent = hash_pair(&current_level[i], &current_level[i + 1]);
                    next_level.push(parent.clone());
                    nodes.push(parent);
                } else {
                    // Odd node: promote (duplicate) it
                    let promoted = current_level[i].clone();
                    next_level.push(promoted.clone());
                    nodes.push(promoted);
                }
            }

            current_level = next_level;
        }

        MerkleTree {
            root: current_level[0].clone(),
            leaves,
            nodes,
        }
    }

    /// Returns the Merkle root hash.
    pub fn root(&self) -> &str {
        &self.root
    }

    /// Returns all leaf hashes.
    pub fn leaves(&self) -> &[String] {
        &self.leaves
    }

    /// Returns the total number of nodes (leaves + internal).
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of leaf nodes.
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    /// Returns true if the tree is empty (no data).
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Generates a Merkle proof (list of sibling hashes) for the leaf at the given index.
    /// Returns None if the index is out of bounds.
    ///
    /// # Teaching Note
    /// A Merkle proof allows verification that a specific data item is part of the tree
    /// without needing all the data -- only O(log n) hashes are required.
    pub fn generate_proof(&self, leaf_index: usize) -> Option<Vec<(String, bool)>> {
        if leaf_index >= self.leaves.len() {
            return None;
        }

        let mut proof = Vec::new();
        let mut current_level: Vec<String> = self.leaves.clone();
        let mut index = leaf_index;

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for i in (0..current_level.len()).step_by(2) {
                if i + 1 < current_level.len() {
                    let parent = hash_pair(&current_level[i], &current_level[i + 1]);
                    next_level.push(parent);

                    // If this pair contains our index, record the sibling
                    if i == index {
                        // Sibling is on the right
                        proof.push((current_level[i + 1].clone(), true));
                    } else if i + 1 == index {
                        // Sibling is on the left
                        proof.push((current_level[i].clone(), false));
                    }
                } else {
                    // Odd node: no sibling to record
                    next_level.push(current_level[i].clone());
                }
            }

            index /= 2;
            current_level = next_level;
        }

        Some(proof)
    }

    /// Verifies a Merkle proof for a given data item.
    /// Returns true if the proof is valid (the data belongs to the tree).
    pub fn verify_proof(root: &str, data: &str, proof: &[(String, bool)]) -> bool {
        let mut current_hash = hash_string(data);

        for (sibling_hash, is_right) in proof {
            if *is_right {
                current_hash = hash_pair(&current_hash, sibling_hash);
            } else {
                current_hash = hash_pair(sibling_hash, &current_hash);
            }
        }

        current_hash == root
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let h1 = hash_string("hello");
        let h2 = hash_string("hello");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_different_inputs() {
        let h1 = hash_string("hello");
        let h2 = hash_string("world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_tree_root_exists() {
        let tree = MerkleTree::new(&["a", "b", "c", "d"]);
        assert!(!tree.root().is_empty());
    }
}
