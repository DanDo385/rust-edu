// Lab 50: Wallet CLI
//
// Complete cryptocurrency wallet with key management, UTXO selection,
// transaction construction, and signing. Integrates digital signatures
// and blockchain concepts into a practical application.
//
// ## Classroom Narrative
//
// 1. **Ownership landscape**: `Wallet` owns its master signing key, derived
// addresses, and UTXOs. The heap stores strings (`name`, `address`, `txid`)
// while the stack carries the struct's fields. Ownership flows from wallet
// to transactions when outputs are spent.
// 2. **Address derivation**: Each address is derived by hashing the public
// key plus index, producing a heap-owned string. We borrow the public key's
// byte slice without cloning so we never duplicate raw key material.
// 3. **UTXO selection & signing**: UTXOs live in a `HashMap<String, UTXO>` owned
// by the wallet. When constructing a transaction, we borrow these entries to
// sum inputs before creating a signed, owned transaction payload.
//
// ## Symbol Drill
//
// - `&self` returns shared borrows for inspection (`get_balance`, etc.).
//   The borrower gets an address into the wallet's heap data, not a clone.
// - `&mut self` is used when we mutate the wallet (adding addresses, receiving
//   funds). The borrow checker ensures these paths can't overlap with readers.
// - `*` is used when calculating fees/size and acts purely on numeric values
//   (counts/bytes), not pointer dereference.
//
// ## Step-by-step Teaching Breakdown
//
// 1. **Key generation**: `Wallet::new` randomly generates a signing key on the
//    stack, pushes it into the struct, and immediately derives an address.
// 2. **Address space**: `generate_address` borrows the verifying key, hashes it,
//    and stores the result as an owned `String`. Reusing `address_index` avoids
//    heap reallocations when deriving multiple addresses.
// 3. **UTXO selection**: Functions like `select_utxos_largest_first` iterate
//    borrowed UTXO references to accumulate enough funds before constructing a
//    new transaction. The transaction takes ownership of the selected UTXO
//    data (cloning identifiers) when injecting them into inputs.
// 4. **Transaction creation & signing**: `create_transaction` builds owned
//    inputs/outputs, computes fees (numeric arithmetic on stack values), signs
//    the final payload with `SigningKey`, and returns an owned `Transaction`.

use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// ============================================================================
// WALLET
// ============================================================================

/// A cryptocurrency wallet managing keys, addresses, and UTXOs.
///
/// Ownership: The Wallet owns its master signing key, all derived addresses,
/// and the set of unspent transaction outputs (UTXOs).
pub struct Wallet {
    pub name: String,
    master_key: SigningKey,
    addresses: Vec<WalletAddress>,
    utxos: HashMap<String, UTXO>,
    address_index: u32,
}

/// A derived wallet address with its public key and usage status.
#[derive(Debug, Clone)]
pub struct WalletAddress {
    pub address: String,
    pub public_key: String,
    pub index: u32,
    pub used: bool,
}

/// An unspent transaction output (UTXO) tracked by the wallet.
#[derive(Debug, Clone)]
pub struct UTXO {
    pub txid: String,
    pub vout: u32,
    pub amount: u64, // in satoshis
    pub address: String,
    pub confirmations: u32,
}

/// Errors that can occur during wallet operations.
#[derive(Debug, PartialEq)]
pub enum WalletError {
    InsufficientFunds,
    InvalidAddress,
    FeeTooHigh,
    SigningFailed,
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletError::InsufficientFunds => write!(f, "Insufficient funds"),
            WalletError::InvalidAddress => write!(f, "Invalid address"),
            WalletError::FeeTooHigh => write!(f, "Fee too high"),
            WalletError::SigningFailed => write!(f, "Signing failed"),
        }
    }
}

impl Wallet {
    /// Create a new wallet with a randomly generated master key.
    pub fn new(name: String) -> Self {
        let master_key = SigningKey::random(&mut OsRng);
        let mut wallet = Wallet {
            name,
            master_key,
            addresses: Vec::new(),
            utxos: HashMap::new(),
            address_index: 0,
        };
        // Generate initial address
        wallet.generate_address();
        wallet
    }

    /// Create a wallet from an existing signing key (for deterministic testing).
    pub fn from_key(name: String, master_key: SigningKey) -> Self {
        let mut wallet = Wallet {
            name,
            master_key,
            addresses: Vec::new(),
            utxos: HashMap::new(),
            address_index: 0,
        };
        wallet.generate_address();
        wallet
    }

