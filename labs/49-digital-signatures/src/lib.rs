//! Lab 49: Digital Signatures - Student API stubs.
//!
//! Implement the functions and methods below as part of the lab exercises.

use k256::ecdsa::{Signature, SigningKey, VerifyingKey};

pub struct KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        todo!("Generate a random secp256k1 signing/verifying keypair")
    }

    pub fn from_signing_key(_signing_key: SigningKey) -> Self {
        todo!("Construct KeyPair from an existing signing key")
    }

    pub fn public_key_hex(&self) -> String {
        let _ = self;
        todo!("Return compressed public key as hex")
    }

    pub fn verifying_key(&self) -> &VerifyingKey {
        let _ = self;
        todo!("Return reference to verifying key")
    }

    pub fn sign(&self, _message: &[u8]) -> SignedMessage {
        let _ = self;
        todo!("Hash message with SHA-256 and produce ECDSA signature")
    }
}

pub struct SignedMessage {
    pub message: Vec<u8>,
    pub signature: Signature,
    pub signer_public_key: String,
}

impl SignedMessage {
    pub fn signature_hex(&self) -> String {
        let _ = self;
        todo!("Encode DER signature as hex")
    }
}

pub fn verify_signature(_message: &[u8], _signature: &Signature, _verifying_key: &VerifyingKey) -> bool {
    todo!("Verify ECDSA signature over SHA-256 hash")
}

pub fn verify_signed_message(_signed: &SignedMessage, _verifying_key: &VerifyingKey) -> bool {
    todo!("Verify SignedMessage against verifying key")
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
}

impl Transaction {
    pub fn new(_from: String, _to: String, _amount: u64, _nonce: u64) -> Self {
        todo!("Construct a transaction value")
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let _ = self;
        todo!("Serialize transaction deterministically")
    }

    pub fn hash(&self) -> Vec<u8> {
        let _ = self;
        todo!("Compute SHA-256 hash of serialized transaction")
    }

    pub fn hash_hex(&self) -> String {
        let _ = self;
        todo!("Hex-encode transaction hash")
    }
}

#[derive(Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub hash: String,
    pub signature: String,
    pub signer_public_key: String,
}

pub fn sign_transaction(_tx: &Transaction, _key_pair: &KeyPair) -> SignedTransaction {
    todo!("Sign transaction hash and package signed transaction")
}

pub fn verify_transaction(_signed_tx: &SignedTransaction, _verifying_key: &VerifyingKey) -> bool {
    todo!("Verify signed transaction integrity and signature")
}

pub fn sha256_hex(_data: &[u8]) -> String {
    todo!("Compute SHA-256 hex digest")
}

pub fn demonstrate_bit_flip(_key_pair: &KeyPair, _message: &[u8]) -> (bool, bool) {
    todo!("Show that tiny message mutation invalidates signature")
}

#[doc(hidden)]
pub mod solution;
