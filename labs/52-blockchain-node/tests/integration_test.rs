// Lab 52: Blockchain Node - Integration Tests
//
// Tests for Block, Transaction, UTXO set, Mempool, Blockchain,
// merkle root computation, mining, and validation.

use blockchain_node::*;

// ============================================================================
// BLOCK TESTS
// ============================================================================

#[test]
fn test_block_creation() {
    let block = Block::new(0, 1000, vec![], "0".into());
    assert_eq!(block.index, 0);
    assert_eq!(block.timestamp, 1000);
    assert_eq!(block.previous_hash, "0");
    assert!(!block.hash.is_empty());
    assert_eq!(block.nonce, 0);
}

#[test]
fn test_block_hash_is_64_hex_chars() {
    let block = Block::new(0, 1000, vec![], "0".into());
    assert_eq!(block.hash.len(), 64); // SHA-256 = 32 bytes = 64 hex chars
}

#[test]
fn test_block_hash_deterministic() {
    let b1 = Block::new(0, 1000, vec![], "prev".into());
    let b2 = Block::new(0, 1000, vec![], "prev".into());
    // Same parameters should produce same hash (same nonce=0, same timestamp)
    assert_eq!(b1.hash, b2.hash);
}

#[test]
fn test_block_hash_changes_with_index() {
    let b1 = Block::new(0, 1000, vec![], "prev".into());
    let b2 = Block::new(1, 1000, vec![], "prev".into());
    assert_ne!(b1.hash, b2.hash);
}

#[test]
fn test_block_hash_changes_with_previous_hash() {
    let b1 = Block::new(0, 1000, vec![], "aaa".into());
    let b2 = Block::new(0, 1000, vec![], "bbb".into());
    assert_ne!(b1.hash, b2.hash);
}

#[test]
fn test_block_mining() {
    let mut block = Block::new(0, 1000, vec![], "0".into());
    block.mine(1); // difficulty 1: hash must start with "0"
    assert!(block.hash.starts_with("0"));
    assert!(block.nonce > 0 || block.hash.starts_with("0"));
}

#[test]
fn test_block_mining_difficulty_2() {
    let mut block = Block::new(0, 1000, vec![], "0".into());
    block.mine(2); // difficulty 2: hash must start with "00"
    assert!(block.hash.starts_with("00"));
}

#[test]
fn test_block_verify_merkle_root() {
    let tx = Transaction::coinbase("miner".into(), 50, 1000, "coinbase".into());
    let block = Block::new(0, 1000, vec![tx], "0".into());
    assert!(block.verify_merkle_root());
}

#[test]
fn test_block_verify_merkle_root_tampered() {
    let tx = Transaction::coinbase("miner".into(), 50, 1000, "coinbase".into());
    let mut block = Block::new(0, 1000, vec![tx], "0".into());
    block.merkle_root = "tampered_root".to_string();
    assert!(!block.verify_merkle_root());
}

// ============================================================================
// TRANSACTION TESTS
// ============================================================================

#[test]
fn test_transaction_creation() {
    let tx = Transaction::new(
        vec![TxInput {
            txid: "prev_tx".into(),
            vout: 0,
            signature: "sig".into(),
        }],
        vec![TxOutput {
            address: "Alice".into(),
            amount: 50,
        }],
        1000,
    );

    assert!(!tx.txid.is_empty());
    assert_eq!(tx.inputs.len(), 1);
    assert_eq!(tx.outputs.len(), 1);
}

#[test]
fn test_transaction_txid_deterministic() {
    let tx1 = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 100 }],
        1000,
    );
    let tx2 = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 100 }],
        1000,
    );
    assert_eq!(tx1.txid, tx2.txid);
}

#[test]
fn test_transaction_different_amounts_different_txid() {
    let tx1 = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 100 }],
        1000,
    );
    let tx2 = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 200 }],
        1000,
    );
    assert_ne!(tx1.txid, tx2.txid);
}

#[test]
fn test_coinbase_transaction() {
    let tx = Transaction::coinbase("miner".into(), 50_00000000, 1000, "cb_0".into());
    assert!(tx.is_coinbase());
    assert!(tx.inputs.is_empty());
    assert_eq!(tx.outputs.len(), 1);
    assert_eq!(tx.outputs[0].address, "miner");
    assert_eq!(tx.outputs[0].amount, 50_00000000);
}

