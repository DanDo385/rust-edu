//! # Lab 14 - UTXO Model
//!
//! This lab implements the UTXO (Unspent Transaction Output) model used by
//! Bitcoin and other cryptocurrencies. You'll learn how UTXOs work, how to
//! prevent double-spending, and how this model differs from account-based systems.
//!
//! ## Key Concepts
//! - Instead of account balances, blockchain tracks individual "coins" (UTXOs)
//! - Each transaction CONSUMES some UTXOs and CREATES new UTXOs
//! - Once a UTXO is spent, it's removed from the "UTXO set"
//! - Your balance = sum of all UTXOs you can spend

use std::collections::HashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Represents a unique identifier for a transaction output.
/// In real Bitcoin: (transaction_hash, output_index).
/// For simplicity: a string like "tx1:0" meaning "transaction tx1, output 0".
pub type UtxoId = String;

/// Represents an address that can own UTXOs.
/// In real Bitcoin: a public key hash.
/// For simplicity: plain strings like "Alice".
pub type Address = String;

/// Represents a UTXO (Unspent Transaction Output).
/// This is a "coin" that exists in the blockchain and can be spent.
///
/// ## Ownership & Borrowing
/// - `owner: Address` is an owned String — the UTXO owns its address data
/// - `amount: u64` is Copy — cheap to pass around on the stack
/// - Clone derived so UTXOs can be duplicated when needed (e.g., for lookups)
#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    /// The address that owns this UTXO (who can spend it)
    pub owner: Address,
    /// The amount of cryptocurrency in this UTXO
    pub amount: u64,
}

impl Utxo {
    /// Creates a new UTXO with the given owner and amount.
    pub fn new(owner: Address, amount: u64) -> Self {
        // TODO: Construct and return `Utxo { owner, amount }`.
        let _ = (owner, amount);
        todo!("Create a Utxo value")
    }
}

/// Represents a transaction input — a reference to a UTXO being spent.
///
/// When you spend a UTXO, you reference it by ID and prove ownership
/// by providing the spender address (in real Bitcoin, this would be a
/// cryptographic signature).
#[derive(Debug, Clone)]
pub struct TxInput {
    /// The ID of the UTXO being spent
    pub utxo_id: UtxoId,
    /// The address spending this UTXO (in real blockchain, this would be a signature)
    pub spender: Address,
}

impl TxInput {
    pub fn new(utxo_id: UtxoId, spender: Address) -> Self {
        // TODO: Construct and return `TxInput { utxo_id, spender }`.
        let _ = (utxo_id, spender);
        todo!("Create a TxInput value")
    }
}

/// Represents a transaction output — a new UTXO being created.
#[derive(Debug, Clone)]
pub struct TxOutput {
    /// Who will own this new UTXO?
    pub recipient: Address,
    /// How much cryptocurrency in this new UTXO?
    pub amount: u64,
}

impl TxOutput {
    pub fn new(recipient: Address, amount: u64) -> Self {
        // TODO: Construct and return `TxOutput { recipient, amount }`.
        let _ = (recipient, amount);
        todo!("Create a TxOutput value")
    }
}

/// Represents a complete transaction.
/// A transaction CONSUMES inputs (old UTXOs) and CREATES outputs (new UTXOs).
///
/// ## Conservation of Value
/// The sum of inputs must be >= the sum of outputs.
/// The difference (inputs - outputs) is the transaction fee.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Unique identifier for this transaction
    pub id: String,
    /// List of UTXOs being spent (consumed)
    pub inputs: Vec<TxInput>,
    /// List of new UTXOs being created
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn new(id: String, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        // TODO: Construct and return `Transaction { id, inputs, outputs }`.
        let _ = (id, inputs, outputs);
        todo!("Create a Transaction value")
    }
}

// ============================================================================
// UTXO SET MANAGEMENT
// ============================================================================

/// The UTXO set is the collection of ALL unspent transaction outputs.
/// This is what blockchain full nodes maintain in memory/disk.
///
/// ## Why HashMap?
/// O(1) lookups are critical for blockchain performance.
/// Bitcoin's UTXO set has MILLIONS of entries!
pub type UtxoSet = HashMap<UtxoId, Utxo>;

/// Validates and applies a transaction to the UTXO set.
///
/// This is the CORE of the UTXO model. This function:
/// 1. Checks that all inputs exist in the UTXO set
/// 2. Checks that the spender owns the UTXOs
/// 3. Checks that input sum >= output sum (no money creation!)
/// 4. REMOVES spent UTXOs from the set
/// 5. ADDS new UTXOs to the set
///
/// ## Ownership & Borrowing
/// - `utxo_set: &mut UtxoSet` — we need mutable access to modify the set
/// - `tx: &Transaction` — we only need to read the transaction
///
/// ## Returns
/// `Ok(fee)` with the transaction fee if valid, `Err(reason)` if invalid.
pub fn apply_transaction(utxo_set: &mut UtxoSet, tx: &Transaction) -> Result<u64, String> {
    // TODO: Validate ownership + value conservation, remove spent inputs, add new outputs.
    let _ = (utxo_set, tx);
    todo!("Apply transaction to UTXO set")
}

/// Calculate the balance of an address by summing all UTXOs they own.
///
/// In the UTXO model, there's no single "account balance" variable.
/// Your balance is the SUM of all UTXOs you control.
pub fn get_balance(utxo_set: &UtxoSet, address: &str) -> u64 {
    // TODO: Sum amounts of all UTXOs owned by `address`.
    let _ = (utxo_set, address);
    todo!("Compute address balance from UTXO set")
}

/// Get all UTXOs owned by an address.
/// Useful for wallet software to show which "coins" you have.
pub fn get_utxos_for_address(utxo_set: &UtxoSet, address: &str) -> Vec<(UtxoId, Utxo)> {
    // TODO: Return cloned `(id, utxo)` entries owned by `address`.
    let _ = (utxo_set, address);
    todo!("Collect UTXOs for one address")
}

/// Create a genesis UTXO (like a mining reward) and add it to the set.
/// Only genesis/coinbase transactions can create value without inputs.
pub fn create_genesis_utxo(utxo_set: &mut UtxoSet, id: &str, owner: &str, amount: u64) {
    // TODO: Insert one genesis UTXO with provided id/owner/amount.
    let _ = (utxo_set, id, owner, amount);
    todo!("Insert a genesis UTXO")
}

pub mod solution;
