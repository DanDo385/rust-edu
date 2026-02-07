// Lab 50: Wallet CLI - Integration Tests
//
// Tests for wallet creation, address generation, UTXO management,
// transaction construction, fee estimation, and UTXO selection strategies.

use wallet_cli::*;

// ============================================================================
// WALLET CREATION TESTS
// ============================================================================

#[test]
fn test_create_wallet() {
    let wallet = Wallet::new("MyWallet".into());
    assert_eq!(wallet.name, "MyWallet");
    assert_eq!(wallet.address_count(), 1); // initial address generated
    assert_eq!(wallet.get_balance(), 0);
    assert_eq!(wallet.utxo_count(), 0);
}

#[test]
fn test_wallet_root_address_format() {
    let wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    // Address should start with bc1q (simulated bech32)
    assert!(addr.starts_with("bc1q"));
    // bc1q + 40 hex chars = 44 chars total
    assert_eq!(addr.len(), 44);
}

#[test]
fn test_generate_multiple_addresses() {
    let mut wallet = Wallet::new("test".into());
    let addr1 = wallet.get_root_address();
    let addr2 = wallet.generate_address();
    let addr3 = wallet.generate_address();

    assert_eq!(wallet.address_count(), 3);
    // All addresses should be different
    assert_ne!(addr1, addr2);
    assert_ne!(addr2, addr3);
    assert_ne!(addr1, addr3);
}

#[test]
fn test_is_my_address() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.generate_address();

    assert!(wallet.is_my_address(&addr));
    assert!(!wallet.is_my_address("bc1qfakeaddress000000000000000000000000"));
}

// ============================================================================
// UTXO AND BALANCE TESTS
// ============================================================================

#[test]
fn test_receive_funds() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();

    wallet.receive_funds("tx001".into(), 0, 100_000_000, addr);
    assert_eq!(wallet.get_balance(), 100_000_000);
    assert_eq!(wallet.utxo_count(), 1);
}

#[test]
fn test_receive_multiple_funds() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();

    wallet.receive_funds("tx001".into(), 0, 150_000_000, addr.clone());
    wallet.receive_funds("tx002".into(), 0, 50_000_000, addr.clone());
    wallet.receive_funds("tx003".into(), 0, 30_000_000, addr);

    assert_eq!(wallet.get_balance(), 230_000_000);
    assert_eq!(wallet.utxo_count(), 3);
}

#[test]
fn test_balance_zero_initially() {
    let wallet = Wallet::new("test".into());
    assert_eq!(wallet.get_balance(), 0);
}

#[test]
fn test_receive_to_different_addresses() {
    let mut wallet = Wallet::new("test".into());
    let addr1 = wallet.get_root_address();
    let addr2 = wallet.generate_address();

    wallet.receive_funds("tx001".into(), 0, 100_000_000, addr1);
    wallet.receive_funds("tx002".into(), 0, 50_000_000, addr2);

    assert_eq!(wallet.get_balance(), 150_000_000);
    assert_eq!(wallet.utxo_count(), 2);
}

#[test]
fn test_same_txid_different_vout() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();

    wallet.receive_funds("tx001".into(), 0, 100_000_000, addr.clone());
    wallet.receive_funds("tx001".into(), 1, 50_000_000, addr);

    assert_eq!(wallet.get_balance(), 150_000_000);
    assert_eq!(wallet.utxo_count(), 2);
}

// ============================================================================
// TRANSACTION CREATION TESTS
// ============================================================================

#[test]
fn test_create_transaction_basic() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    wallet.receive_funds("tx001".into(), 0, 200_000_000, addr); // 2 BTC

    let tx = wallet
        .create_transaction("recipient_addr".into(), 50_000_000, 10)
        .expect("transaction should succeed");

    assert!(!tx.txid.is_empty());
    assert!(!tx.inputs.is_empty());
    assert!(tx.outputs.len() >= 1); // at least payment output
    assert!(tx.fee > 0);
}

#[test]
fn test_transaction_has_correct_payment() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    wallet.receive_funds("tx001".into(), 0, 200_000_000, addr);

    let tx = wallet
        .create_transaction("recipient_addr".into(), 50_000_000, 10)
        .unwrap();

    // First output should be the payment
    assert_eq!(tx.outputs[0].address, "recipient_addr");
    assert_eq!(tx.outputs[0].amount, 50_000_000);
}