#[test]
fn test_regular_transaction_is_not_coinbase() {
    let tx = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 100 }],
        1000,
    );
    assert!(!tx.is_coinbase());
}

#[test]
fn test_transaction_calculate_fee() {
    let mut utxo_set = UTXOSet::new();
    utxo_set.add_utxo("prev".into(), 0, TxOutput { address: "A".into(), amount: 1000 });

    let tx = Transaction::new(
        vec![TxInput { txid: "prev".into(), vout: 0, signature: "sig".into() }],
        vec![TxOutput { address: "B".into(), amount: 900 }],
        1000,
    );

    assert_eq!(tx.calculate_fee(&utxo_set), 100);
}

// ============================================================================
// UTXO SET TESTS
// ============================================================================

#[test]
fn test_utxo_set_empty() {
    let set = UTXOSet::new();
    assert_eq!(set.count(), 0);
    assert_eq!(set.get_balance("nobody"), 0);
}

#[test]
fn test_utxo_set_add_and_get() {
    let mut set = UTXOSet::new();
    set.add_utxo("tx1".into(), 0, TxOutput { address: "Alice".into(), amount: 500 });

    let utxo = set.get_utxo("tx1", 0);
    assert!(utxo.is_some());
    assert_eq!(utxo.unwrap().output.amount, 500);
}

#[test]
fn test_utxo_set_remove() {
    let mut set = UTXOSet::new();
    set.add_utxo("tx1".into(), 0, TxOutput { address: "Alice".into(), amount: 500 });
    set.remove_utxo("tx1", 0);
    assert!(set.get_utxo("tx1", 0).is_none());
    assert_eq!(set.count(), 0);
}

#[test]
fn test_utxo_set_balance() {
    let mut set = UTXOSet::new();
    set.add_utxo("tx1".into(), 0, TxOutput { address: "Alice".into(), amount: 300 });
    set.add_utxo("tx2".into(), 0, TxOutput { address: "Alice".into(), amount: 200 });
    set.add_utxo("tx3".into(), 0, TxOutput { address: "Bob".into(), amount: 100 });

    assert_eq!(set.get_balance("Alice"), 500);
    assert_eq!(set.get_balance("Bob"), 100);
    assert_eq!(set.get_balance("Charlie"), 0);
}

#[test]
fn test_utxo_set_get_for_address() {
    let mut set = UTXOSet::new();
    set.add_utxo("tx1".into(), 0, TxOutput { address: "Alice".into(), amount: 100 });
    set.add_utxo("tx2".into(), 0, TxOutput { address: "Bob".into(), amount: 200 });
    set.add_utxo("tx3".into(), 0, TxOutput { address: "Alice".into(), amount: 300 });

    let alice_utxos = set.get_utxos_for_address("Alice");
    assert_eq!(alice_utxos.len(), 2);
}

#[test]
fn test_utxo_set_same_tx_different_vout() {
    let mut set = UTXOSet::new();
    set.add_utxo("tx1".into(), 0, TxOutput { address: "Alice".into(), amount: 100 });
    set.add_utxo("tx1".into(), 1, TxOutput { address: "Bob".into(), amount: 200 });

    assert_eq!(set.count(), 2);
    assert_eq!(set.get_balance("Alice"), 100);
    assert_eq!(set.get_balance("Bob"), 200);
}

// ============================================================================
// MEMPOOL TESTS
// ============================================================================

#[test]
fn test_mempool_empty() {
    let mempool = Mempool::new();
    assert_eq!(mempool.size(), 0);
}

#[test]
fn test_mempool_add_transaction() {
    let mut mempool = Mempool::new();
    let tx = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 100 }],
        1000,
    );
    let txid = tx.txid.clone();
    mempool.add_transaction(tx);

    assert_eq!(mempool.size(), 1);
    assert!(mempool.contains(&txid));
}

