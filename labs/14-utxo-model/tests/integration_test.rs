use utxo_model::solution::*;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn setup_genesis() -> UtxoSet {
    let mut utxo_set = UtxoSet::new();
    create_genesis_utxo(&mut utxo_set, "genesis:0", "Alice", 100);
    create_genesis_utxo(&mut utxo_set, "genesis:1", "Bob", 50);
    utxo_set
}

// ============================================================================
// TESTS: UTXO CREATION
// ============================================================================

#[test]
fn test_utxo_new() {
    let utxo = Utxo::new("Alice".to_string(), 100);
    assert_eq!(utxo.owner, "Alice");
    assert_eq!(utxo.amount, 100);
}

#[test]
fn test_genesis_utxo_creation() {
    let mut utxo_set = UtxoSet::new();
    create_genesis_utxo(&mut utxo_set, "genesis:0", "Alice", 100);
    assert_eq!(utxo_set.len(), 1);
    assert_eq!(utxo_set.get("genesis:0").unwrap().owner, "Alice");
    assert_eq!(utxo_set.get("genesis:0").unwrap().amount, 100);
}

#[test]
fn test_multiple_genesis_utxos() {
    let utxo_set = setup_genesis();
    assert_eq!(utxo_set.len(), 2);
    assert_eq!(get_balance(&utxo_set, "Alice"), 100);
    assert_eq!(get_balance(&utxo_set, "Bob"), 50);
}

// ============================================================================
// TESTS: BALANCE CALCULATION
// ============================================================================

#[test]
fn test_get_balance_single_utxo() {
    let utxo_set = setup_genesis();
    assert_eq!(get_balance(&utxo_set, "Alice"), 100);
}

#[test]
fn test_get_balance_no_utxos() {
    let utxo_set = setup_genesis();
    assert_eq!(get_balance(&utxo_set, "Charlie"), 0);
}

#[test]
fn test_get_balance_empty_set() {
    let utxo_set = UtxoSet::new();
    assert_eq!(get_balance(&utxo_set, "Alice"), 0);
}

#[test]
fn test_get_balance_multiple_utxos() {
    let mut utxo_set = UtxoSet::new();
    create_genesis_utxo(&mut utxo_set, "g:0", "Alice", 30);
    create_genesis_utxo(&mut utxo_set, "g:1", "Alice", 50);
    create_genesis_utxo(&mut utxo_set, "g:2", "Alice", 20);
    assert_eq!(get_balance(&utxo_set, "Alice"), 100);
}

// ============================================================================
// TESTS: GET UTXOS FOR ADDRESS
// ============================================================================

#[test]
fn test_get_utxos_for_address() {
    let utxo_set = setup_genesis();
    let alice_utxos = get_utxos_for_address(&utxo_set, "Alice");
    assert_eq!(alice_utxos.len(), 1);
    assert_eq!(alice_utxos[0].1.amount, 100);
}

#[test]
fn test_get_utxos_for_unknown_address() {
    let utxo_set = setup_genesis();
    let utxos = get_utxos_for_address(&utxo_set, "Charlie");
    assert!(utxos.is_empty());
}

// ============================================================================
// TESTS: SIMPLE TRANSFER
// ============================================================================

#[test]
fn test_simple_transfer() {
    let mut utxo_set = setup_genesis();

    // Alice sends 30 to Charlie, 70 change back to herself
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
            TxOutput::new("Alice".to_string(), 70),
        ],
    );

    let result = apply_transaction(&mut utxo_set, &tx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0); // no fee

    assert_eq!(get_balance(&utxo_set, "Alice"), 70);
    assert_eq!(get_balance(&utxo_set, "Charlie"), 30);
    assert_eq!(get_balance(&utxo_set, "Bob"), 50);
}

#[test]
fn test_transfer_with_fee() {
    let mut utxo_set = setup_genesis();

    // Alice sends 30 to Charlie, 65 change (fee = 5)
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
            TxOutput::new("Alice".to_string(), 65),
        ],
    );

    let result = apply_transaction(&mut utxo_set, &tx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5); // 5 fee
}