#[test]
fn test_transaction_has_change_output() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    wallet.receive_funds("tx001".into(), 0, 200_000_000, addr.clone());

    let tx = wallet
        .create_transaction("recipient_addr".into(), 50_000_000, 10)
        .unwrap();

    // Should have 2 outputs: payment + change
    assert_eq!(tx.outputs.len(), 2);

    // Change output should go back to wallet
    let change_output = &tx.outputs[1];
    assert!(wallet.is_my_address(&change_output.address));

    // Total outputs + fee should equal total inputs
    let total_out: u64 = tx.outputs.iter().map(|o| o.amount).sum();
    let total_in: u64 = tx.inputs.iter().map(|i| i.amount).sum();
    assert_eq!(total_in, total_out + tx.fee);
}

#[test]
fn test_transaction_inputs_signed() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    wallet.receive_funds("tx001".into(), 0, 200_000_000, addr);

    let tx = wallet
        .create_transaction("recipient_addr".into(), 50_000_000, 10)
        .unwrap();

    // All inputs should have non-empty signatures
    for input in &tx.inputs {
        assert!(!input.signature.is_empty());
    }
}

#[test]
fn test_insufficient_funds() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    wallet.receive_funds("tx001".into(), 0, 10_000, addr); // tiny amount

    let result = wallet.create_transaction("recipient".into(), 100_000_000, 10);
    assert_eq!(result.unwrap_err(), WalletError::InsufficientFunds);
}

#[test]
fn test_insufficient_funds_with_fee() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    // Fund exactly 1 BTC -- but after fee, it won't be enough for a 1 BTC send
    wallet.receive_funds("tx001".into(), 0, 100_000_000, addr);

    let result = wallet.create_transaction("recipient".into(), 100_000_000, 10);
    assert_eq!(result.unwrap_err(), WalletError::InsufficientFunds);
}

#[test]
fn test_mark_utxos_spent() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();
    wallet.receive_funds("tx001".into(), 0, 200_000_000, addr);

    let tx = wallet
        .create_transaction("recipient".into(), 50_000_000, 10)
        .unwrap();

    let balance_before = wallet.get_balance();
    wallet.mark_utxos_spent(&tx.inputs);

    assert!(wallet.get_balance() < balance_before);
}

#[test]
fn test_full_send_receive_cycle() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();

    // Receive 2 BTC
    wallet.receive_funds("tx001".into(), 0, 200_000_000, addr);
    assert_eq!(wallet.get_balance(), 200_000_000);

    // Send 0.5 BTC
    let tx = wallet
        .create_transaction("recipient".into(), 50_000_000, 10)
        .unwrap();

    // Mark spent
    wallet.mark_utxos_spent(&tx.inputs);

    // Add change back
    if let Some(change) = tx.outputs.iter().find(|o| wallet.is_my_address(&o.address)) {
        wallet.receive_funds(tx.txid.clone(), 1, change.amount, change.address.clone());
    }

    // Balance should be original minus payment minus fee
    assert_eq!(wallet.get_balance(), 200_000_000 - 50_000_000 - tx.fee);
}

// ============================================================================
// UTXO SELECTION STRATEGY TESTS
// ============================================================================

fn make_test_utxos() -> Vec<UTXO> {
    vec![
        UTXO {
            txid: "tx1".into(),
            vout: 0,
            amount: 100_000_000,
            address: "addr1".into(),
            confirmations: 6,
        },
        UTXO {
            txid: "tx2".into(),
            vout: 0,
            amount: 50_000_000,
            address: "addr2".into(),
            confirmations: 6,
        },
        UTXO {
            txid: "tx3".into(),
            vout: 0,
            amount: 25_000_000,
            address: "addr3".into(),
            confirmations: 6,
        },
        UTXO {
            txid: "tx4".into(),
            vout: 0,
            amount: 10_000_000,
            address: "addr4".into(),
            confirmations: 6,
        },
    ]
}

#[test]
fn test_select_largest_first() {
    let utxos = make_test_utxos();
    let selected = select_largest_first(&utxos, 60_000_000);

    let total: u64 = selected.iter().map(|u| u.amount).sum();
    assert!(total >= 60_000_000);
    // Largest-first should pick 100M first (single UTXO covers target)
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].amount, 100_000_000);
}

#[test]
fn test_select_smallest_first() {
    let utxos = make_test_utxos();
    let selected = select_smallest_first(&utxos, 60_000_000);

    let total: u64 = selected.iter().map(|u| u.amount).sum();
    assert!(total >= 60_000_000);
    // Smallest-first: 10M + 25M + 50M = 85M (need 3 UTXOs)
    assert_eq!(selected.len(), 3);
}

