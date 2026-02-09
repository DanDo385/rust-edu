use simple_blockchain_alternative::solution::{Block, Blockchain};

#[test]
fn test_genesis_block() {
    let genesis = Block::genesis();
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.previous_hash, "0");
    assert!(!genesis.hash.is_empty());
}

#[test]
fn test_add_block() {
    let mut chain = Blockchain::new(2);
    chain.add_block("A->B:5".to_string());
    assert_eq!(chain.chain.len(), 2);
    assert_eq!(chain.chain[1].index, 1);
    assert_eq!(chain.chain[1].previous_hash, chain.chain[0].hash);
}

#[test]
fn test_chain_valid_after_mining() {
    let mut chain = Blockchain::new(2);
    chain.add_block("tx1".to_string());
    chain.add_block("tx2".to_string());
    assert!(chain.is_valid());
}

#[test]
fn test_chain_invalid_after_tamper() {
    let mut chain = Blockchain::new(2);
    chain.add_block("tx1".to_string());
    chain.add_block("tx2".to_string());

    chain.chain[1].data = "tampered".to_string();
    assert!(!chain.is_valid());
}
