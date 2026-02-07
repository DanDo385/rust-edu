// Lab 49: Digital Signatures - Integration Tests
//
// Tests for secp256k1 ECDSA signing, verification, and blockchain
// transaction signing. Validates cryptographic correctness and
// tamper detection.

use digital_signatures::*;

// ============================================================================
// KEY PAIR TESTS
// ============================================================================

#[test]
fn test_generate_keypair() {
    let kp = KeyPair::generate();
    let pub_hex = kp.public_key_hex();
    // Compressed secp256k1 public key is 33 bytes = 66 hex chars
    assert_eq!(pub_hex.len(), 66);
    // Compressed keys start with 02 or 03
    assert!(pub_hex.starts_with("02") || pub_hex.starts_with("03"));
}

#[test]
fn test_two_keypairs_are_different() {
    let kp1 = KeyPair::generate();
    let kp2 = KeyPair::generate();
    // Two randomly generated keys should differ (astronomically unlikely to collide)
    assert_ne!(kp1.public_key_hex(), kp2.public_key_hex());
}

#[test]
fn test_keypair_from_signing_key() {
    use k256::ecdsa::SigningKey;
    use rand::rngs::OsRng;

    let sk = SigningKey::random(&mut OsRng);
    let expected_pub = hex::encode(sk.verifying_key().to_encoded_point(true).as_bytes());

    let kp = KeyPair::from_signing_key(sk);
    assert_eq!(kp.public_key_hex(), expected_pub);
}

// ============================================================================
// SIGNING AND VERIFICATION TESTS
// ============================================================================

#[test]
fn test_sign_message() {
    let kp = KeyPair::generate();
    let signed = kp.sign(b"Hello, Blockchain!");
    assert_eq!(signed.message, b"Hello, Blockchain!");
    assert!(!signed.signature_hex().is_empty());
    assert_eq!(signed.signer_public_key, kp.public_key_hex());
}

#[test]
fn test_verify_valid_signature() {
    let kp = KeyPair::generate();
    let signed = kp.sign(b"test message");
    assert!(verify_signed_message(&signed, kp.verifying_key()));
}

#[test]
fn test_verify_with_raw_function() {
    let kp = KeyPair::generate();
    let message = b"raw verify test";
    let signed = kp.sign(message);
    assert!(verify_signature(message, &signed.signature, kp.verifying_key()));
}

#[test]
fn test_reject_tampered_message() {
    let kp = KeyPair::generate();
    let signed = kp.sign(b"original message");
    // Try verifying with a different message
    assert!(!verify_signature(b"tampered message", &signed.signature, kp.verifying_key()));
}

#[test]
fn test_reject_wrong_key() {
    let kp1 = KeyPair::generate();
    let kp2 = KeyPair::generate();
    let signed = kp1.sign(b"signed by kp1");
    // Verify with kp2's key should fail
    assert!(!verify_signed_message(&signed, kp2.verifying_key()));
}

#[test]
fn test_empty_message_signing() {
    let kp = KeyPair::generate();
    let signed = kp.sign(b"");
    assert!(verify_signed_message(&signed, kp.verifying_key()));
}

#[test]
fn test_large_message_signing() {
    let kp = KeyPair::generate();
    let large_msg = vec![0xABu8; 10_000];
    let signed = kp.sign(&large_msg);
    assert!(verify_signed_message(&signed, kp.verifying_key()));
}

// ============================================================================
// BIT FLIP / TAMPER DETECTION TESTS
// ============================================================================

#[test]
fn test_demonstrate_bit_flip() {
    let kp = KeyPair::generate();
    let (original_valid, tampered_valid) = demonstrate_bit_flip(&kp, b"sensitive data");
    assert!(original_valid, "original message should verify");
    assert!(!tampered_valid, "tampered message should NOT verify");
}

#[test]
fn test_single_byte_change_invalidates() {
    let kp = KeyPair::generate();
    let signed = kp.sign(b"ABCDEFGHIJ");

    // Change one byte in the middle
    let mut tampered = b"ABCDEFGHIJ".to_vec();
    tampered[5] = b'X';

    assert!(!verify_signature(&tampered, &signed.signature, kp.verifying_key()));
}

// ============================================================================
// TRANSACTION TESTS
// ============================================================================

#[test]
fn test_transaction_creation() {
    let tx = Transaction::new("Alice".into(), "Bob".into(), 100, 1);
    assert_eq!(tx.from, "Alice");
    assert_eq!(tx.to, "Bob");
    assert_eq!(tx.amount, 100);
    assert_eq!(tx.nonce, 1);
}