#[test]
fn test_find_exact_match_found() {
    let utxos = make_test_utxos();
    let result = find_exact_match(&utxos, 50_000_000);
    assert!(result.is_some());
    assert_eq!(result.unwrap().amount, 50_000_000);
}

#[test]
fn test_find_exact_match_not_found() {
    let utxos = make_test_utxos();
    let result = find_exact_match(&utxos, 99_999_999);
    assert!(result.is_none());
}

#[test]
fn test_select_largest_first_empty() {
    let utxos: Vec<UTXO> = vec![];
    let selected = select_largest_first(&utxos, 100);
    assert!(selected.is_empty());
}

// ============================================================================
// FEE ESTIMATION TESTS
// ============================================================================

#[test]
fn test_estimate_tx_size_1_in_2_out() {
    let size = estimate_tx_size(1, 2);
    // 10 + 148 + 68 = 226
    assert_eq!(size, 226);
}

#[test]
fn test_estimate_tx_size_2_in_2_out() {
    let size = estimate_tx_size(2, 2);
    // 10 + 296 + 68 = 374
    assert_eq!(size, 374);
}

#[test]
fn test_estimate_tx_size_5_in_2_out() {
    let size = estimate_tx_size(5, 2);
    // 10 + 740 + 68 = 818
    assert_eq!(size, 818);
}

#[test]
fn test_fee_calculation_at_rate() {
    let size = estimate_tx_size(2, 2);
    let fee_rate = 10u64; // sat/vB
    let fee = size * fee_rate;
    assert_eq!(fee, 3740);
}

// ============================================================================
// FORMAT UTILITY TESTS
// ============================================================================

#[test]
fn test_format_btc_one() {
    assert_eq!(format_btc(100_000_000), "1.00000000");
}

#[test]
fn test_format_btc_fractional() {
    assert_eq!(format_btc(50_000_000), "0.50000000");
}

#[test]
fn test_format_btc_one_satoshi() {
    assert_eq!(format_btc(1), "0.00000001");
}

#[test]
fn test_format_btc_zero() {
    assert_eq!(format_btc(0), "0.00000000");
}

// ============================================================================
// TRANSACTION HASH TESTS
// ============================================================================

#[test]
fn test_transaction_hash_deterministic() {
    let tx = Transaction {
        txid: String::new(),
        inputs: vec![TxInput {
            txid: "tx001".into(),
            vout: 0,
            amount: 100_000_000,
            signature: String::new(),
        }],
        outputs: vec![TxOutput {
            address: "recipient".into(),
            amount: 50_000_000,
        }],
        fee: 1000,
        size: 226,
    };

    let hash1 = tx.calculate_hash();
    let hash2 = tx.calculate_hash();
    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 32); // SHA-256
}

#[test]
fn test_different_transactions_different_hashes() {
    let tx1 = Transaction {
        txid: String::new(),
        inputs: vec![TxInput {
            txid: "tx001".into(),
            vout: 0,
            amount: 100_000_000,
            signature: String::new(),
        }],
        outputs: vec![TxOutput {
            address: "recipient".into(),
            amount: 50_000_000,
        }],
        fee: 1000,
        size: 226,
    };

    let tx2 = Transaction {
        txid: String::new(),
        inputs: vec![TxInput {
            txid: "tx001".into(),
            vout: 0,
            amount: 100_000_000,
            signature: String::new(),
        }],
        outputs: vec![TxOutput {
            address: "recipient".into(),
            amount: 60_000_000, // different amount
        }],
        fee: 1000,
        size: 226,
    };

    assert_ne!(tx1.calculate_hash(), tx2.calculate_hash());
}

// ============================================================================
// DUST THRESHOLD TESTS
// ============================================================================

#[test]
fn test_dust_threshold_constant() {
    assert_eq!(DUST_THRESHOLD, 546);
}

#[test]
fn test_no_change_below_dust() {
    let mut wallet = Wallet::new("test".into());
    let addr = wallet.get_root_address();

    // Fund with exactly enough that change would be below dust
    // estimate_tx_size(1, 2) = 226, fee_rate=1 => fee = 226 sat
    // 100_000_000 - 99_999_500 - 226 = 274 < 546 (dust), so no change output
    wallet.receive_funds("tx001".into(), 0, 100_000_000, addr);

    let tx = wallet
        .create_transaction("recipient".into(), 99_999_500, 1)
        .unwrap();

    // Should only have payment output (no change since change < dust)
    assert_eq!(tx.outputs.len(), 1);
    assert_eq!(tx.outputs[0].address, "recipient");
}
