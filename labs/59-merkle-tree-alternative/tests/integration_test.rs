//! Integration tests for Lab 59: Merkle Tree (Alternative)
//!
//! Tests verify tree construction, root computation, data integrity detection,
//! proof generation and verification, and edge cases.

use merkle_tree_alternative::*;

// ============================================================================
// HASH FUNCTION TESTS
// ============================================================================

#[test]
fn test_hash_string_deterministic() {
    let h1 = hash_string("hello");
    let h2 = hash_string("hello");
    assert_eq!(h1, h2);
}

#[test]
fn test_hash_string_different_inputs() {
    let h1 = hash_string("hello");
    let h2 = hash_string("world");
    assert_ne!(h1, h2);
}

#[test]
fn test_hash_bytes_deterministic() {
    let h1 = hash_bytes(b"test data");
    let h2 = hash_bytes(b"test data");
    assert_eq!(h1, h2);
}

#[test]
fn test_hash_string_is_hex() {
    let h = hash_string("hello");
    // All characters should be hex digits
    assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hash_string_consistent_length() {
    let h1 = hash_string("short");
    let h2 = hash_string("a much longer string that has many more characters");
    // Both should produce same-length hex output
    assert_eq!(h1.len(), h2.len());
}

#[test]
fn test_hash_pair_deterministic() {
    let a = hash_string("left");
    let b = hash_string("right");
    let p1 = hash_pair(&a, &b);
    let p2 = hash_pair(&a, &b);
    assert_eq!(p1, p2);
}

#[test]
fn test_hash_pair_order_matters() {
    let a = hash_string("left");
    let b = hash_string("right");
    let p1 = hash_pair(&a, &b);
    let p2 = hash_pair(&b, &a);
    assert_ne!(p1, p2);
}

// ============================================================================
// MERKLE TREE CONSTRUCTION TESTS
// ============================================================================

#[test]
fn test_tree_four_leaves() {
    let tree = MerkleTree::new(&["a", "b", "c", "d"]);
    assert_eq!(tree.leaf_count(), 4);
    assert!(!tree.root().is_empty());
    assert!(!tree.is_empty());
}

#[test]
fn test_tree_single_leaf() {
    let tree = MerkleTree::new(&["only one"]);
    assert_eq!(tree.leaf_count(), 1);
    assert!(!tree.root().is_empty());
    // Root should be the hash of the single item
    assert_eq!(tree.root(), hash_string("only one"));
}

#[test]
fn test_tree_two_leaves() {
    let tree = MerkleTree::new(&["a", "b"]);
    assert_eq!(tree.leaf_count(), 2);
    // Root = hash(hash("a") + hash("b"))
    let expected_root = hash_pair(&hash_string("a"), &hash_string("b"));
    assert_eq!(tree.root(), expected_root);
}

#[test]
fn test_tree_three_leaves_odd() {
    // Odd number of leaves: the last one is promoted
    let tree = MerkleTree::new(&["a", "b", "c"]);
    assert_eq!(tree.leaf_count(), 3);
    assert!(!tree.root().is_empty());
}

#[test]
fn test_tree_empty() {
    let tree = MerkleTree::new(&[]);
    assert!(tree.is_empty());
    assert_eq!(tree.leaf_count(), 0);
    assert_eq!(tree.root(), "");
}

#[test]
fn test_tree_node_count_four_leaves() {
    let tree = MerkleTree::new(&["a", "b", "c", "d"]);
    // 4 leaves + 2 internal + 1 root = 7 nodes
    assert_eq!(tree.node_count(), 7);
}

#[test]
fn test_tree_node_count_two_leaves() {
    let tree = MerkleTree::new(&["a", "b"]);
    // 2 leaves + 1 root = 3 nodes
    assert_eq!(tree.node_count(), 3);
}

#[test]
fn test_tree_node_count_single() {
    let tree = MerkleTree::new(&["a"]);
    // 1 leaf only (it IS the root)
    assert_eq!(tree.node_count(), 1);
}

#[test]
fn test_tree_leaves_match_input_hashes() {
    let data = ["tx1", "tx2", "tx3", "tx4"];
    let tree = MerkleTree::new(&data);
    let expected_leaves: Vec<String> = data.iter().map(|d| hash_string(d)).collect();
    assert_eq!(tree.leaves(), &expected_leaves[..]);
}

// ============================================================================
// DATA INTEGRITY TESTS
// ============================================================================

#[test]
fn test_data_integrity_same_data_same_root() {
    let tree1 = MerkleTree::new(&["tx1", "tx2", "tx3", "tx4"]);
    let tree2 = MerkleTree::new(&["tx1", "tx2", "tx3", "tx4"]);
    assert_eq!(tree1.root(), tree2.root());
}

#[test]
fn test_data_integrity_modified_data_different_root() {
    let original = MerkleTree::new(&["tx1", "tx2", "tx3", "tx4"]);
    let tampered = MerkleTree::new(&["tx1", "TAMPERED", "tx3", "tx4"]);
    assert_ne!(original.root(), tampered.root());
}

#[test]
fn test_data_integrity_reordered_data_different_root() {
    let tree1 = MerkleTree::new(&["a", "b", "c", "d"]);
    let tree2 = MerkleTree::new(&["b", "a", "c", "d"]);
    assert_ne!(tree1.root(), tree2.root());
}

#[test]
fn test_data_integrity_subset_different_root() {
    let tree1 = MerkleTree::new(&["a", "b", "c", "d"]);
    let tree2 = MerkleTree::new(&["a", "b", "c"]);
    assert_ne!(tree1.root(), tree2.root());
}

// ============================================================================
// PROOF GENERATION & VERIFICATION TESTS
// ============================================================================

#[test]
fn test_generate_proof_valid_index() {
    let tree = MerkleTree::new(&["a", "b", "c", "d"]);
    let proof = tree.generate_proof(0);
    assert!(proof.is_some());
    let proof = proof.unwrap();
    assert!(!proof.is_empty());
}

#[test]
fn test_generate_proof_out_of_bounds() {
    let tree = MerkleTree::new(&["a", "b", "c", "d"]);
    assert!(tree.generate_proof(4).is_none());
    assert!(tree.generate_proof(100).is_none());
}

#[test]
fn test_verify_proof_valid() {
    let data = ["tx1", "tx2", "tx3", "tx4"];
    let tree = MerkleTree::new(&data);

    // Verify proof for each leaf
    for (i, item) in data.iter().enumerate() {
        let proof = tree.generate_proof(i).unwrap();
        assert!(
            MerkleTree::verify_proof(tree.root(), item, &proof),
            "Proof verification failed for leaf {}",
            i
        );
    }
}

#[test]
fn test_verify_proof_invalid_data() {
    let tree = MerkleTree::new(&["tx1", "tx2", "tx3", "tx4"]);
    let proof = tree.generate_proof(0).unwrap();
    // Wrong data should fail verification
    assert!(!MerkleTree::verify_proof(tree.root(), "WRONG", &proof));
}

#[test]
fn test_verify_proof_wrong_root() {
    let tree = MerkleTree::new(&["tx1", "tx2", "tx3", "tx4"]);
    let proof = tree.generate_proof(0).unwrap();
    // Wrong root should fail verification
    assert!(!MerkleTree::verify_proof("deadbeef", "tx1", &proof));
}

#[test]
fn test_verify_proof_two_elements() {
    let data = ["left", "right"];
    let tree = MerkleTree::new(&data);

    let proof0 = tree.generate_proof(0).unwrap();
    assert!(MerkleTree::verify_proof(tree.root(), "left", &proof0));

    let proof1 = tree.generate_proof(1).unwrap();
    assert!(MerkleTree::verify_proof(tree.root(), "right", &proof1));
}

#[test]
fn test_verify_proof_three_elements_odd() {
    let data = ["a", "b", "c"];
    let tree = MerkleTree::new(&data);

    for (i, item) in data.iter().enumerate() {
        let proof = tree.generate_proof(i).unwrap();
        assert!(
            MerkleTree::verify_proof(tree.root(), item, &proof),
            "Proof verification failed for leaf {} ('{}')",
            i,
            item
        );
    }
}

// ============================================================================
// CLONE TESTS
// ============================================================================

#[test]
fn test_tree_clone() {
    let tree1 = MerkleTree::new(&["a", "b", "c", "d"]);
    let tree2 = tree1.clone();
    assert_eq!(tree1.root(), tree2.root());
    assert_eq!(tree1.leaf_count(), tree2.leaf_count());
}

// ============================================================================
// LARGE TREE TESTS
// ============================================================================

#[test]
fn test_tree_eight_leaves() {
    let data: Vec<&str> = (0..8).map(|i| match i {
        0 => "tx0", 1 => "tx1", 2 => "tx2", 3 => "tx3",
        4 => "tx4", 5 => "tx5", 6 => "tx6", _ => "tx7",
    }).collect();
    let tree = MerkleTree::new(&data);
    assert_eq!(tree.leaf_count(), 8);
    assert!(!tree.root().is_empty());

    // Verify all proofs
    for (i, item) in data.iter().enumerate() {
        let proof = tree.generate_proof(i).unwrap();
        assert!(MerkleTree::verify_proof(tree.root(), item, &proof));
    }
}
