// Project 46: Digital Signatures (CAPSTONE)
//
// Implements cryptographic digital signatures using Ed25519 and secp256k1.
// Demonstrates public-key cryptography, signing, verification, and blockchain use cases.

use ed25519_dalek::{Keypair as Ed25519Keypair, PublicKey as Ed25519PublicKey,
                    Signature as Ed25519Signature, Signer, Verifier};
use k256::ecdsa::{SigningKey, VerifyingKey, Signature as Secp256k1Signature,
                  signature::{Signer as K256Signer, Verifier as K256Verifier}};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use std::time::Instant;

fn main() {
    println!("=== Digital Signatures Demonstration ===\n");

    // Part 1: Ed25519 Signatures (Modern, Fast)
    demonstrate_ed25519();

    // Part 2: secp256k1 Signatures (Bitcoin/Ethereum)
    demonstrate_secp256k1();

    // Part 3: Performance Comparison
    performance_comparison();

    // Part 4: Blockchain Use Case
    blockchain_transaction_signing();
}

// ============================================================================
// ED25519 SIGNATURES
// ============================================================================

fn demonstrate_ed25519() {
    println!("--- Ed25519 Signatures ---");

    // Generate keypair
    let mut csprng = OsRng{};
    let keypair = Ed25519Keypair::generate(&mut csprng);

    println!("Generated Ed25519 keypair");
    println!("Public key: {}", hex::encode(keypair.public.as_bytes()));
    println!();

    // Sign a message
    let message = b"Hello, Blockchain!";
    println!("Signing message: {:?}", String::from_utf8_lossy(message));

    let signature = keypair.sign(message);
    println!("Signature: {}", hex::encode(signature.to_bytes()));
    println!();

    // Verify signature
    match keypair.public.verify(message, &signature) {
        Ok(_) => println!("✅ Signature verified successfully!"),
        Err(_) => println!("❌ Signature verification failed!"),
    }
    println!();

    // Try to verify with tampered message
    let tampered_message = b"Hello, Blockchain?";
    println!("Verifying tampered message: {:?}", String::from_utf8_lossy(tampered_message));
    match keypair.public.verify(tampered_message, &signature) {
        Ok(_) => println!("❌ Tampered message verified (this should not happen!)"),
        Err(_) => println!("✅ Signature verification failed (as expected)"),
    }
    println!();

    // Demonstrate deterministic signatures
    demonstrate_deterministic_signatures(&keypair, message);
}

fn demonstrate_deterministic_signatures(keypair: &Ed25519Keypair, message: &[u8]) {
    println!("--- Deterministic Signature Property ---");

    // Ed25519 is deterministic - same message produces same signature
    let sig1 = keypair.sign(message);
    let sig2 = keypair.sign(message);

    if sig1.to_bytes() == sig2.to_bytes() {
        println!("✅ Ed25519 signatures are deterministic");
        println!("   Same message always produces same signature");
    } else {
        println!("❌ Signatures differ (unexpected!)");
    }
    println!();
}

// ============================================================================
// SECP256K1 SIGNATURES (Bitcoin/Ethereum)
// ============================================================================

fn demonstrate_secp256k1() {
    println!("--- secp256k1 Signatures (Bitcoin-style) ---");

    // Generate keypair
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    println!("Generated secp256k1 keypair");
    println!("Public key: {}", hex::encode(verifying_key.to_encoded_point(true).as_bytes()));
    println!();

    // Create a transaction-like message
    let transaction = b"Alice sends 10 BTC to Bob";
    println!("Signing transaction: {:?}", String::from_utf8_lossy(transaction));

    // Hash the message (standard practice)
    let message_hash = Sha256::digest(transaction);

    // Sign the hash
    let signature: Secp256k1Signature = signing_key.sign(&message_hash);
    println!("Signature (DER): {}", hex::encode(signature.to_der().as_bytes()));
    println!();

    // Verify signature
    match verifying_key.verify(&message_hash, &signature) {
        Ok(_) => println!("✅ Signature verified successfully!"),
        Err(_) => println!("❌ Signature verification failed!"),
    }
    println!();

    // Demonstrate signature malleability protection
    demonstrate_signature_properties(&signing_key, transaction);
}

