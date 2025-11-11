//! Integration tests for transaction-validation

use transaction_validation::solution::*;

#[test]
fn test_create_wallet() {
    let wallet = Wallet::new();
    let address = wallet.address();
    assert!(!address.is_empty());
    assert_eq!(address.len(), 64); // 32 bytes × 2 hex digits
}

#[test]
fn test_different_wallets_different_keys() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    assert_ne!(wallet1.address(), wallet2.address());
}

#[test]
fn test_sign_transaction() {
    let wallet = Wallet::new();
    let mut tx = Transaction {
        from: wallet.address(),
        to: "recipient".to_string(),
        amount: 100,
        signature: None,
    };

    wallet.sign_transaction(&mut tx);
    assert!(tx.signature.is_some());
}

#[test]
fn test_verify_valid_transaction() {
    let wallet = Wallet::new();
    let mut tx = Transaction {
        from: wallet.address(),
        to: "recipient".to_string(),
        amount: 100,
        signature: None,
    };

    wallet.sign_transaction(&mut tx);
    assert!(verify_transaction(&tx, &wallet.verifying_key));
}

#[test]
fn test_verify_unsigned_transaction() {
    let wallet = Wallet::new();
    let tx = Transaction {
        from: wallet.address(),
        to: "recipient".to_string(),
        amount: 100,
        signature: None,
    };

    assert!(!verify_transaction(&tx, &wallet.verifying_key));
}

#[test]
fn test_verify_tampered_transaction() {
    let wallet = Wallet::new();
    let mut tx = Transaction {
        from: wallet.address(),
        to: "recipient".to_string(),
        amount: 100,
        signature: None,
    };

    wallet.sign_transaction(&mut tx);

    // Tamper with amount
    tx.amount = 1000;

    // Should fail verification
    assert!(!verify_transaction(&tx, &wallet.verifying_key));
}

#[test]
fn test_verify_wrong_public_key() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let mut tx = Transaction {
        from: alice.address(),
        to: "recipient".to_string(),
        amount: 100,
        signature: None,
    };

    alice.sign_transaction(&mut tx);

    // Try to verify with Bob's key
    assert!(!verify_transaction(&tx, &bob.verifying_key));
}