    /// Derive and register a new address from the master key.
    ///
    /// In a real wallet, this would use BIP32 hierarchical deterministic derivation.
    /// Here we simplify by hashing public_key + index.
    pub fn generate_address(&mut self) -> String {
        let pub_hex = hex::encode(
            self.master_key
                .verifying_key()
                .to_encoded_point(true)
                .as_bytes(),
        );
        let address_data = format!("{}:{}", pub_hex, self.address_index);

        let mut hasher = Sha256::new();
        hasher.update(address_data.as_bytes());
        let hash = hasher.finalize();

        let address = format!("bc1q{}", hex::encode(&hash[..20]));

        self.addresses.push(WalletAddress {
            address: address.clone(),
            public_key: pub_hex,
            index: self.address_index,
            used: false,
        });

        self.address_index += 1;
        address
    }

    /// Return the first (root) address.
    pub fn get_root_address(&self) -> String {
        self.addresses[0].address.clone()
    }

    /// Check if an address belongs to this wallet.
    pub fn is_my_address(&self, address: &str) -> bool {
        self.addresses.iter().any(|a| a.address == address)
    }

    /// Record receiving funds as a new UTXO.
    pub fn receive_funds(&mut self, txid: String, vout: u32, amount: u64, address: String) {
        let utxo = UTXO {
            txid: txid.clone(),
            vout,
            amount,
            address,
            confirmations: 6,
        };
        let key = format!("{}:{}", txid, vout);
        self.utxos.insert(key, utxo);
    }

    /// Get the total wallet balance (sum of all UTXOs).
    pub fn get_balance(&self) -> u64 {
        self.utxos.values().map(|u| u.amount).sum()
    }

    /// Return the number of UTXOs.
    pub fn utxo_count(&self) -> usize {
        self.utxos.len()
    }

    /// Return the number of addresses.
    pub fn address_count(&self) -> usize {
        self.addresses.len()
    }

    /// Create and sign a transaction sending `amount` satoshis to `recipient`.
    pub fn create_transaction(
        &self,
        recipient: String,
        amount: u64,
        fee_rate: u64,
    ) -> Result<Transaction, WalletError> {
        if amount > self.get_balance() {
            return Err(WalletError::InsufficientFunds);
        }

        // Select UTXOs using largest-first strategy
        let selected_utxos = self.select_utxos_largest_first(amount, fee_rate)?;

        let total_input: u64 = selected_utxos.iter().map(|u| u.amount).sum();
        let estimated_size = estimate_tx_size(selected_utxos.len(), 2);
        let fee = estimated_size * fee_rate;

        if amount + fee > total_input {
            return Err(WalletError::InsufficientFunds);
        }

        let change = total_input - amount - fee;

        // Build inputs
        let inputs: Vec<TxInput> = selected_utxos
            .iter()
            .map(|utxo| TxInput {
                txid: utxo.txid.clone(),
                vout: utxo.vout,
                amount: utxo.amount,
                signature: String::new(),
            })
            .collect();

        // Build outputs
        let mut outputs = vec![TxOutput {
            address: recipient,
            amount,
        }];

        // Add change output if above dust threshold (546 satoshis)
        if change > DUST_THRESHOLD {
            let change_address = self.addresses[0].address.clone();
            outputs.push(TxOutput {
                address: change_address,
                amount: change,
            });
        }

        let mut tx = Transaction {
            txid: String::new(),
            inputs,
            outputs,
            fee,
            size: estimated_size,
        };

        // Sign the transaction
        self.sign_transaction(&mut tx)?;

        Ok(tx)
    }

    /// Select UTXOs using largest-first strategy.
    fn select_utxos_largest_first(
        &self,
        target: u64,
        fee_rate: u64,
    ) -> Result<Vec<UTXO>, WalletError> {
        let mut utxos: Vec<UTXO> = self.utxos.values().cloned().collect();
        utxos.sort_by(|a, b| b.amount.cmp(&a.amount));

        let mut selected = Vec::new();
        let mut total = 0u64;

        for utxo in utxos {
            selected.push(utxo.clone());
            total += utxo.amount;

            let estimated_size = estimate_tx_size(selected.len(), 2);
            let fee = estimated_size * fee_rate;

            if total >= target + fee {
                return Ok(selected);
            }
        }

        Err(WalletError::InsufficientFunds)
    }

    /// Sign all inputs of a transaction with the master key.
    fn sign_transaction(&self, tx: &mut Transaction) -> Result<(), WalletError> {
        let tx_hash = tx.calculate_hash();

        for input in &mut tx.inputs {
            let signature: Signature = self.master_key.sign(tx_hash.as_ref());
            input.signature = hex::encode(signature.to_der().as_bytes());
        }

        tx.txid = hex::encode(&tx_hash[..16]);
        Ok(())
    }