fn demonstrate_signature_properties(signing_key: &SigningKey, message: &[u8]) {
    println!("--- Signature Properties ---");

    let message_hash = Sha256::digest(message);
    let signature: Secp256k1Signature = signing_key.sign(&message_hash);

    // Show that changing even one bit in message invalidates signature
    let mut tampered = message.to_vec();
    if let Some(last) = tampered.last_mut() {
        *last ^= 0x01; // Flip one bit
    }

    let tampered_hash = Sha256::digest(&tampered);
    match signing_key.verifying_key().verify(&tampered_hash, &signature) {
        Ok(_) => println!("❌ Tampered message verified (should not happen!)"),
        Err(_) => println!("✅ Single bit change invalidates signature"),
    }
    println!();
}

// ============================================================================
// PERFORMANCE COMPARISON
// ============================================================================

fn performance_comparison() {
    println!("--- Performance Comparison ---");

    const ITERATIONS: usize = 1000;
    let message = b"Performance test message for digital signatures";

    // Ed25519 performance
    let mut csprng = OsRng{};
    let ed25519_keypair = Ed25519Keypair::generate(&mut csprng);

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = ed25519_keypair.sign(message);
    }
    let ed25519_sign_time = start.elapsed();

    let signature = ed25519_keypair.sign(message);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = ed25519_keypair.public.verify(message, &signature);
    }
    let ed25519_verify_time = start.elapsed();

    // secp256k1 performance
    let secp256k1_key = SigningKey::random(&mut OsRng);
    let message_hash = Sha256::digest(message);

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _: Secp256k1Signature = secp256k1_key.sign(&message_hash);
    }
    let secp256k1_sign_time = start.elapsed();

    let sig: Secp256k1Signature = secp256k1_key.sign(&message_hash);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = secp256k1_key.verifying_key().verify(&message_hash, &sig);
    }
    let secp256k1_verify_time = start.elapsed();

    // Print results
    println!("Ed25519:");
    println!("  Signing:   {} signatures in {:?}", ITERATIONS, ed25519_sign_time);
    println!("  Verifying: {} verifications in {:?}", ITERATIONS, ed25519_verify_time);
    println!();
    println!("secp256k1:");
    println!("  Signing:   {} signatures in {:?}", ITERATIONS, secp256k1_sign_time);
    println!("  Verifying: {} verifications in {:?}", ITERATIONS, secp256k1_verify_time);
    println!();

    // Analysis
    println!("Analysis:");
    println!("  Ed25519 is typically 3-8x faster than secp256k1");
    println!("  Both provide 128-bit security level");
    println!("  Ed25519: Modern choice for new systems");
    println!("  secp256k1: Standard for Bitcoin/Ethereum compatibility");
    println!();
}

// ============================================================================
// BLOCKCHAIN USE CASE
// ============================================================================

fn blockchain_transaction_signing() {
    println!("--- Blockchain Transaction Signing ---");

    // Simulate a blockchain transaction
    let tx = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50,
        nonce: 1,
    };

    println!("Transaction:");
    println!("  From: {}", tx.from);
    println!("  To: {}", tx.to);
    println!("  Amount: {} coins", tx.amount);
    println!("  Nonce: {}", tx.nonce);
    println!();

    // Sign with secp256k1 (like Bitcoin/Ethereum)
    let signing_key = SigningKey::random(&mut OsRng);
    let signed_tx = sign_transaction(&tx, &signing_key);

    println!("Signed transaction hash: {}", signed_tx.hash);
    println!("Signature: {}", signed_tx.signature);
    println!();

    // Verify the transaction
    if verify_transaction(&signed_tx, signing_key.verifying_key()) {
        println!("✅ Transaction signature is valid!");
        println!("   Network would accept this transaction");
    } else {
        println!("❌ Transaction signature is invalid!");
        println!("   Network would reject this transaction");
    }
    println!();

    // Demonstrate double-spend protection
    demonstrate_double_spend_protection(&tx, &signing_key);
}

