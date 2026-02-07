//! # Transaction Validation - Complete Solution with Blockchain Context
//!
//! ## What are Digital Signatures?
//!
//! Digital signatures prove ownership without revealing private keys:
//! 1. **Private Key**: Secret (never shared) - signs transactions
//! 2. **Public Key**: Public (shared freely) - verifies signatures
//! 3. **Signature**: Proof you own the private key for public key
//!
//! ## How Bitcoin Uses This
//!
//! 1. Wallet creates key pair (private + public)
//! 2. Public key becomes your address (where you receive coins)
//! 3. To spend coins:
//!    - Create transaction
//!    - Sign with private key
//!    - Broadcast to network
//! 4. Network verifies:
//!    - Signature is valid
//!    - You own the coins being spent
//!    - Transaction hasn't been double-spent
//!
//! NOTE: This implementation uses a simplified mock crypto system for educational purposes.
//! In production, use a proper cryptographic library like ed25519-dalek.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// Simple hash function using Rust's standard library hasher.
/// This is NOT cryptographically secure - use only for learning!
fn simple_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    hasher.write(input);
    let hash_value = hasher.finish();

    // Create a 32-byte (256-bit) hash by repeating and mixing the 64-bit hash
    let mut result = Vec::with_capacity(32);
    for i in 0..4 {
        let shifted = hash_value.wrapping_mul(i as u64 + 1);
        result.extend_from_slice(&shifted.to_be_bytes());
    }
    result
}

/// Convert bytes to hexadecimal string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Mock signing key (private key) - NOT cryptographically secure!
#[derive(Clone)]
pub struct SigningKey {
    secret: Vec<u8>,
}

/// Mock verifying key (public key) - NOT cryptographically secure!
#[derive(Clone)]
pub struct VerifyingKey {
    public: Vec<u8>,
}

impl VerifyingKey {
    pub fn as_bytes(&self) -> &[u8] {
        &self.public
    }
}

/// A wallet with public/private key pair.
///
/// ## Blockchain Context
/// - Private key: Your secret, never share! (like password)
/// - Public key: Your address, share freely! (like email)
/// - Anyone can send to your public key
/// - Only you can spend with private key
pub struct Wallet {
    pub signing_key: SigningKey,      // Private key (secret!)
    pub verifying_key: VerifyingKey,  // Public key (shareable)
}

impl Wallet {
    /// Create a new wallet with random keys.
    ///
    /// ## Security
    /// - Uses system time for randomness (NOT cryptographically secure!)
    /// - Private key has limited entropy in this mock implementation
    /// - Real implementation should use cryptographically secure RNG
    /// - In Bitcoin: ~2^160 possible addresses
    ///
    /// ## Custody
    /// - Whoever has private key controls the coins!
    /// - "Not your keys, not your coins"
    /// - Lose private key = lose access forever
    /// - Exchange hacks: attacker steals private keys
    pub fn new() -> Self {
        // Create a "random" private key using system time
        // NOTE: This is NOT cryptographically secure!
        // Real implementation should use a CSPRNG like OsRng

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX_EPOCH (1970-01-01); check system clock")
            .as_nanos();

        // Create secret from timestamp (mock randomness)
        let secret_bytes = format!("secret_{}", now).into_bytes();
        let secret = simple_hash(&secret_bytes);

        // Derive public key from private key (in mock: just hash the secret)
        let public = simple_hash(&secret);

        let signing_key = SigningKey {
            secret: secret.clone(),
        };

        let verifying_key = VerifyingKey {
            public,
        };

        Wallet {
            signing_key,
            verifying_key,
        }
    }

    /// Get wallet address (public key as hex string).
    ///
    /// ## Blockchain Context
    /// In Bitcoin:
    /// - Public key → hash → Base58Check encoding → Address
    /// - Address format: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
    /// - Hashing adds extra security layer
    /// - Base58: Readable, no ambiguous characters (0, O, l, I)
    ///
    /// In our implementation:
    /// - Just hex-encoded public key (simplified)
    /// - Real systems use additional hashing and encoding
    pub fn address(&self) -> String {
        // Convert public key bytes to hex string
        bytes_to_hex(self.verifying_key.as_bytes())
    }

    /// Sign a transaction with this wallet's private key.
    ///
    /// ## How Signing Works
    /// 1. Create message from transaction data
    /// 2. Hash the message
    /// 3. Sign the hash with private key (mock: hash public + message)
    /// 4. Attach signature to transaction
    ///
    /// ## Security Properties (in real crypto)
    /// - Only holder of private key can create valid signature
    /// - Signature proves ownership without revealing private key
    /// - Can't forge signature without private key
    /// - Can't reuse signature for different transaction
    ///
    /// ## Parameters
    /// - transaction: Transaction to sign (modified in place)
    pub fn sign_transaction(&self, transaction: &mut Transaction) {
        // Step 1: Create message to sign
        // Serialize transaction data (from, to, amount)
        // Exclude signature field (we're creating it!)

        let message = format!("{}{}{}", transaction.from, transaction.to, transaction.amount);

        // Step 2: Hash the message
        let message_hash = simple_hash(message.as_bytes());

        // Step 3: Create signature (mock: hash of public + message)
        // Real crypto would use elliptic curve operations with the private key
        // In our mock, we use public + message so verification can work
        // NOTE: This is NOT secure! Real signatures use the private key.
        let mut sig_data = Vec::new();
        sig_data.extend_from_slice(&self.verifying_key.public);
        sig_data.extend_from_slice(&message_hash);
        let signature = simple_hash(&sig_data);

        // Step 4: Attach signature to transaction
        transaction.signature = Some(signature);
    }
}