#[test]
fn test_spent_utxo_removed() {
    let mut utxo_set = setup_genesis();

    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Charlie".to_string(), 100)],
    );

    apply_transaction(&mut utxo_set, &tx).unwrap();

    // genesis:0 should no longer exist
    assert!(utxo_set.get("genesis:0").is_none());
    // new UTXO should exist
    assert!(utxo_set.get("tx1:0").is_some());
}

#[test]
fn test_new_utxo_ids_format() {
    let mut utxo_set = setup_genesis();

    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
            TxOutput::new("Alice".to_string(), 70),
        ],
    );

    apply_transaction(&mut utxo_set, &tx).unwrap();

    // New UTXOs should have format "tx_id:index"
    assert!(utxo_set.contains_key("tx1:0"));
    assert!(utxo_set.contains_key("tx1:1"));
}

// ============================================================================
// TESTS: MULTIPLE INPUTS
// ============================================================================

#[test]
fn test_multiple_inputs() {
    let mut utxo_set = setup_genesis();

    // First: Alice sends 30 to Charlie
    let tx1 = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
            TxOutput::new("Alice".to_string(), 70),
        ],
    );
    apply_transaction(&mut utxo_set, &tx1).unwrap();

    // Second: Bob sends 20 to Charlie
    let tx2 = Transaction::new(
        "tx2".to_string(),
        vec![TxInput::new("genesis:1".to_string(), "Bob".to_string())],
        vec![
            TxOutput::new("Charlie".to_string(), 20),
            TxOutput::new("Bob".to_string(), 30),
        ],
    );
    apply_transaction(&mut utxo_set, &tx2).unwrap();

    // Charlie combines both UTXOs (30 + 20 = 50)
    let tx3 = Transaction::new(
        "tx3".to_string(),
        vec![
            TxInput::new("tx1:0".to_string(), "Charlie".to_string()),
            TxInput::new("tx2:0".to_string(), "Charlie".to_string()),
        ],
        vec![
            TxOutput::new("Alice".to_string(), 45),
            TxOutput::new("Charlie".to_string(), 5),
        ],
    );
    apply_transaction(&mut utxo_set, &tx3).unwrap();

    assert_eq!(get_balance(&utxo_set, "Alice"), 70 + 45);
    assert_eq!(get_balance(&utxo_set, "Charlie"), 5);
    assert_eq!(get_balance(&utxo_set, "Bob"), 30);
}

// ============================================================================
// TESTS: DOUBLE-SPEND PREVENTION
// ============================================================================

#[test]
fn test_double_spend_prevented() {
    let mut utxo_set = setup_genesis();

    // First spend: valid
    let tx1 = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Bob".to_string(), 100)],
    );
    assert!(apply_transaction(&mut utxo_set, &tx1).is_ok());

    // Second spend of same UTXO: should fail
    let tx2 = Transaction::new(
        "tx2".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Charlie".to_string(), 100)],
    );
    let result = apply_transaction(&mut utxo_set, &tx2);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_nonexistent_utxo() {
    let mut utxo_set = setup_genesis();

    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("fake:99".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Bob".to_string(), 100)],
    );

    let result = apply_transaction(&mut utxo_set, &tx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

// ============================================================================
// TESTS: OWNERSHIP VIOLATION
// ============================================================================

#[test]
fn test_ownership_violation() {
    let mut utxo_set = setup_genesis();

    // Charlie tries to spend Alice's UTXO
    let tx = Transaction::new(
        "theft".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Charlie".to_string())],
        vec![TxOutput::new("Charlie".to_string(), 100)],
    );

    let result = apply_transaction(&mut utxo_set, &tx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Ownership violation"));
}

#[test]
fn test_wrong_owner_cannot_spend() {
    let mut utxo_set = setup_genesis();

    // Bob tries to spend Alice's UTXO
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Bob".to_string())],
        vec![TxOutput::new("Bob".to_string(), 100)],
    );

    let result = apply_transaction(&mut utxo_set, &tx);
    assert!(result.is_err());
}

// ============================================================================
// TESTS: INSUFFICIENT FUNDS
// ============================================================================

#[test]
fn test_outputs_exceed_inputs() {
    let mut utxo_set = setup_genesis();

    // Bob has 50, tries to send 100
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:1".to_string(), "Bob".to_string())],
        vec![TxOutput::new("Alice".to_string(), 100)],
    );

    let result = apply_transaction(&mut utxo_set, &tx);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("outputs"));
}