#[derive(Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    nonce: u64,
}

struct SignedTransaction {
    transaction: Transaction,
    hash: String,
    signature: String,
}

fn sign_transaction(tx: &Transaction, signing_key: &SigningKey) -> SignedTransaction {
    // Serialize transaction (in production, use proper serialization)
    let tx_data = format!("{}{}{}{}", tx.from, tx.to, tx.amount, tx.nonce);

    // Hash the transaction
    let mut hasher = Sha256::new();
    hasher.update(tx_data.as_bytes());
    let tx_hash = hasher.finalize();

    // Sign the hash
    let signature: Secp256k1Signature = signing_key.sign(&tx_hash);

    SignedTransaction {
        transaction: Transaction {
            from: tx.from.clone(),
            to: tx.to.clone(),
            amount: tx.amount,
            nonce: tx.nonce,
        },
        hash: hex::encode(tx_hash),
        signature: hex::encode(signature.to_der().as_bytes()),
    }
}

fn verify_transaction(signed_tx: &SignedTransaction, verifying_key: &VerifyingKey) -> bool {
    // Recreate the transaction hash
    let tx = &signed_tx.transaction;
    let tx_data = format!("{}{}{}{}", tx.from, tx.to, tx.amount, tx.nonce);

    let mut hasher = Sha256::new();
    hasher.update(tx_data.as_bytes());
    let tx_hash = hasher.finalize();

    // Decode signature
    let sig_bytes = hex::decode(&signed_tx.signature).expect("Invalid hex");
    let signature = Secp256k1Signature::from_der(&sig_bytes).expect("Invalid signature");

    // Verify
    verifying_key.verify(&tx_hash, &signature).is_ok()
}