    /// Remove spent UTXOs by their input references.
    pub fn mark_utxos_spent(&mut self, inputs: &[TxInput]) {
        for input in inputs {
            let key = format!("{}:{}", input.txid, input.vout);
            self.utxos.remove(&key);
        }
    }
}

// ============================================================================
// TRANSACTION TYPES
// ============================================================================

/// A wallet transaction with inputs, outputs, fee, and size.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub fee: u64,
    pub size: u64,
}

/// A transaction input referencing a spent UTXO.
#[derive(Debug, Clone)]
pub struct TxInput {
    pub txid: String,
    pub vout: u32,
    pub amount: u64,
    pub signature: String,
}

/// A transaction output sending funds to an address.
#[derive(Debug, Clone)]
pub struct TxOutput {
    pub address: String,
    pub amount: u64,
}

impl Transaction {
    /// Compute the SHA-256 hash of the transaction contents.
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();

        for input in &self.inputs {
            hasher.update(input.txid.as_bytes());
            hasher.update(&input.vout.to_le_bytes());
            hasher.update(&input.amount.to_le_bytes());
        }

        for output in &self.outputs {
            hasher.update(output.address.as_bytes());
            hasher.update(&output.amount.to_le_bytes());
        }

        hasher.finalize().to_vec()
    }
}

// ============================================================================
// UTXO SELECTION STRATEGIES (standalone functions)
// ============================================================================

/// Select UTXOs using largest-first strategy.
pub fn select_largest_first(utxos: &[UTXO], target: u64) -> Vec<UTXO> {
    let mut sorted = utxos.to_vec();
    sorted.sort_by(|a, b| b.amount.cmp(&a.amount));

    let mut selected = Vec::new();
    let mut total = 0u64;

    for utxo in sorted {
        if total >= target {
            break;
        }
        total += utxo.amount;
        selected.push(utxo);
    }
    selected
}

/// Select UTXOs using smallest-first strategy.
pub fn select_smallest_first(utxos: &[UTXO], target: u64) -> Vec<UTXO> {
    let mut sorted = utxos.to_vec();
    sorted.sort_by(|a, b| a.amount.cmp(&b.amount));

    let mut selected = Vec::new();
    let mut total = 0u64;

    for utxo in sorted {
        if total >= target {
            break;
        }
        total += utxo.amount;
        selected.push(utxo);
    }
    selected
}

/// Find a UTXO that exactly matches the target amount.
pub fn find_exact_match(utxos: &[UTXO], target: u64) -> Option<UTXO> {
    utxos.iter().find(|u| u.amount == target).cloned()
}

// ============================================================================
// FEE ESTIMATION
// ============================================================================

/// Dust threshold in satoshis -- outputs below this are uneconomical to spend.
pub const DUST_THRESHOLD: u64 = 546;

/// Base transaction overhead in virtual bytes.
pub const TX_BASE_SIZE: u64 = 10;

/// Size of a typical P2PKH input in virtual bytes.
pub const TX_INPUT_SIZE: u64 = 148;

/// Size of a typical P2PKH output in virtual bytes.
pub const TX_OUTPUT_SIZE: u64 = 34;

/// Estimate the virtual size of a transaction.
pub fn estimate_tx_size(inputs: usize, outputs: usize) -> u64 {
    TX_BASE_SIZE + (inputs as u64 * TX_INPUT_SIZE) + (outputs as u64 * TX_OUTPUT_SIZE)
}

// ============================================================================
// FORMATTING UTILITIES
// ============================================================================

/// Format satoshis as a BTC string with 8 decimal places.
pub fn format_btc(satoshis: u64) -> String {
    let btc = satoshis as f64 / 100_000_000.0;
    format!("{:.8}", btc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let w = Wallet::new("test".into());
        assert_eq!(w.name, "test");
        assert_eq!(w.address_count(), 1);
        assert_eq!(w.get_balance(), 0);
    }

    #[test]
    fn test_format_btc() {
        assert_eq!(format_btc(100_000_000), "1.00000000");
        assert_eq!(format_btc(50_000_000), "0.50000000");
        assert_eq!(format_btc(1), "0.00000001");
    }

    #[test]
    fn test_estimate_tx_size() {
        let size = estimate_tx_size(1, 2);
        assert_eq!(size, 10 + 148 + 68); // base + 1 input + 2 outputs
    }
}
