// Integration tests for Lab 48: Proof of Work
//
// Tests SHA-256 hashing, block creation, mining, validation,
// difficulty checking, and blockchain integrity.

use proof_of_work::solution::*;

// ============================================================================
// SHA-256 HASH UTILITY TESTS
// ============================================================================

#[test]
fn test_sha256_hex_basic() {
    let hash = sha256_hex(b"hello");
    // Known SHA-256 hash of "hello"
    assert_eq!(
        hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha256_hex_empty() {
    let hash = sha256_hex(b"");
    // Known SHA-256 hash of empty string
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_sha256_hex_deterministic() {
    let hash1 = sha256_hex(b"test data");
    let hash2 = sha256_hex(b"test data");
    assert_eq!(hash1, hash2);
}

#[test]
fn test_sha256_hex_different_inputs() {
    let hash1 = sha256_hex(b"hello");
    let hash2 = sha256_hex(b"world");
    assert_ne!(hash1, hash2);
}

#[test]
fn test_sha256_hex_length() {
    let hash = sha256_hex(b"anything");
    // SHA-256 produces 64 hex characters (256 bits / 4 bits per hex char)
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_sha256_hex_all_lowercase_hex() {
    let hash = sha256_hex(b"test");
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
}

// ============================================================================
// DIFFICULTY CHECK TESTS
// ============================================================================

#[test]
fn test_meets_difficulty_zero() {
    // Difficulty 0: any hash meets it
    assert!(meets_difficulty("abcdef", 0));
}

#[test]
fn test_meets_difficulty_one() {
    assert!(meets_difficulty("0abcdef", 1));
    assert!(!meets_difficulty("1abcdef", 1));
}

#[test]
fn test_meets_difficulty_two() {
    assert!(meets_difficulty("00abcdef", 2));
    assert!(!meets_difficulty("01abcdef", 2));
}

#[test]
fn test_meets_difficulty_four() {
    assert!(meets_difficulty("0000abcdef", 4));
    assert!(!meets_difficulty("000abcdef", 4));
}

#[test]
fn test_meets_difficulty_all_zeros() {
    let all_zeros = "0".repeat(64);
    assert!(meets_difficulty(&all_zeros, 10));
}

// ============================================================================
// BLOCK CREATION TESTS
// ============================================================================

#[test]
fn test_block_new() {
    let block = Block::with_timestamp(1, "data".to_string(), "prev".to_string(), 2, 1000);
    assert_eq!(block.index, 1);
    assert_eq!(block.data, "data");
    assert_eq!(block.previous_hash, "prev");
    assert_eq!(block.difficulty, 2);
    assert_eq!(block.nonce, 0);
    assert!(block.hash.is_empty());
}

#[test]
fn test_block_genesis() {
    let genesis = Block::genesis(1);
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.data, "Genesis Block");
    assert_eq!(genesis.previous_hash, "0");
    assert!(!genesis.hash.is_empty());
    assert_eq!(genesis.difficulty, 1);
}

#[test]
fn test_block_with_timestamp() {
    let block = Block::with_timestamp(5, "test".to_string(), "hash".to_string(), 3, 42);
    assert_eq!(block.timestamp, 42);
    assert_eq!(block.index, 5);
}

// ============================================================================
// HASH CALCULATION TESTS
// ============================================================================

#[test]
fn test_calculate_hash_deterministic() {
    let block = Block::with_timestamp(1, "data".to_string(), "prev".to_string(), 2, 1000);
    let hash1 = block.calculate_hash();
    let hash2 = block.calculate_hash();
    assert_eq!(hash1, hash2);
}

#[test]
fn test_calculate_hash_length() {
    let block = Block::with_timestamp(0, "test".to_string(), "0".to_string(), 1, 0);
    let hash = block.calculate_hash();
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_calculate_hash_changes_with_nonce() {
    let mut block = Block::with_timestamp(1, "data".to_string(), "prev".to_string(), 2, 1000);
    let hash1 = block.calculate_hash();
    block.nonce = 1;
    let hash2 = block.calculate_hash();
    assert_ne!(hash1, hash2);
}

#[test]
fn test_calculate_hash_changes_with_data() {
    let block1 = Block::with_timestamp(1, "data1".to_string(), "prev".to_string(), 2, 1000);
    let block2 = Block::with_timestamp(1, "data2".to_string(), "prev".to_string(), 2, 1000);
    assert_ne!(block1.calculate_hash(), block2.calculate_hash());
}

#[test]
fn test_calculate_hash_changes_with_previous_hash() {
    let block1 = Block::with_timestamp(1, "data".to_string(), "prev1".to_string(), 2, 1000);
    let block2 = Block::with_timestamp(1, "data".to_string(), "prev2".to_string(), 2, 1000);
    assert_ne!(block1.calculate_hash(), block2.calculate_hash());
}

// ============================================================================
// MINING TESTS
// ============================================================================

#[test]
fn test_mine_difficulty_1() {
    let mut block = Block::with_timestamp(
        1,
        "test mining".to_string(),
        "0".repeat(64),
        1,
        1000,
    );
    let result = block.mine();

    assert!(block.hash.starts_with("0"));
    assert!(result.attempts > 0);
    assert_eq!(result.hash, block.hash);
    assert_eq!(result.nonce, block.nonce);
}

#[test]
fn test_mine_difficulty_2() {
    let mut block = Block::with_timestamp(
        1,
        "harder mining".to_string(),
        "0".repeat(64),
        2,
        2000,
    );
    let result = block.mine();

    assert!(block.hash.starts_with("00"));
    assert!(result.attempts > 0);
}

#[test]
fn test_mine_produces_valid_block() {
    let mut block = Block::with_timestamp(
        1,
        "validate me".to_string(),
        "0".repeat(64),
        2,
        3000,
    );
    block.mine();

    assert!(block.is_valid());
}

#[test]
fn test_mine_nonce_is_nonzero() {
    let mut block = Block::with_timestamp(
        1,
        "nonce test".to_string(),
        "0".repeat(64),
        2,
        4000,
    );
    block.mine();

    // The nonce should have been incremented during mining
    assert!(block.nonce > 0);
}

#[test]
fn test_mining_result_has_duration() {
    let mut block = Block::with_timestamp(
        1,
        "timing".to_string(),
        "0".repeat(64),
        1,
        5000,
    );
    let result = block.mine();

    // Duration should be non-negative
    assert!(result.duration.as_nanos() >= 0);
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[test]
fn test_mined_block_is_valid() {
    let mut block = Block::with_timestamp(
        1,
        "valid block".to_string(),
        "0".repeat(64),
        2,
        6000,
    );
    block.mine();
    assert!(block.is_valid());
}

#[test]
fn test_unmined_block_is_invalid() {
    let block = Block::with_timestamp(
        1,
        "unmined".to_string(),
        "0".repeat(64),
        2,
        7000,
    );
    // Hash is empty, so it doesn't meet difficulty
    assert!(!block.is_valid());
}

#[test]
fn test_tampered_block_is_invalid() {
    let mut block = Block::with_timestamp(
        1,
        "original data".to_string(),
        "0".repeat(64),
        2,
        8000,
    );
    block.mine();
    assert!(block.is_valid());

    // Tamper with the data
    block.data = "tampered data".to_string();
    assert!(!block.is_valid());
}

#[test]
fn test_tampered_nonce_is_invalid() {
    let mut block = Block::with_timestamp(
        1,
        "nonce tamper".to_string(),
        "0".repeat(64),
        2,
        9000,
    );
    block.mine();
    assert!(block.is_valid());

    // Change the nonce without recalculating hash
    block.nonce += 1;
    assert!(!block.is_valid());
}

#[test]
fn test_genesis_block_hash_is_not_empty() {
    let genesis = Block::genesis(1);
    assert!(!genesis.hash.is_empty());
    assert_eq!(genesis.hash.len(), 64);
}

// ============================================================================
// BLOCKCHAIN TESTS
// ============================================================================

#[test]
fn test_blockchain_new() {
    let bc = Blockchain::new(1, 10);
    assert_eq!(bc.len(), 1); // Genesis block
    assert_eq!(bc.difficulty, 1);
    assert_eq!(bc.target_block_time, 10);
}

#[test]
fn test_blockchain_genesis_is_valid() {
    let bc = Blockchain::new(1, 10);
    assert!(bc.is_valid());
}

#[test]
fn test_blockchain_add_block() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("Transaction 1".to_string());
    assert_eq!(bc.len(), 2);
}

#[test]
fn test_blockchain_add_multiple_blocks() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("Block 1".to_string());
    bc.add_block("Block 2".to_string());
    bc.add_block("Block 3".to_string());
    assert_eq!(bc.len(), 4);
}

#[test]
fn test_blockchain_is_valid_after_mining() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("TX: Alice -> Bob".to_string());
    bc.add_block("TX: Bob -> Charlie".to_string());
    assert!(bc.is_valid());
}

#[test]
fn test_blockchain_chain_links() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("Block 1".to_string());
    bc.add_block("Block 2".to_string());

    // Each block's previous_hash should match the prior block's hash
    assert_eq!(bc.chain[1].previous_hash, bc.chain[0].hash);
    assert_eq!(bc.chain[2].previous_hash, bc.chain[1].hash);
}

#[test]
fn test_blockchain_tamper_detection() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("Legitimate transaction".to_string());
    bc.add_block("Another transaction".to_string());
    assert!(bc.is_valid());

    // Tamper with block 1's data
    bc.chain[1].data = "Fraudulent transaction".to_string();
    assert!(!bc.is_valid());
}