fn demonstrate_double_spend_protection(tx: &Transaction, signing_key: &SigningKey) {
    println!("--- Double-Spend Protection ---");

    // Create a valid transaction
    let signed_tx1 = sign_transaction(tx, signing_key);

    // Try to modify the transaction (e.g., increase amount)
    let modified_tx = Transaction {
        from: tx.from.clone(),
        to: tx.to.clone(),
        amount: tx.amount * 10, // Attacker tries to increase amount!
        nonce: tx.nonce,
    };

    // Use the old signature (won't work!)
    let fake_signed_tx = SignedTransaction {
        transaction: modified_tx,
        hash: signed_tx1.hash.clone(),
        signature: signed_tx1.signature.clone(),
    };

    println!("Attacker tries to modify transaction amount...");
    if verify_transaction(&fake_signed_tx, signing_key.verifying_key()) {
        println!("❌ Modified transaction verified (security failure!)");
    } else {
        println!("✅ Modified transaction rejected!");
        println!("   Signature doesn't match modified transaction");
        println!("   Blockchain remains secure");
    }
    println!();
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. CRYPTOGRAPHIC OPERATIONS
//    Ed25519 and secp256k1 use carefully optimized assembly for critical operations.
//    Rust's zero-cost abstractions mean these are as fast as C implementations.
//    No garbage collection pauses during signing/verification (critical for performance).
//
// 2. RANDOM NUMBER GENERATION
//    OsRng uses the operating system's secure RNG (e.g., /dev/urandom on Linux).
//    This is cryptographically secure - unpredictable even to attackers.
//    Critical for key generation - weak RNG = stolen funds!
//
// 3. CONSTANT-TIME OPERATIONS
//    Signature verification uses constant-time comparisons to prevent timing attacks.
//    Variable-time comparisons leak information about the signature!
//    Rust's type system helps enforce this (but can't guarantee it).
//
// 4. MEMORY SAFETY
//    Private keys are sensitive data. Rust prevents:
//    - Buffer overflows that could leak keys
//    - Use-after-free that could expose key material
//    - Data races in concurrent signing operations
//
// 5. ZERO-COPY OPERATIONS
//    Signature verification doesn't copy the message - works on references.
//    This is efficient for large messages (e.g., blockchain blocks).
//    Ownership system ensures message isn't modified during verification.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Digital signatures prove message authenticity without revealing private key
// 2. Ed25519 is faster and simpler - good for new systems
// 3. secp256k1 is standard for Bitcoin/Ethereum - use for compatibility
// 4. Always hash messages before signing
// 5. Signatures are deterministic in Ed25519, can be in secp256k1 (RFC 6979)
// 6. Changing one bit in message invalidates signature
// 7. Private key = secret, Public key = shareable
// 8. Blockchain transactions are signed to prove ownership
// 9. Signature verification prevents double-spending and fraud
// 10. Rust's memory safety prevents common cryptographic implementation bugs

// ============================================================================
// SECURITY CONSIDERATIONS
// ============================================================================
// ✅ DO:
//    - Use cryptographically secure RNG (OsRng)
//    - Hash messages before signing
//    - Verify ALL signatures before trusting data
//    - Use deterministic signatures when possible
//    - Keep private keys secret and secure
//    - Use constant-time comparison for signatures
//
// ❌ DON'T:
//    - Reuse nonces/randomness (catastrophic for ECDSA!)
//    - Sign data without hashing first
//    - Compare signatures with == (timing attack)
//    - Store private keys in plaintext
//    - Use weak RNG (e.g., time-based seeds)
//    - Trust unsigned data in blockchain context

// ============================================================================
// BLOCKCHAIN INTEGRATION
// ============================================================================
// In a real blockchain:
// 1. User creates transaction with recipient address and amount
// 2. Transaction is serialized (e.g., using bincode or ProtoBuf)
// 3. Serialized data is hashed (SHA-256 or Keccak-256)
// 4. User signs hash with private key
// 5. Transaction + signature is broadcast to network
// 6. Each node verifies signature using sender's public key
// 7. Valid signatures prove sender owns the coins (UTXO model)
// 8. Invalid signatures are rejected immediately
//
// This enables trustless transactions - no central authority needed!

// ============================================================================
// PERFORMANCE NOTES
// ============================================================================
// Ed25519:
//   - Signing: ~15,000 signatures/second (single core)
//   - Verification: ~5,000 verifications/second
//   - Signature size: 64 bytes
//   - Public key: 32 bytes
//
// secp256k1:
//   - Signing: ~2,000 signatures/second (single core)
//   - Verification: ~1,000 verifications/second
//   - Signature size: 64-72 bytes (DER encoding)
//   - Public key: 33 bytes (compressed) or 65 bytes (uncompressed)
//
// Both are fast enough for most applications. Ed25519 wins on pure performance,
// but secp256k1 is required for Bitcoin/Ethereum compatibility.

// ============================================================================
// REAL-WORLD USAGE
// ============================================================================
// Bitcoin:
//   - Uses secp256k1 ECDSA
//   - Each input in a transaction has a signature
//   - Signatures prove ownership of UTXOs
//   - Schnorr signatures added in Taproot upgrade (2021)
//
// Ethereum:
//   - Uses secp256k1 ECDSA
//   - Recoverable signatures (can derive public key from signature)
//   - Account model instead of UTXO
//   - Planning to migrate to post-quantum signatures eventually
//
// Polkadot/Substrate:
//   - Uses Ed25519 and sr25519 (Schnorr on Ristretto25519)
//   - Faster and more modern than Bitcoin/Ethereum
//   - Better for high-throughput blockchains
//
// Signal (Messaging):
//   - Uses Ed25519 (X25519 for key exchange)
//   - Fast signature verification on mobile devices
//   - Part of the Double Ratchet protocol
