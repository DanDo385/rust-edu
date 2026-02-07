// Lab 52: Blockchain Node
//
// Complete blockchain node with Block, Transaction, UTXO set, Mempool,
// and chain validation. Demonstrates how all pieces of a blockchain
// fit together into a working system.
//
// Key concepts:
// - SHA-256 proof-of-work mining
// - Merkle root for transaction commitment
// - UTXO (Unspent Transaction Output) model
// - Mempool for pending transactions
// - Block validation (PoW, merkle root, transaction validity)
// - Coinbase transactions (block reward + fees)

use sha2::{Digest, Sha256};
use std::collections::HashMap;

// ============================================================================
// BLOCK
// ============================================================================

/// A block in the blockchain, containing transactions and proof-of-work.
#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub merkle_root: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Create a new block (hash is computed but not mined yet).
    pub fn new(index: u64, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let merkle_root = calculate_merkle_root(&transactions);
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            merkle_root,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Compute the SHA-256 hash of the block header.
    pub fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.merkle_root, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let result = hasher.finalize();
        result.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Mine the block by finding a nonce that produces a hash starting
    /// with `difficulty` zeros.
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    /// Recompute the merkle root from the block's transactions.
    pub fn verify_merkle_root(&self) -> bool {
        let computed = calculate_merkle_root(&self.transactions);
        self.merkle_root == computed
    }
}

// ============================================================================
// TRANSACTION
// ============================================================================

/// A blockchain transaction with inputs (spent UTXOs) and outputs (new UTXOs).
#[derive(Clone, Debug)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub timestamp: u64,
}

impl Transaction {
    /// Create a new transaction and compute its txid.
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>, timestamp: u64) -> Self {
        let mut tx = Transaction {
            txid: String::new(),
            inputs,
            outputs,
            timestamp,
        };
        tx.txid = tx.calculate_txid();
        tx
    }

    /// Create a coinbase transaction (no inputs, miner reward).
    pub fn coinbase(address: String, amount: u64, timestamp: u64, label: String) -> Self {
        Transaction {
            txid: label,
            inputs: Vec::new(),
            outputs: vec![TxOutput {
                address,
                amount,
            }],
            timestamp,
        }
    }

    /// Compute the transaction ID from its contents.
    pub fn calculate_txid(&self) -> String {
        let mut hasher = Sha256::new();

        for input in &self.inputs {
            hasher.update(input.txid.as_bytes());
            hasher.update(&input.vout.to_le_bytes());
        }

        for output in &self.outputs {
            hasher.update(output.address.as_bytes());
            hasher.update(&output.amount.to_le_bytes());
        }

        hasher.update(&self.timestamp.to_le_bytes());

        let result = hasher.finalize();
        result.iter().map(|b| format!("{:02x}", b)).take(16).collect()
    }

    /// Calculate the fee for this transaction given a UTXO set.
    pub fn calculate_fee(&self, utxo_set: &UTXOSet) -> u64 {
        let input_total: u64 = self
            .inputs
            .iter()
            .filter_map(|input| utxo_set.get_utxo(&input.txid, input.vout))
            .map(|utxo| utxo.output.amount)
            .sum();

        let output_total: u64 = self.outputs.iter().map(|o| o.amount).sum();

        input_total.saturating_sub(output_total)
    }

    /// Check if this is a coinbase transaction (no inputs).
    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}

/// A transaction input referencing a previously unspent output.
#[derive(Clone, Debug)]
pub struct TxInput {
    pub txid: String,
    pub vout: usize,
    pub signature: String,
}

/// A transaction output assigning value to an address.
#[derive(Clone, Debug)]
pub struct TxOutput {
    pub address: String,
    pub amount: u64,
}

// ============================================================================
// UTXO SET
// ============================================================================

/// The set of all unspent transaction outputs, keyed by "txid:vout".
pub struct UTXOSet {
    utxos: HashMap<String, UTXO>,
}