#[test]
fn test_mempool_remove_transaction() {
    let mut mempool = Mempool::new();
    let tx = Transaction::new(
        vec![TxInput { txid: "a".into(), vout: 0, signature: "s".into() }],
        vec![TxOutput { address: "B".into(), amount: 100 }],
        1000,
    );
    let txid = tx.txid.clone();
    mempool.add_transaction(tx);
    mempool.remove_transaction(&txid);

    assert_eq!(mempool.size(), 0);
    assert!(!mempool.contains(&txid));
}

#[test]
fn test_mempool_select_transactions() {
    let mut mempool = Mempool::new();
    for i in 0..5 {
        let tx = Transaction::new(
            vec![TxInput { txid: format!("tx{}", i), vout: 0, signature: "s".into() }],
            vec![TxOutput { address: "B".into(), amount: 100 }],
            1000 + i,
        );
        mempool.add_transaction(tx);
    }

    let selected = mempool.select_transactions();
    assert_eq!(selected.len(), 5);
}

// ============================================================================
// MERKLE ROOT TESTS
// ============================================================================

#[test]
fn test_merkle_root_empty() {
    assert_eq!(calculate_merkle_root(&[]), "0");
}

#[test]
fn test_merkle_root_single_tx() {
    let tx = Transaction::coinbase("addr".into(), 100, 0, "tx1".into());
    let root = calculate_merkle_root(&[tx]);
    assert_eq!(root, "tx1"); // single tx -> root is its txid
}

#[test]
fn test_merkle_root_two_transactions() {
    let tx1 = Transaction::coinbase("a".into(), 100, 0, "txA".into());
    let tx2 = Transaction::coinbase("b".into(), 200, 0, "txB".into());
    let root = calculate_merkle_root(&[tx1, tx2]);

    // Should be SHA-256("txAtxB") as hex
    assert!(!root.is_empty());
    assert_eq!(root.len(), 64); // SHA-256 hash
}

#[test]
fn test_merkle_root_deterministic() {
    let tx1 = Transaction::coinbase("a".into(), 100, 0, "txA".into());
    let tx2 = Transaction::coinbase("b".into(), 200, 0, "txB".into());

    let root1 = calculate_merkle_root(&[tx1.clone(), tx2.clone()]);
    let root2 = calculate_merkle_root(&[tx1, tx2]);
    assert_eq!(root1, root2);
}

#[test]
fn test_merkle_root_odd_count() {
    let tx1 = Transaction::coinbase("a".into(), 100, 0, "tx1".into());
    let tx2 = Transaction::coinbase("b".into(), 200, 0, "tx2".into());
    let tx3 = Transaction::coinbase("c".into(), 300, 0, "tx3".into());
    let root = calculate_merkle_root(&[tx1, tx2, tx3]);

    assert!(!root.is_empty());
    assert_eq!(root.len(), 64);
}

// ============================================================================
// BLOCKCHAIN TESTS
// ============================================================================

#[test]
fn test_blockchain_creation() {
    let chain = Blockchain::new(1, 0);
    assert_eq!(chain.height(), 1); // genesis block
    assert!(chain.get_latest_block().is_some());
}

#[test]
fn test_blockchain_genesis_block() {
    let chain = Blockchain::new(1, 0);
    let genesis = chain.get_block(0).unwrap();
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.previous_hash, "0");
    assert!(genesis.hash.starts_with("0")); // difficulty 1
}

#[test]
fn test_blockchain_add_block() {
    let mut chain = Blockchain::new(1, 0);
    let prev_hash = chain.get_latest_block().unwrap().hash.clone();

    let tx = Transaction::coinbase("miner".into(), 50, 1000, "cb_1".into());
    let mut block = Block::new(1, 1000, vec![tx], prev_hash);
    block.mine(1);
    chain.add_block(block);

    assert_eq!(chain.height(), 2);
}

#[test]
fn test_blockchain_valid_chain() {
    let mut chain = Blockchain::new(1, 0);
    let prev_hash = chain.get_latest_block().unwrap().hash.clone();

    let tx = Transaction::coinbase("miner".into(), 50, 1000, "cb_1".into());
    let mut block = Block::new(1, 1000, vec![tx], prev_hash);
    block.mine(1);
    chain.add_block(block);

    assert!(chain.is_valid());
}