#[test]
fn test_transaction_hash_deterministic() {
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let hash1 = tx.hash_hex();
    let hash2 = tx.hash_hex();
    assert_eq!(hash1, hash2);
    // SHA-256 produces 64 hex chars
    assert_eq!(hash1.len(), 64);
}

#[test]
fn test_different_transactions_different_hashes() {
    let tx1 = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let tx2 = Transaction::new("Alice".into(), "Bob".into(), 51, 1);
    assert_ne!(tx1.hash_hex(), tx2.hash_hex());
}

#[test]
fn test_transaction_to_bytes() {
    let tx = Transaction::new("Alice".into(), "Bob".into(), 100, 1);
    let bytes = tx.to_bytes();
    // Should contain "AliceBob1001"
    assert_eq!(String::from_utf8(bytes).unwrap(), "AliceBob1001");
}

// ============================================================================
// SIGNED TRANSACTION TESTS
// ============================================================================

#[test]
fn test_sign_transaction() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let signed_tx = sign_transaction(&tx, &kp);

    assert_eq!(signed_tx.transaction.from, "Alice");
    assert_eq!(signed_tx.transaction.to, "Bob");
    assert_eq!(signed_tx.transaction.amount, 50);
    assert!(!signed_tx.hash.is_empty());
    assert!(!signed_tx.signature.is_empty());
}

#[test]
fn test_verify_signed_transaction() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let signed_tx = sign_transaction(&tx, &kp);

    assert!(verify_transaction(&signed_tx, kp.verifying_key()));
}

#[test]
fn test_reject_tampered_transaction_amount() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let mut signed_tx = sign_transaction(&tx, &kp);

    // Attacker modifies the amount
    signed_tx.transaction.amount = 500;

    assert!(!verify_transaction(&signed_tx, kp.verifying_key()));
}

#[test]
fn test_reject_tampered_transaction_recipient() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let mut signed_tx = sign_transaction(&tx, &kp);

    // Attacker changes the recipient
    signed_tx.transaction.to = "Mallory".into();

    assert!(!verify_transaction(&signed_tx, kp.verifying_key()));
}

#[test]
fn test_reject_transaction_wrong_key() {
    let kp1 = KeyPair::generate();
    let kp2 = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let signed_tx = sign_transaction(&tx, &kp1);

    // Verify with wrong key
    assert!(!verify_transaction(&signed_tx, kp2.verifying_key()));
}

#[test]
fn test_transaction_hash_matches_signed() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let signed_tx = sign_transaction(&tx, &kp);

    // The hash stored in signed_tx should match recomputing from transaction
    assert_eq!(signed_tx.hash, signed_tx.transaction.hash_hex());
}

#[test]
fn test_double_spend_protection() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let signed_tx = sign_transaction(&tx, &kp);

    // Attacker creates a different transaction but tries to reuse the signature
    let modified_tx = Transaction::new("Alice".into(), "Bob".into(), 500, 1);
    let fake_signed = SignedTransaction {
        transaction: modified_tx,
        hash: signed_tx.hash.clone(),
        signature: signed_tx.signature.clone(),
    };

    assert!(!verify_transaction(&fake_signed, kp.verifying_key()));
}

// ============================================================================
// UTILITY TESTS
// ============================================================================

#[test]
fn test_sha256_hex() {
    let hash = sha256_hex(b"hello");
    // Known SHA-256 of "hello"
    assert_eq!(
        hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha256_hex_empty() {
    let hash = sha256_hex(b"");
    // Known SHA-256 of empty string
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_nonce_prevents_replay() {
    let kp = KeyPair::generate();
    let tx1 = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let tx2 = Transaction::new("Alice".into(), "Bob".into(), 50, 2); // different nonce

    let signed1 = sign_transaction(&tx1, &kp);
    let signed2 = sign_transaction(&tx2, &kp);

    // Both should verify individually
    assert!(verify_transaction(&signed1, kp.verifying_key()));
    assert!(verify_transaction(&signed2, kp.verifying_key()));

    // But signatures should differ (different nonces produce different hashes)
    assert_ne!(signed1.signature, signed2.signature);
}

#[test]
fn test_invalid_signature_hex_rejected() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let mut signed_tx = sign_transaction(&tx, &kp);

    // Corrupt the signature hex
    signed_tx.signature = "not_valid_hex_gg".to_string();
    assert!(!verify_transaction(&signed_tx, kp.verifying_key()));
}

#[test]
fn test_truncated_signature_rejected() {
    let kp = KeyPair::generate();
    let tx = Transaction::new("Alice".into(), "Bob".into(), 50, 1);
    let mut signed_tx = sign_transaction(&tx, &kp);

    // Truncate the signature
    signed_tx.signature = signed_tx.signature[..10].to_string();
    assert!(!verify_transaction(&signed_tx, kp.verifying_key()));
}