// ============================================================================
// TESTS: EDGE CASES
// ============================================================================

#[test]
fn test_full_amount_transfer_no_change() {
    let mut utxo_set = setup_genesis();

    // Alice sends her entire 100 to Bob (no change)
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Bob".to_string(), 100)],
    );

    assert!(apply_transaction(&mut utxo_set, &tx).is_ok());
    assert_eq!(get_balance(&utxo_set, "Alice"), 0);
    assert_eq!(get_balance(&utxo_set, "Bob"), 150); // 50 + 100
}

#[test]
fn test_transaction_preserves_unrelated_utxos() {
    let mut utxo_set = setup_genesis();

    // Alice sends to Charlie — Bob's UTXO should be untouched
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Charlie".to_string(), 100)],
    );

    apply_transaction(&mut utxo_set, &tx).unwrap();
    assert_eq!(get_balance(&utxo_set, "Bob"), 50);
    assert!(utxo_set.contains_key("genesis:1"));
}

#[test]
fn test_chain_of_transactions() {
    let mut utxo_set = UtxoSet::new();
    create_genesis_utxo(&mut utxo_set, "g:0", "Alice", 100);

    // Alice -> Bob: 100
    let tx1 = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("g:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Bob".to_string(), 100)],
    );
    apply_transaction(&mut utxo_set, &tx1).unwrap();

    // Bob -> Charlie: 100
    let tx2 = Transaction::new(
        "tx2".to_string(),
        vec![TxInput::new("tx1:0".to_string(), "Bob".to_string())],
        vec![TxOutput::new("Charlie".to_string(), 100)],
    );
    apply_transaction(&mut utxo_set, &tx2).unwrap();

    // Charlie -> Dave: 100
    let tx3 = Transaction::new(
        "tx3".to_string(),
        vec![TxInput::new("tx2:0".to_string(), "Charlie".to_string())],
        vec![TxOutput::new("Dave".to_string(), 100)],
    );
    apply_transaction(&mut utxo_set, &tx3).unwrap();

    assert_eq!(get_balance(&utxo_set, "Alice"), 0);
    assert_eq!(get_balance(&utxo_set, "Bob"), 0);
    assert_eq!(get_balance(&utxo_set, "Charlie"), 0);
    assert_eq!(get_balance(&utxo_set, "Dave"), 100);
}

#[test]
fn test_failed_transaction_does_not_modify_set() {
    let mut utxo_set = setup_genesis();
    let original_len = utxo_set.len();

    // Invalid transaction (ownership violation)
    let tx = Transaction::new(
        "bad".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Charlie".to_string())],
        vec![TxOutput::new("Charlie".to_string(), 100)],
    );

    let _ = apply_transaction(&mut utxo_set, &tx);

    // UTXO set should be unchanged
    assert_eq!(utxo_set.len(), original_len);
    assert_eq!(get_balance(&utxo_set, "Alice"), 100);
}

#[test]
fn test_self_transfer() {
    let mut utxo_set = setup_genesis();

    // Alice sends to herself (consolidation or change)
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![TxOutput::new("Alice".to_string(), 100)],
    );

    assert!(apply_transaction(&mut utxo_set, &tx).is_ok());
    assert_eq!(get_balance(&utxo_set, "Alice"), 100);
}

#[test]
fn test_multiple_outputs_same_recipient() {
    let mut utxo_set = setup_genesis();

    // Alice sends two separate UTXOs to Bob
    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![
            TxOutput::new("Bob".to_string(), 60),
            TxOutput::new("Bob".to_string(), 40),
        ],
    );

    apply_transaction(&mut utxo_set, &tx).unwrap();
    assert_eq!(get_balance(&utxo_set, "Bob"), 50 + 60 + 40);

    // Bob should have 3 UTXOs: genesis:1 + tx1:0 + tx1:1
    let bob_utxos = get_utxos_for_address(&utxo_set, "Bob");
    assert_eq!(bob_utxos.len(), 3);
}