#[test]
fn test_blockchain_invalid_chain_tampered_hash() {
    let mut chain = Blockchain::new(1, 0);
    let prev_hash = chain.get_latest_block().unwrap().hash.clone();

    let tx = Transaction::coinbase("miner".into(), 50, 1000, "cb_1".into());
    let mut block = Block::new(1, 1000, vec![tx], prev_hash);
    block.mine(1);
    // Tamper with the hash after mining
    block.hash = "0tampered_hash_value_that_starts_with_zero_for_pow".to_string();
    chain.add_block(block);

    assert!(!chain.is_valid());
}

#[test]
fn test_blockchain_invalid_previous_hash() {
    let mut chain = Blockchain::new(1, 0);

    let tx = Transaction::coinbase("miner".into(), 50, 1000, "cb_1".into());
    let mut block = Block::new(1, 1000, vec![tx], "wrong_prev_hash".into());
    block.mine(1);
    chain.add_block(block);

    assert!(!chain.is_valid());
}

// ============================================================================
// TRANSACTION VALIDATION TESTS
// ============================================================================

#[test]
fn test_validate_transaction_success() {
    let mut utxo_set = UTXOSet::new();
    utxo_set.add_utxo("prev_tx".into(), 0, TxOutput {
        address: "Alice".into(),
        amount: 100_000,
    });

    let tx = Transaction::new(
        vec![TxInput {
            txid: "prev_tx".into(),
            vout: 0,
            signature: "valid_sig".into(),
        }],
        vec![TxOutput { address: "Bob".into(), amount: 98_000 }],
        1000,
    );

    assert!(validate_transaction(&tx, &utxo_set, 1000).is_ok());
}

#[test]
fn test_validate_transaction_missing_utxo() {
    let utxo_set = UTXOSet::new(); // empty

    let tx = Transaction::new(
        vec![TxInput {
            txid: "nonexistent".into(),
            vout: 0,
            signature: "sig".into(),
        }],
        vec![TxOutput { address: "Bob".into(), amount: 100 }],
        1000,
    );

    let result = validate_transaction(&tx, &utxo_set, 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("UTXO not found"));
}