/// A single unspent transaction output.
#[derive(Clone, Debug)]
pub struct UTXO {
    pub txid: String,
    pub vout: usize,
    pub output: TxOutput,
}

impl UTXOSet {
    /// Create an empty UTXO set.
    pub fn new() -> Self {
        UTXOSet {
            utxos: HashMap::new(),
        }
    }

    /// Add a UTXO to the set.
    pub fn add_utxo(&mut self, txid: String, vout: usize, output: TxOutput) {
        let key = format!("{}:{}", txid, vout);
        self.utxos.insert(key, UTXO { txid, vout, output });
    }

    /// Remove a UTXO (mark it as spent).
    pub fn remove_utxo(&mut self, txid: &str, vout: usize) {
        let key = format!("{}:{}", txid, vout);
        self.utxos.remove(&key);
    }

    /// Look up a UTXO by txid and output index.
    pub fn get_utxo(&self, txid: &str, vout: usize) -> Option<&UTXO> {
        let key = format!("{}:{}", txid, vout);
        self.utxos.get(&key)
    }

    /// Get the total balance for a given address.
    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxos
            .values()
            .filter(|utxo| utxo.output.address == address)
            .map(|utxo| utxo.output.amount)
            .sum()
    }

    /// Count all UTXOs.
    pub fn count(&self) -> usize {
        self.utxos.len()
    }

    /// Get all UTXOs belonging to an address.
    pub fn get_utxos_for_address(&self, address: &str) -> Vec<&UTXO> {
        self.utxos
            .values()
            .filter(|utxo| utxo.output.address == address)
            .collect()
    }
}

impl Default for UTXOSet {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MEMPOOL
// ============================================================================

/// A mempool holding unconfirmed transactions.
pub struct Mempool {
    transactions: HashMap<String, Transaction>,
}

impl Mempool {
    /// Create an empty mempool.
    pub fn new() -> Self {
        Mempool {
            transactions: HashMap::new(),
        }
    }

    /// Add a transaction to the mempool.
    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.insert(tx.txid.clone(), tx);
    }

    /// Remove a transaction from the mempool.
    pub fn remove_transaction(&mut self, txid: &str) {
        self.transactions.remove(txid);
    }

    /// Select transactions for inclusion in a block.
    pub fn select_transactions(&self) -> Vec<Transaction> {
        self.transactions.values().cloned().collect()
    }

    /// Return the number of pending transactions.
    pub fn size(&self) -> usize {
        self.transactions.len()
    }

    /// Check if a transaction is in the mempool.
    pub fn contains(&self, txid: &str) -> bool {
        self.transactions.contains_key(txid)
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// BLOCKCHAIN
// ============================================================================

/// A chain of blocks with proof-of-work consensus.
pub struct Blockchain {
    chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// Create a new blockchain with a genesis block.
    pub fn new(difficulty: usize, genesis_timestamp: u64) -> Self {
        let genesis_tx = Transaction::coinbase(
            "genesis_address".to_string(),
            100_00000000,
            0,
            "genesis_tx".to_string(),
        );

        let mut genesis = Block::new(0, genesis_timestamp, vec![genesis_tx], "0".to_string());
        genesis.mine(difficulty);

        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    /// Add a pre-mined block to the chain.
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    /// Return the latest block.
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    /// Return the chain height (number of blocks).
    pub fn height(&self) -> usize {
        self.chain.len()
    }

    /// Get a block by height.
    pub fn get_block(&self, height: usize) -> Option<&Block> {
        self.chain.get(height)
    }

    /// Validate the entire chain: check hashes, previous_hash links, and PoW.
    pub fn is_valid(&self) -> bool {
        let target = "0".repeat(self.difficulty);

        for i in 1..self.chain.len() {
            let block = &self.chain[i];
            let prev = &self.chain[i - 1];

            // Check previous hash link
            if block.previous_hash != prev.hash {
                return false;
            }

            // Verify hash matches contents
            if block.hash != block.calculate_hash() {
                return false;
            }

            // Check proof-of-work
            if !block.hash.starts_with(&target) {
                return false;
            }

            // Check merkle root
            if !block.verify_merkle_root() {
                return false;
            }
        }

        true
    }
}

// ============================================================================
// MERKLE ROOT
// ============================================================================

/// Calculate the Merkle root of a list of transactions.
///
/// Uses SHA-256 to pair-hash transaction IDs bottom-up until a single
/// root hash remains. If the count is odd, the last hash is duplicated.
pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    if transactions.is_empty() {
        return "0".to_string();
    }

    let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.txid.clone()).collect();

    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        let mut new_hashes = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let combined = format!("{}{}", hashes[i], hashes[i + 1]);
            let mut hasher = Sha256::new();
            hasher.update(combined.as_bytes());
            let result = hasher.finalize();
            new_hashes.push(result.iter().map(|b| format!("{:02x}", b)).collect());
        }

