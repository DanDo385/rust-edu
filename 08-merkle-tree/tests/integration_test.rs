//! Integration tests for merkle-tree

use merkle_tree::solution::*;

#[test]
fn test_single_item() {
    let tree = MerkleTree::new(vec!["A".to_string()]);
    assert!(!tree.root.is_empty());
}

#[test]
fn test_two_items() {
    let tree = MerkleTree::new(vec!["A".to_string(), "B".to_string()]);
    assert!(!tree.root.is_empty());
    assert_eq!(tree.leaves.len(), 2);
}

#[test]
fn test_four_items() {
    let data = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()];
    let tree = MerkleTree::new(data);
    assert!(!tree.root.is_empty());
    assert_eq!(tree.leaves.len(), 4);
}

#[test]
fn test_different_data_different_root() {
    let tree1 = MerkleTree::new(vec!["A".to_string(), "B".to_string()]);
    let tree2 = MerkleTree::new(vec!["A".to_string(), "C".to_string()]);
    assert_ne!(tree1.root, tree2.root);
}

#[test]
fn test_same_data_same_root() {
    let tree1 = MerkleTree::new(vec!["A".to_string(), "B".to_string()]);
    let tree2 = MerkleTree::new(vec!["A".to_string(), "B".to_string()]);
    assert_eq!(tree1.root, tree2.root);
}

#[test]
fn test_hash_pair() {
    let hash1 = hash_pair("a", "b");
    let hash2 = hash_pair("a", "b");
    assert_eq!(hash1, hash2);

    let hash3 = hash_pair("b", "a");
    assert_ne!(hash1, hash3); // Order matters
}