#[test]
fn test_validate_transaction_empty_signature() {
    let mut utxo_set = UTXOSet::new();
    utxo_set.add_utxo("prev_tx".into(), 0, TxOutput {
        address: "Alice".into(),
        amount: 100_000,
    });

    let tx = Transaction::new(
        vec![TxInput {
            txid: "prev_tx".into(),
            vout: 0,
            signature: String::new(), // empty!
        }],
        vec![TxOutput { address: "Bob".into(), amount: 98_000 }],
        1000,
    );

    let result = validate_transaction(&tx, &utxo_set, 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid signature"));
}

#[test]
fn test_validate_transaction_outputs_exceed_inputs() {
    let mut utxo_set = UTXOSet::new();
    utxo_set.add_utxo("prev_tx".into(), 0, TxOutput {
        address: "Alice".into(),
        amount: 100,
    });

    let tx = Transaction::new(
        vec![TxInput {
            txid: "prev_tx".into(),
            vout: 0,
            signature: "sig".into(),
        }],
        vec![TxOutput { address: "Bob".into(), amount: 200 }], // more than input
        1000,
    );

    let result = validate_transaction(&tx, &utxo_set, 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Outputs exceed inputs"));
}

#[test]
fn test_validate_transaction_fee_too_low() {
    let mut utxo_set = UTXOSet::new();
    utxo_set.add_utxo("prev_tx".into(), 0, TxOutput {
        address: "Alice".into(),
        amount: 100_000,
    });

    let tx = Transaction::new(
        vec![TxInput {
            txid: "prev_tx".into(),
            vout: 0,
            signature: "sig".into(),
        }],
        vec![TxOutput { address: "Bob".into(), amount: 99_999 }], // fee = 1
        1000,
    );

    let result = validate_transaction(&tx, &utxo_set, 1000); // min_fee = 1000
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Fee too low"));
}

// ============================================================================
// PROOF-OF-WORK VALIDATION TESTS
// ============================================================================

#[test]
fn test_validate_pow_valid() {
    let mut block = Block::new(0, 1000, vec![], "0".into());
    block.mine(2);
    assert!(validate_proof_of_work(&block, 2));
}

#[test]
fn test_validate_pow_invalid() {
    let block = Block::new(0, 1000, vec![], "0".into());
    // Unmined block likely doesn't start with "000"
    // (there's a tiny chance it does, but astronomically unlikely)
    // We check for difficulty 3
    if !block.hash.starts_with("000") {
        assert!(!validate_proof_of_work(&block, 3));
    }
}

// ============================================================================
// APPLY BLOCK TO UTXO SET TESTS
// ============================================================================

#[test]
fn test_apply_block_adds_utxos() {
    let mut utxo_set = UTXOSet::new();
    let tx = Transaction::coinbase("miner".into(), 50, 1000, "cb_0".into());
    let block = Block::new(0, 1000, vec![tx], "0".into());

    apply_block_to_utxo_set(&block, &mut utxo_set);

    assert_eq!(utxo_set.count(), 1);
    assert_eq!(utxo_set.get_balance("miner"), 50);
}

#[test]
fn test_apply_block_removes_spent_utxos() {
    let mut utxo_set = UTXOSet::new();
    utxo_set.add_utxo("prev_tx".into(), 0, TxOutput { address: "Alice".into(), amount: 100 });

    let tx = Transaction::new(
        vec![TxInput { txid: "prev_tx".into(), vout: 0, signature: "sig".into() }],
        vec![TxOutput { address: "Bob".into(), amount: 90 }],
        1000,
    );

    let block = Block::new(1, 2000, vec![tx], "prev_hash".into());
    apply_block_to_utxo_set(&block, &mut utxo_set);

    // prev_tx:0 should be removed, new UTXO added for Bob
    assert!(utxo_set.get_utxo("prev_tx", 0).is_none());
    assert_eq!(utxo_set.get_balance("Bob"), 90);
    assert_eq!(utxo_set.get_balance("Alice"), 0);
}

#[test]
fn test_apply_block_full_flow() {
    let mut utxo_set = UTXOSet::new();

    // Genesis: miner gets 100 coins
    let genesis_tx = Transaction::coinbase("genesis".into(), 100_00000000, 0, "gen_tx".into());
    let genesis = Block::new(0, 0, vec![genesis_tx], "0".into());
    apply_block_to_utxo_set(&genesis, &mut utxo_set);

    assert_eq!(utxo_set.get_balance("genesis"), 100_00000000);

    // Block 1: genesis sends 50 to Alice, 49.99 change
    let tx1 = Transaction::new(
        vec![TxInput { txid: "gen_tx".into(), vout: 0, signature: "sig".into() }],
        vec![
            TxOutput { address: "Alice".into(), amount: 50_00000000 },
            TxOutput { address: "genesis".into(), amount: 49_99000000 },
        ],
        1000,
    );
    let block1 = Block::new(1, 1000, vec![tx1], genesis.hash.clone());
    apply_block_to_utxo_set(&block1, &mut utxo_set);

    assert_eq!(utxo_set.get_balance("Alice"), 50_00000000);
    assert_eq!(utxo_set.get_balance("genesis"), 49_99000000);
}

// ============================================================================
// FORMAT UTILITY TESTS
// ============================================================================

#[test]
fn test_format_coins_whole() {
    assert_eq!(format_coins(100_00000000), "100.00");
}

#[test]
fn test_format_coins_fractional() {
    assert_eq!(format_coins(50_50000000), "50.50");
}

#[test]
fn test_format_coins_zero() {
    assert_eq!(format_coins(0), "0.00");
}

// ============================================================================
// INTEGRATION: MULTI-BLOCK CHAIN
// ============================================================================

#[test]
fn test_three_block_chain_valid() {
    let mut chain = Blockchain::new(1, 0);

    for i in 1..=2 {
        let prev_hash = chain.get_latest_block().unwrap().hash.clone();
        let tx = Transaction::coinbase("miner".into(), 50, i * 1000, format!("cb_{}", i));
        let mut block = Block::new(i as u64, i as u64 * 1000, vec![tx], prev_hash);
        block.mine(1);
        chain.add_block(block);
    }

    assert_eq!(chain.height(), 3);
    assert!(chain.is_valid());
}
