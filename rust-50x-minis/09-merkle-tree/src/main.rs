// Project 09: Merkle Tree
//
// A Merkle tree is a binary tree of hashes. It's used in blockchains
// to efficiently verify that data belongs to a large set.
//
// IMPORTANT: Add these to Cargo.toml:
// [dependencies]
// sha2 = "0.10"
// hex = "0.4"

use sha2::{Sha256, Digest};

fn main() {
    println!("=== Merkle Tree ===\n");

    // ============================================================================
    // WHAT IS A MERKLE TREE?
    // ============================================================================
    // A Merkle tree is a binary tree where:
    // - Leaves are hashes of data blocks
    // - Internal nodes are hashes of their children
    // - The root is a hash of the entire dataset
    //
    // Example:
    //         ROOT (H(H01 + H23))
    //        /                   \
    //    H01 (H(H0 + H1))    H23 (H(H2 + H3))
    //    /      \              /       \
    //  H0       H1            H2       H3
    // (data)  (data)        (data)   (data)

    let data = vec![
        "Transaction 1",
        "Transaction 2",
        "Transaction 3",
        "Transaction 4",
    ];

    let tree = MerkleTree::new(data.clone());
    println!("Data: {:?}", data);
    println!("Merkle Root: {}", tree.root());

    // If we change any data, the root changes
    let modified_data = vec![
        "Transaction 1 MODIFIED",
        "Transaction 2",
        "Transaction 3",
        "Transaction 4",
    ];

    let modified_tree = MerkleTree::new(modified_data);
    println!("Modified Merkle Root: {}", modified_tree.root());

    println!();

    // ============================================================================
    // DEMONSTRATING DATA INTEGRITY
    // ============================================================================

    println!("=== Data Integrity ===");

    let original = vec!["Block 1", "Block 2", "Block 3", "Block 4"];
    let tree1 = MerkleTree::new(original.clone());

    let tampered = vec!["Block 1", "Block 2 TAMPERED", "Block 3", "Block 4"];
    let tree2 = MerkleTree::new(tampered);

    println!("Original root: {}", tree1.root());
    println!("Tampered root: {}", tree2.root());
    println!("Roots match? {}", tree1.root() == tree2.root());

    println!();

    // ============================================================================
    // MERKLE TREE IMPLEMENTATION
    // ============================================================================
}

// ============================================================================
// MERKLE TREE STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
struct MerkleTree {
    root: String,
    nodes: Vec<String>,
}

impl MerkleTree {
    /// Creates a new Merkle tree from data
    fn new(data: Vec<&str>) -> Self {
        if data.is_empty() {
            return MerkleTree {
                root: String::new(),
                nodes: vec![],
            };
        }

        // Step 1: Hash all the data (leaf nodes)
        let mut nodes: Vec<String> = data
            .iter()
            .map(|d| hash(d.as_bytes()))
            .collect();

        // Step 2: Build the tree from bottom to top
        // Keep combining pairs of nodes until we have one root
        let mut current_level = nodes.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            // Process pairs
            for i in (0..current_level.len()).step_by(2) {
                if i + 1 < current_level.len() {
                    // Combine two nodes
                    let combined = format!("{}{}", current_level[i], current_level[i + 1]);
                    let parent_hash = hash(combined.as_bytes());
                    next_level.push(parent_hash.clone());
                    nodes.push(parent_hash);
                } else {
                    // Odd number of nodes - promote the last one
                    let parent_hash = current_level[i].clone();
                    next_level.push(parent_hash.clone());
                    nodes.push(parent_hash);
                }
            }

            current_level = next_level;
        }

        MerkleTree {
            root: current_level[0].clone(),
            nodes,
        }
    }

    /// Returns the Merkle root
    fn root(&self) -> &str {
        &self.root
    }

    /// Returns all nodes in the tree
    fn nodes(&self) -> &[String] {
        &self.nodes
    }
}

// ============================================================================
// HASHING FUNCTION
// ============================================================================

/// Computes SHA-256 hash of data
fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // Convert to hex string
    result.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. VECTOR GROWTH
//    Vec<String> starts with capacity 0 and grows as needed.
//    Each growth reallocates and copies (typically doubles capacity).
//    We could optimize with Vec::with_capacity().
//
// 2. STRING CLONING
//    .clone() makes a deep copy of the String (allocates new heap memory).
//    This is expensive! In production, we'd use references or Rc<String>.
//
// 3. SHA-256 HASHING
//    The sha2 crate provides a safe, fast implementation of SHA-256.
//    It's written in pure Rust (no C dependencies).
//    The compiler optimizes it heavily (often as fast as C).
//
// 4. MEMORY LAYOUT
//    - Vec<String> on stack: pointer, length, capacity
//    - Each String on heap: owned byte array
//    - Total memory: O(n log n) for n leaf nodes
//
// 5. PERFORMANCE
//    - Time complexity: O(n) to build tree
//    - Space complexity: O(n) for leaves + O(n) for internal nodes = O(n)
//    - Verification: O(log n) with Merkle proof (not shown here)

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Merkle trees provide efficient data verification
// 2. Root hash represents the entire dataset
// 3. Changing any leaf changes the root
// 4. Used in Bitcoin, Ethereum, Git, IPFS
// 5. SHA-256 provides cryptographic security
// 6. Tree structure enables O(log n) proofs
// 7. Rust's ownership ensures we don't leak memory
// 8. Vec and String handle dynamic sizing automatically

// ============================================================================
// WHY THIS MATTERS FOR BLOCKCHAIN
// ============================================================================
// In blockchain:
// - Each block contains a Merkle root of all transactions
// - Light clients can verify a transaction is in a block
//   by downloading only O(log n) hashes, not all transactions
// - Any tampering changes the Merkle root (and thus block hash)
// - This makes blockchain immutable and verifiable

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Use Rc<String> or &str to avoid cloning
// 2. Implement Merkle proof generation and verification
// 3. Support arbitrary data types with generic T: Hash
// 4. Add tests for edge cases (empty tree, single element, odd number)
// 5. Use iterative algorithm instead of recursive (avoid stack overflow)
// 6. Cache intermediate results for faster queries

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to handle odd number of nodes
// ❌ Using unsafe cryptographic hash (MD5, SHA-1)
// ❌ Not cloning data (ownership errors)
// ❌ Off-by-one errors in tree building
// ❌ Not validating input (empty data)
