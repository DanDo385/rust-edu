//! Lab 50: Wallet CLI - Student API stubs.
//!
//! Implement the wallet API below; the reference implementation is in `solution.rs`.

use k256::ecdsa::SigningKey;
use std::collections::HashMap;

pub struct Wallet {
    pub name: String,
    master_key: SigningKey,
    addresses: Vec<WalletAddress>,
    utxos: HashMap<String, UTXO>,
    address_index: u32,
}

#[derive(Debug, Clone)]
pub struct WalletAddress {
    pub address: String,
    pub public_key: String,
    pub index: u32,
    pub used: bool,
}

#[derive(Debug, Clone)]
pub struct UTXO {
    pub txid: String,
    pub vout: u32,
    pub amount: u64,
    pub address: String,
    pub confirmations: u32,
}

#[derive(Debug, PartialEq)]
pub enum WalletError {
    InsufficientFunds,
    InvalidAddress,
    FeeTooHigh,
    SigningFailed,
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Format wallet error messages")
    }
}

impl Wallet {
    pub fn new(_name: String) -> Self {
        todo!("Create wallet with random master key and initial address")
    }

    pub fn from_key(_name: String, _master_key: SigningKey) -> Self {
        todo!("Create deterministic wallet from existing signing key")
    }

    pub fn generate_address(&mut self) -> String {
        let _ = self;
        todo!("Derive and store a new receiving address")
    }

    pub fn get_root_address(&self) -> String {
        let _ = self;
        todo!("Return the first wallet address")
    }

    pub fn is_my_address(&self, _address: &str) -> bool {
        let _ = self;
        todo!("Check whether address belongs to this wallet")
    }

    pub fn receive_funds(&mut self, _txid: String, _vout: u32, _amount: u64, _address: String) {
        let _ = self;
        todo!("Insert UTXO for received funds")
    }

    pub fn get_balance(&self) -> u64 {
        let _ = self;
        todo!("Sum UTXO amounts")
    }

    pub fn utxo_count(&self) -> usize {
        let _ = self;
        todo!("Count tracked UTXOs")
    }

    pub fn address_count(&self) -> usize {
        let _ = self;
        todo!("Count generated wallet addresses")
    }

    pub fn create_transaction(
        &self,
        _recipient: String,
        _amount: u64,
        _fee_rate: u64,
    ) -> Result<Transaction, WalletError> {
        todo!("Select UTXOs, compute fees, build outputs, and sign inputs")
    }

    pub fn mark_utxos_spent(&mut self, _inputs: &[TxInput]) {
        let _ = self;
        todo!("Remove spent UTXOs after broadcast")
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub fee: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct TxInput {
    pub txid: String,
    pub vout: u32,
    pub amount: u64,
    pub signature: String,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub address: String,
    pub amount: u64,
}

impl Transaction {
    pub fn calculate_hash(&self) -> Vec<u8> {
        let _ = self;
        todo!("Hash transaction content deterministically")
    }
}

pub fn select_largest_first(_utxos: &[UTXO], _target: u64) -> Vec<UTXO> {
    todo!("Select largest UTXOs first until target is met")
}

pub fn select_smallest_first(_utxos: &[UTXO], _target: u64) -> Vec<UTXO> {
    todo!("Select smallest UTXOs first until target is met")
}

pub fn find_exact_match(_utxos: &[UTXO], _target: u64) -> Option<UTXO> {
    todo!("Find a single UTXO exactly matching the target amount")
}

pub fn estimate_tx_size(_inputs: usize, _outputs: usize) -> u64 {
    todo!("Estimate transaction vbytes from input/output counts")
}

pub fn format_btc(_satoshis: u64) -> String {
    todo!("Render satoshis as BTC string with 8 decimals")
}

#[doc(hidden)]
pub mod solution;
