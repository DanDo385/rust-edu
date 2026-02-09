//! # Lab 59: Merkle Tree (Alternative) - Student API
//!
//! Implement hashing and Merkle proof logic.
//! See `src/solution.rs` for reference.

pub fn hash_bytes(_data: &[u8]) -> String {
    todo!("Hash bytes to deterministic hex string")
}

pub fn hash_string(_data: &str) -> String {
    todo!("Hash UTF-8 string")
}

pub fn hash_pair(_left: &str, _right: &str) -> String {
    todo!("Hash concatenated child hashes")
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: String,
    leaves: Vec<String>,
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn new(_data: &[&str]) -> Self {
        todo!("Build Merkle tree bottom-up from leaves")
    }

    pub fn root(&self) -> &str {
        let _ = self;
        todo!("Return root hash")
    }

    pub fn leaves(&self) -> &[String] {
        let _ = self;
        todo!("Return leaf hashes")
    }

    pub fn node_count(&self) -> usize {
        let _ = self;
        todo!("Return total node count")
    }

    pub fn leaf_count(&self) -> usize {
        let _ = self;
        todo!("Return leaf count")
    }

    pub fn is_empty(&self) -> bool {
        let _ = self;
        todo!("Return true when tree has no leaves")
    }

    pub fn generate_proof(&self, _leaf_index: usize) -> Option<Vec<(String, bool)>> {
        let _ = self;
        todo!("Generate sibling path proof for leaf")
    }

    pub fn verify_proof(_root: &str, _data: &str, _proof: &[(String, bool)]) -> bool {
        todo!("Verify Merkle inclusion proof")
    }
}

#[doc(hidden)]
pub mod solution;