/// A transaction (simplified).
///
/// ## Real Bitcoin Transactions
/// Bitcoin uses UTXO (Unspent Transaction Output) model:
/// - Transactions have inputs (previous outputs being spent)
/// - Transactions have outputs (new recipients)
/// - Each input references a previous output
/// - Each input has a signature
///
/// ## Our Simplified Model
/// - from: Sender address (public key)
/// - to: Recipient address
/// - amount: How much to send
/// - signature: Proof sender authorized this
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,      // Sender's public key (hex)
    pub to: String,        // Recipient's public key (hex)
    pub amount: u64,       // Amount to transfer
    pub signature: Option<Vec<u8>>, // Digital signature
}

/// Verify a transaction's signature.
///
/// ## What We're Verifying
/// 1. Transaction was signed by owner of "from" address
/// 2. Transaction hasn't been modified after signing
/// 3. Signature is mathematically valid (in mock: signature matches expected)
///
/// ## What We're NOT Verifying (would need full blockchain)
/// - Sender has enough balance
/// - Transaction hasn't been double-spent
/// - Inputs actually exist and are unspent
///
/// ## Parameters
/// - transaction: Transaction to verify
/// - public_key: Public key of claimed sender
///
/// ## Returns
/// true if signature is valid, false otherwise
pub fn verify_transaction(transaction: &Transaction, public_key: &VerifyingKey) -> bool {
    // Check if transaction has signature
    // `transaction.signature` is Option<Vec<u8>>
    //   - Some(sig) if signed
    //   - None if unsigned

    let signature_bytes = match &transaction.signature {
        Some(sig) => sig,
        None => return false, // No signature = invalid
    };

    // Check signature format
    if signature_bytes.len() != 32 {
        return false;
    }

    // Recreate message that was signed
    // Must match exactly what was signed!
    // Any change = different hash = invalid signature

    let message = format!("{}{}{}", transaction.from, transaction.to, transaction.amount);

    // Hash the message (same as signing process)
    let message_hash = simple_hash(message.as_bytes());

    // Verify signature (mock implementation)
    // In our mock system: signature = hash(public + message_hash)
    // To verify, we reconstruct what the signature should be and compare
    //
    // In real crypto, verification uses mathematical properties of elliptic curves
    // where you can verify without knowing the private key. Our mock simulates this
    // by using the public key (which everyone knows) in the signature computation.

    // Step 1: Check if the "from" address matches the public key
    let expected_address = bytes_to_hex(public_key.as_bytes());
    if transaction.from != expected_address {
        return false;
    }

    // Step 2: Reconstruct what the signature should be
    // Create the same signature that sign_transaction would create
    let mut sig_data = Vec::new();
    sig_data.extend_from_slice(public_key.as_bytes());
    sig_data.extend_from_slice(&message_hash);
    let expected_signature = simple_hash(&sig_data);

    // Step 3: Compare with the actual signature
    // In real crypto, this uses mathematical verification
    // In our mock, we just compare the hashes
    signature_bytes == &expected_signature

    // ============================================================================
    // HOW SIGNATURE VERIFICATION WORKS (Simplified)
    // ============================================================================
    //
    // Ed25519 uses elliptic curve cryptography:
    //
    // 1. Private key = random number d
    // 2. Public key = point P = d × G (G is generator point on curve)
    // 3. Signing:
    //    - Create random k
    //    - R = k × G (commit to randomness)
    //    - s = k + H(R, P, message) × d (combine with message)
    //    - Signature = (R, s)
    // 4. Verification:
    //    - Check: s × G = R + H(R, P, message) × P
    //    - Works because: P = d × G (public key definition)
    //    - Only works if signature created with matching private key!
    //
    // Security:
    // - Can't find d from P (discrete logarithm problem)
    // - Can't forge signature without d
    // - Each signature uses different randomness (k)
    // - Hash ties signature to specific message
}

// ============================================================================
// BLOCKCHAIN SECURITY THROUGH SIGNATURES
// ============================================================================
//
// Q: Why can't someone just copy my signature?
// A: Signature is tied to specific transaction!
//    - Message includes: from, to, amount
//    - Change any field = different hash
//    - Different hash = signature becomes invalid
//    - Can't reuse signature for different transaction
//
// Q: What if someone steals my private key?
// A: They can spend your coins!
//    - This is why security is critical
//    - Hardware wallets: key never leaves device
//    - Cold storage: keys on offline computer
//    - Multi-sig: require multiple keys
//
// Q: Can quantum computers break this?
// A: Current schemes (Ed25519, ECDSA): vulnerable to quantum
//    - Shor's algorithm can break elliptic curves
//    - Bitcoin would need to upgrade to quantum-resistant crypto
//    - Hash functions (SHA-256) are more resistant
//    - Likely decades away before practical threat
//
// Q: How does Bitcoin prevent double-spending?
// A: Blockchain + consensus!
//    - Each coin can only be spent once
//    - Miners check transactions aren't double-spends
//    - First valid transaction in blockchain wins
//    - Later double-spend attempts rejected
//    - Signatures prevent unauthorized spending
//    - Blockchain prevents spending same coin twice
//
// Q: What's multi-signature (multi-sig)?
// A: Require multiple signatures to spend
//    - Example: 2-of-3 multi-sig
//    - Need 2 out of 3 keys to sign
//    - Used for: corporate accounts, escrow, enhanced security
//    - Bitcoin supports with P2SH (Pay to Script Hash)