#[test]
fn test_blockchain_latest_block() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("Latest".to_string());
    let latest = bc.latest_block();
    assert_eq!(latest.data, "Latest");
    assert_eq!(latest.index, 1);
}

#[test]
fn test_blockchain_is_not_empty() {
    let bc = Blockchain::new(1, 10);
    assert!(!bc.is_empty());
}

#[test]
fn test_blockchain_block_indices() {
    let mut bc = Blockchain::new(1, 10);
    bc.add_block("A".to_string());
    bc.add_block("B".to_string());

    assert_eq!(bc.chain[0].index, 0);
    assert_eq!(bc.chain[1].index, 1);
    assert_eq!(bc.chain[2].index, 2);
}

// ============================================================================
// DIFFICULTY SCALING TESTS (LOWER DIFFICULTY FOR SPEED)
// ============================================================================

#[test]
fn test_higher_difficulty_means_more_leading_zeros() {
    let mut block1 = Block::with_timestamp(1, "d1".to_string(), "0".repeat(64), 1, 100);
    let mut block2 = Block::with_timestamp(1, "d2".to_string(), "0".repeat(64), 2, 200);

    block1.mine();
    block2.mine();

    assert!(block1.hash.starts_with("0"));
    assert!(block2.hash.starts_with("00"));
}

#[test]
fn test_difficulty_2_requires_more_attempts_on_average() {
    // Run a small statistical test: difficulty 2 should generally
    // require more attempts than difficulty 1
    let mut total_d1 = 0u64;
    let mut total_d2 = 0u64;

    for i in 0..5 {
        let mut b1 = Block::with_timestamp(i, format!("d1_{}", i), "0".repeat(64), 1, i * 100);
        let mut b2 = Block::with_timestamp(i, format!("d2_{}", i), "0".repeat(64), 2, i * 100 + 50);
        let r1 = b1.mine();
        let r2 = b2.mine();
        total_d1 += r1.attempts;
        total_d2 += r2.attempts;
    }

    // On average, difficulty 2 should require ~16x more attempts
    // Use a very loose bound to avoid flaky tests
    assert!(total_d2 > total_d1);
}