        hashes = new_hashes;
    }

    hashes[0].clone()
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validate a transaction against a UTXO set.
///
/// Checks:
/// - All inputs reference existing UTXOs
/// - All inputs have non-empty signatures
/// - Total outputs do not exceed total inputs
/// - Fee meets minimum threshold
pub fn validate_transaction(
    tx: &Transaction,
    utxo_set: &UTXOSet,
    min_fee: u64,
) -> Result<(), String> {
    let mut input_total = 0u64;

    for input in &tx.inputs {
        match utxo_set.get_utxo(&input.txid, input.vout) {
            Some(utxo) => {
                if input.signature.is_empty() {
                    return Err("Invalid signature".to_string());
                }
                input_total += utxo.output.amount;
            }
            None => {
                return Err(format!("UTXO not found: {}:{}", input.txid, input.vout));
            }
        }
    }

    let output_total: u64 = tx.outputs.iter().map(|o| o.amount).sum();

    if output_total > input_total {
        return Err("Outputs exceed inputs".to_string());
    }

    let fee = input_total - output_total;
    if fee < min_fee {
        return Err(format!("Fee too low: {} < {}", fee, min_fee));
    }

    Ok(())
}

/// Validate a block's proof-of-work.
pub fn validate_proof_of_work(block: &Block, difficulty: usize) -> bool {
    let target = "0".repeat(difficulty);
    block.hash.starts_with(&target)
}

/// Update a UTXO set given a newly confirmed block.
pub fn apply_block_to_utxo_set(block: &Block, utxo_set: &mut UTXOSet) {
    // Remove spent UTXOs
    for tx in &block.transactions {
        for input in &tx.inputs {
            utxo_set.remove_utxo(&input.txid, input.vout);
        }
    }

    // Add new UTXOs
    for tx in &block.transactions {
        for (idx, output) in tx.outputs.iter().enumerate() {
            utxo_set.add_utxo(tx.txid.clone(), idx, output.clone());
        }
    }
}

// ============================================================================
// UTILITY
// ============================================================================

/// Format satoshi-like amounts as coin strings.
pub fn format_coins(satoshis: u64) -> String {
    let coins = satoshis as f64 / 100_000_000.0;
    format!("{:.2}", coins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash_deterministic() {
        let b = Block::new(0, 1000, vec![], "prev".into());
        let h1 = b.calculate_hash();
        let h2 = b.calculate_hash();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_merkle_root_single_tx() {
        let tx = Transaction::coinbase("addr".into(), 100, 0, "tx1".into());
        let root = calculate_merkle_root(&[tx]);
        assert_eq!(root, "tx1"); // single tx -> root is just its id
    }

    #[test]
    fn test_utxo_set_basic() {
        let mut set = UTXOSet::new();
        set.add_utxo("tx1".into(), 0, TxOutput { address: "A".into(), amount: 100 });
        assert_eq!(set.get_balance("A"), 100);
        assert_eq!(set.count(), 1);
    }
}
