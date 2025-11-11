//! Integration tests for simple-blockchain

use simple_blockchain::solution::*;

#[test]
fn test_create_blockchain() {
    let blockchain = Blockchain::new(2);
    assert_eq!(blockchain.chain.len(), 1);
    assert_eq!(blockchain.chain[0].index, 0);
}

#[test]
fn test_add_block() {
    let mut blockchain = Blockchain::new(1);
    blockchain.add_block("Test data".to_string());
    assert_eq!(blockchain.chain.len(), 2);
}

#[test]
fn test_valid_chain() {
    let mut blockchain = Blockchain::new(1);
    blockchain.add_block("Block 1".to_string());
    blockchain.add_block("Block 2".to_string());
    assert!(blockchain.is_valid());
}

#[test]
fn test_tampered_chain() {
    let mut blockchain = Blockchain::new(1);
    blockchain.add_block("Block 1".to_string());
    blockchain.add_block("Block 2".to_string());

    // Tamper with data
    blockchain.chain[1].data = "Tampered".to_string();

    // Chain should be invalid
    assert!(!blockchain.is_valid());
}

#[test]
fn test_block_hash_changes() {
    let mut block1 = Block::new(1, "Data 1".to_string(), "0".to_string());
    let hash1 = block1.calculate_hash();

    block1.data = "Data 2".to_string();
    let hash2 = block1.calculate_hash();

    assert_ne!(hash1, hash2);
}

#[test]
fn test_mining_produces_valid_hash() {
    let mut block = Block::new(1, "Test".to_string(), "0".to_string());
    block.mine(2);
    assert!(block.hash.starts_with("00"));
}
