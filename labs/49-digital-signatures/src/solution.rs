// Lab 49: Digital Signatures
//
// Implements cryptographic digital signatures using secp256k1 (ECDSA).
// Demonstrates public-key cryptography, signing, verification, and
// blockchain transaction signing use cases.
//
// Key concepts:
// - Asymmetric cryptography: private key signs, public key verifies
// - SHA-256 hashing before signing (standard practice)
// - Signature verification prevents tampering and fraud
// - secp256k1 is the curve used by Bitcoin and Ethereum

use k256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

// ============================================================================
// KEY PAIR
// ============================================================================

/// A secp256k1 key pair for signing and verification.
///
/// Ownership: KeyPair owns both the signing key (private) and verifying key (public).
/// The signing key must remain secret; the verifying key can be shared freely.
pub struct KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl KeyPair {
    /// Generate a new random key pair using OS-level secure randomness.
    pub fn generate() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = *signing_key.verifying_key();
        KeyPair {
            signing_key,
            verifying_key,
        }
    }

    /// Create a KeyPair from an existing SigningKey.
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        let verifying_key = *signing_key.verifying_key();
        KeyPair {
            signing_key,
            verifying_key,
        }
    }

    /// Return the public (verifying) key as a hex-encoded compressed point.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_encoded_point(true).as_bytes())
    }

    /// Return a reference to the verifying key.
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Sign a message (hashes with SHA-256 first, then signs the hash).
    ///
    /// Returns the raw signature bytes as a hex string and the Signature object.
    pub fn sign(&self, message: &[u8]) -> SignedMessage {
        let hash = Sha256::digest(message);
        let signature: Signature = self.signing_key.sign(&hash);
        SignedMessage {
            message: message.to_vec(),
            signature,
            signer_public_key: self.public_key_hex(),
        }
    }
}

// ============================================================================
// SIGNED MESSAGE
// ============================================================================

/// A message paired with its cryptographic signature.
pub struct SignedMessage {
    pub message: Vec<u8>,
    pub signature: Signature,
    pub signer_public_key: String,
}

impl SignedMessage {
    /// Return the signature encoded in DER format as hex.
    pub fn signature_hex(&self) -> String {
        hex::encode(self.signature.to_der().as_bytes())
    }
}

// ============================================================================
// VERIFICATION
// ============================================================================

/// Verify a signed message against a verifying (public) key.
///
/// Returns true if the signature is valid for the given message and key.
pub fn verify_signature(message: &[u8], signature: &Signature, verifying_key: &VerifyingKey) -> bool {
    let hash = Sha256::digest(message);
    verifying_key.verify(&hash, signature).is_ok()
}

/// Verify a SignedMessage given the expected verifying key.
pub fn verify_signed_message(signed: &SignedMessage, verifying_key: &VerifyingKey) -> bool {
    verify_signature(&signed.message, &signed.signature, verifying_key)
}

// ============================================================================
// BLOCKCHAIN TRANSACTION
// ============================================================================

/// A simplified blockchain transaction for signing demonstration.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
}

impl Transaction {
    /// Create a new transaction.
    pub fn new(from: String, to: String, amount: u64, nonce: u64) -> Self {
        Transaction { from, to, amount, nonce }
    }

    /// Serialize the transaction to bytes for hashing/signing.
    pub fn to_bytes(&self) -> Vec<u8> {
        format!("{}{}{}{}", self.from, self.to, self.amount, self.nonce).into_bytes()
    }

    /// Compute the SHA-256 hash of the transaction.
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes());
        hasher.finalize().to_vec()
    }

    /// Compute the hash as a hex string.
    pub fn hash_hex(&self) -> String {
        hex::encode(self.hash())
    }
}

/// A transaction that has been cryptographically signed.
#[derive(Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub hash: String,
    pub signature: String, // DER-encoded hex
}

/// Sign a transaction with the given key pair.
///
/// The transaction data is hashed with SHA-256, then the hash is signed
/// using the secp256k1 private key.
pub fn sign_transaction(tx: &Transaction, key_pair: &KeyPair) -> SignedTransaction {
    let tx_hash = tx.hash();
    let signature: Signature = key_pair.signing_key.sign(&tx_hash);

    SignedTransaction {
        transaction: tx.clone(),
        hash: hex::encode(&tx_hash),
        signature: hex::encode(signature.to_der().as_bytes()),
    }
}

/// Verify a signed transaction against a verifying key.
///
/// Re-computes the transaction hash and checks the signature.
pub fn verify_transaction(signed_tx: &SignedTransaction, verifying_key: &VerifyingKey) -> bool {
    // Recompute the hash from transaction data
    let tx_hash = signed_tx.transaction.hash();

    // Decode the stored signature
    let sig_bytes = match hex::decode(&signed_tx.signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let signature = match Signature::from_der(&sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    // Verify
    verifying_key.verify(&tx_hash, &signature).is_ok()
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Hash arbitrary data with SHA-256 and return hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    hex::encode(Sha256::digest(data))
}

/// Demonstrate that flipping a single bit in a message invalidates the signature.
/// Returns (original_valid, tampered_valid).
pub fn demonstrate_bit_flip(key_pair: &KeyPair, message: &[u8]) -> (bool, bool) {
    let signed = key_pair.sign(message);
    let original_valid = verify_signed_message(&signed, key_pair.verifying_key());

    // Tamper: flip one bit in the last byte
    let mut tampered = message.to_vec();
    if let Some(last) = tampered.last_mut() {
        *last ^= 0x01;
    }

    let tampered_valid = verify_signature(&tampered, &signed.signature, key_pair.verifying_key());

    (original_valid, tampered_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate();
        assert!(!kp.public_key_hex().is_empty());
    }

    #[test]
    fn test_sign_and_verify() {
        let kp = KeyPair::generate();
        let signed = kp.sign(b"hello world");
        assert!(verify_signed_message(&signed, kp.verifying_key()));
    }

    #[test]
    fn test_tampered_message_rejected() {
        let kp = KeyPair::generate();
        let (original, tampered) = demonstrate_bit_flip(&kp, b"secure data");
        assert!(original);
        assert!(!tampered);
    }
}
