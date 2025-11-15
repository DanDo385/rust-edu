// Project 49: Blockchain Node (CAPSTONE)
//
// Complete blockchain node integrating UTXO model, mempool, consensus,
// P2P networking, and block validation. This demonstrates how all the
// pieces of a blockchain fit together into a working system.

use sha2::{Sha256, Digest};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== Blockchain Node ===\n");

    // Initialize the node
    println!("Initializing node...");
    let mut node = BlockchainNode::new();
    println!("✅ Genesis block created");
    println!("✅ UTXO set initialized");
    println!("✅ Mempool created");
    println!();

    // Show initial status
    node.show_status();

    // Simulate network activity
    simulate_blockchain_operations(&mut node);
}

// ============================================================================
// BLOCKCHAIN NODE
// ============================================================================

struct BlockchainNode {
    blockchain: Blockchain,
    utxo_set: UTXOSet,
    mempool: Mempool,
    peers: Vec<Peer>,
    config: NodeConfig,
}

struct NodeConfig {
    difficulty: usize,
    block_reward: u64,
    max_block_size: usize,
    min_fee: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            difficulty: 3,
            block_reward: 50_00000000, // 50 coins
            max_block_size: 1_000_000, // 1 MB
            min_fee: 1000, // 0.00001 coins
        }
    }
}

impl BlockchainNode {
    fn new() -> Self {
        let config = NodeConfig::default();
        let mut blockchain = Blockchain::new(config.difficulty);
        let mut utxo_set = UTXOSet::new();

        // Initialize UTXO set from genesis block
        if let Some(genesis) = blockchain.get_latest_block() {
            for (idx, output) in genesis.transactions[0].outputs.iter().enumerate() {
                utxo_set.add_utxo(
                    genesis.transactions[0].txid.clone(),
                    idx,
                    output.clone(),
                );
            }
        }

        BlockchainNode {
            blockchain,
            utxo_set,
            mempool: Mempool::new(),
            peers: Vec::new(),
            config,
        }
    }

    fn show_status(&self) {
        println!("--- Node Status ---");
        println!("Chain height: {}", self.blockchain.height());
        println!("UTXO count: {}", self.utxo_set.count());
        println!("Mempool size: {}", self.mempool.size());
        println!("Connected peers: {}", self.peers.len());
        println!();
    }

    fn receive_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Validate transaction
        self.validate_transaction(&tx)?;

        // Add to mempool
        self.mempool.add_transaction(tx.clone());

        println!("✅ Transaction validated");
        println!("✅ Added to mempool");
        println!("📡 Broadcasting to peers...");
        self.broadcast_transaction(tx);

        Ok(())
    }

    fn validate_transaction(&self, tx: &Transaction) -> Result<(), String> {
        // Check inputs exist in UTXO set
        let mut input_total = 0u64;

        for input in &tx.inputs {
            match self.utxo_set.get_utxo(&input.txid, input.vout) {
                Some(utxo) => {
                    // Verify signature (simplified - just check it's not empty)
                    if input.signature.is_empty() {
                        return Err("Invalid signature".to_string());
                    }

                    input_total += utxo.amount;
                }
                None => return Err(format!("UTXO not found: {}:{}", input.txid, input.vout)),
            }
        }

        // Check outputs
        let output_total: u64 = tx.outputs.iter().map(|o| o.amount).sum();

        // Validate total (outputs + fee ≤ inputs)
        if output_total > input_total {
            return Err("Outputs exceed inputs".to_string());
        }

        let fee = input_total - output_total;
        if fee < self.config.min_fee {
            return Err(format!("Fee too low: {} < {}", fee, self.config.min_fee));
        }

        Ok(())
    }

    fn mine_block(&mut self, miner_address: String) -> Block {
        println!("--- Mining Block ---");

        // Select transactions from mempool
        let transactions = self.mempool.select_transactions(self.config.max_block_size);
        println!("Selected {} transactions from mempool", transactions.len());

        // Create coinbase transaction (block reward)
        let mut all_transactions = vec![self.create_coinbase(miner_address, &transactions)];
        all_transactions.extend(transactions.clone());

        // Mine the block
        let previous_hash = self.blockchain.get_latest_block()
            .map(|b| b.hash.clone())
            .unwrap_or_else(|| "0".to_string());

        let mut block = Block::new(
            self.blockchain.height() as u64,
            all_transactions,
            previous_hash,
        );

        println!("Mining block {}...", block.index);
        block.mine(self.config.difficulty);

        // Validate and add block
        if let Err(e) = self.receive_block(block.clone()) {
            panic!("Failed to add mined block: {}", e);
        }

        block
    }

    fn create_coinbase(&self, miner_address: String, transactions: &[Transaction]) -> Transaction {
        // Calculate total fees
        let fees: u64 = transactions.iter().map(|tx| tx.calculate_fee(&self.utxo_set)).sum();

        // Coinbase reward = block reward + fees
        let total_reward = self.config.block_reward + fees;

        Transaction {
            txid: format!("coinbase_{}", self.blockchain.height()),
            inputs: Vec::new(), // Coinbase has no inputs
            outputs: vec![TxOutput {
                address: miner_address,
                amount: total_reward,
            }],
            timestamp: get_timestamp(),
        }
    }

    fn receive_block(&mut self, block: Block) -> Result<(), String> {
        // Validate block
        self.validate_block(&block)?;

        // Add to blockchain
        self.blockchain.add_block(block.clone());

        // Update UTXO set
        self.update_utxo_set(&block);

        // Remove transactions from mempool
        for tx in &block.transactions {
            self.mempool.remove_transaction(&tx.txid);
        }

        Ok(())
    }

    fn validate_block(&self, block: &Block) -> Result<(), String> {
        // Check proof-of-work
        let target = "0".repeat(self.config.difficulty);
        if !block.hash.starts_with(&target) {
            return Err("Invalid proof-of-work".to_string());
        }

        // Check merkle root
        let calculated_merkle = block.calculate_merkle_root();
        if block.merkle_root != calculated_merkle {
            return Err("Invalid merkle root".to_string());
        }

        // Check previous hash
        if let Some(latest) = self.blockchain.get_latest_block() {
            if block.previous_hash != latest.hash {
                return Err("Invalid previous hash".to_string());
            }
        }

        // Validate all transactions (except coinbase)
        for tx in block.transactions.iter().skip(1) {
            self.validate_transaction(tx)?;
        }

        Ok(())
    }

    fn update_utxo_set(&mut self, block: &Block) {
        // Remove spent UTXOs
        for tx in &block.transactions {
            for input in &tx.inputs {
                self.utxo_set.remove_utxo(&input.txid, input.vout);
            }
        }

        // Add new UTXOs
        for tx in &block.transactions {
            for (idx, output) in tx.outputs.iter().enumerate() {
                self.utxo_set.add_utxo(tx.txid.clone(), idx, output.clone());
            }
        }
    }

    fn broadcast_transaction(&self, tx: Transaction) {
        for peer in &self.peers {
            println!("  → Sending tx {} to peer {}", tx.txid, peer.address);
        }
    }

    fn broadcast_block(&self, block: &Block) {
        println!("📡 Broadcasting block to network...");
        for peer in &self.peers {
            println!("Peer [{}] received block", peer.address);
        }
    }

    fn get_balance(&self, address: &str) -> u64 {
        self.utxo_set.get_balance(address)
    }
}

// ============================================================================
// BLOCKCHAIN
// ============================================================================

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };

        // Create genesis block
        blockchain.create_genesis();
        blockchain
    }

    fn create_genesis(&mut self) {
        let genesis_tx = Transaction {
            txid: "genesis_tx".to_string(),
            inputs: Vec::new(),
            outputs: vec![TxOutput {
                address: "genesis_address".to_string(),
                amount: 100_00000000, // 100 coins
            }],
            timestamp: 0,
        };

        let mut genesis = Block::new(0, vec![genesis_tx], "0".to_string());
        genesis.mine(self.difficulty);

        self.chain.push(genesis);
    }

    fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    fn height(&self) -> usize {
        self.chain.len()
    }

    fn get_block(&self, height: usize) -> Option<&Block> {
        self.chain.get(height)
    }
}

// ============================================================================
// BLOCK
// ============================================================================

#[derive(Clone, Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    merkle_root: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = get_timestamp();
        let merkle_root = Block::calculate_merkle_root_static(&transactions);

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

    fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.merkle_root, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let result = hasher.finalize();

        result.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();

            if self.nonce % 10000 == 0 {
                print!("\rMining... (nonce: {})", self.nonce);
            }
        }

        println!("\n✅ Block mined! Hash: {}", self.hash);
    }

    fn calculate_merkle_root(&self) -> String {
        Block::calculate_merkle_root_static(&self.transactions)
    }

    fn calculate_merkle_root_static(transactions: &[Transaction]) -> String {
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
}

// ============================================================================
// TRANSACTION
// ============================================================================

#[derive(Clone, Debug)]
struct Transaction {
    txid: String,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    timestamp: u64,
}

#[derive(Clone, Debug)]
struct TxInput {
    txid: String,
    vout: usize,
    signature: String,
}

#[derive(Clone, Debug)]
struct TxOutput {
    address: String,
    amount: u64,
}

impl Transaction {
    fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        let timestamp = get_timestamp();

        let mut tx = Transaction {
            txid: String::new(),
            inputs,
            outputs,
            timestamp,
        };

        tx.txid = tx.calculate_txid();
        tx
    }

    fn calculate_txid(&self) -> String {
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

    fn calculate_fee(&self, utxo_set: &UTXOSet) -> u64 {
        let input_total: u64 = self.inputs.iter()
            .filter_map(|input| utxo_set.get_utxo(&input.txid, input.vout))
            .map(|utxo| utxo.amount)
            .sum();

        let output_total: u64 = self.outputs.iter().map(|o| o.amount).sum();

        input_total.saturating_sub(output_total)
    }
}

// ============================================================================
// UTXO SET
// ============================================================================

struct UTXOSet {
    utxos: HashMap<String, UTXO>, // key: "txid:vout"
}

#[derive(Clone, Debug)]
struct UTXO {
    txid: String,
    vout: usize,
    output: TxOutput,
}

impl UTXOSet {
    fn new() -> Self {
        UTXOSet {
            utxos: HashMap::new(),
        }
    }

    fn add_utxo(&mut self, txid: String, vout: usize, output: TxOutput) {
        let key = format!("{}:{}", txid, vout);
        self.utxos.insert(key, UTXO { txid, vout, output });
    }

    fn remove_utxo(&mut self, txid: &str, vout: usize) {
        let key = format!("{}:{}", txid, vout);
        self.utxos.remove(&key);
    }

    fn get_utxo(&self, txid: &str, vout: usize) -> Option<&UTXO> {
        let key = format!("{}:{}", txid, vout);
        self.utxos.get(&key)
    }

    fn get_balance(&self, address: &str) -> u64 {
        self.utxos.values()
            .filter(|utxo| utxo.output.address == address)
            .map(|utxo| utxo.output.amount)
            .sum()
    }

    fn count(&self) -> usize {
        self.utxos.len()
    }

    fn get_utxos_for_address(&self, address: &str) -> Vec<&UTXO> {
        self.utxos.values()
            .filter(|utxo| utxo.output.address == address)
            .collect()
    }
}

// ============================================================================
// MEMPOOL
// ============================================================================

struct Mempool {
    transactions: HashMap<String, Transaction>,
}

impl Mempool {
    fn new() -> Self {
        Mempool {
            transactions: HashMap::new(),
        }
    }

    fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.insert(tx.txid.clone(), tx);
    }

    fn remove_transaction(&mut self, txid: &str) {
        self.transactions.remove(txid);
    }

    fn select_transactions(&self, _max_size: usize) -> Vec<Transaction> {
        // In production, would prioritize by fee and respect size limit
        // For simplicity, return all transactions
        self.transactions.values().cloned().collect()
    }

    fn size(&self) -> usize {
        self.transactions.len()
    }
}

// ============================================================================
// P2P PEER
// ============================================================================

#[derive(Clone)]
struct Peer {
    address: String,
    version: u32,
    services: u64,
}

// ============================================================================
// SIMULATION
// ============================================================================

fn simulate_blockchain_operations(node: &mut BlockchainNode) {
    // Add some simulated peers
    node.peers.push(Peer {
        address: "127.0.0.1:8334".to_string(),
        version: 1,
        services: 1,
    });
    node.peers.push(Peer {
        address: "127.0.0.1:8335".to_string(),
        version: 1,
        services: 1,
    });

    println!("--- Receiving Transaction ---");

    // Create a transaction from genesis address to Alice
    let genesis_utxos = node.utxo_set.get_utxos_for_address("genesis_address");

    if let Some(utxo) = genesis_utxos.first() {
        let tx = Transaction::new(
            vec![TxInput {
                txid: utxo.txid.clone(),
                vout: utxo.vout,
                signature: "simulated_signature".to_string(),
            }],
            vec![
                TxOutput {
                    address: "Alice".to_string(),
                    amount: 50_00000000, // 50 coins
                },
                TxOutput {
                    address: "genesis_address".to_string(), // Change
                    amount: 49_99000000, // 49.99 coins (0.01 fee)
                },
            ],
        );

        println!("Transaction: genesis_address → Alice (50 coins)");

        if let Err(e) = node.receive_transaction(tx) {
            println!("❌ Transaction failed: {}", e);
        }
        println!();
    }

    // Mine a block
    let block = node.mine_block("Miner1".to_string());

    println!();
    println!("--- Block Validation ---");
    println!("✅ Proof-of-work valid");
    println!("✅ Merkle root valid");
    println!("✅ All transactions valid");
    println!("✅ Block added to chain");
    println!();

    // Show updated status
    node.show_status();

    // Show balance changes
    println!("Balance changes:");
    println!("  genesis_address: {} coins", format_coins(node.get_balance("genesis_address")));
    println!("  Alice: {} coins", format_coins(node.get_balance("Alice")));
    println!("  Miner1: {} coins (block reward)", format_coins(node.get_balance("Miner1")));
    println!();

    // Broadcast block
    node.broadcast_block(&block);
    println!("✅ Block propagated to {} peers", node.peers.len());
    println!();

    // Create another transaction (Alice to Bob)
    demonstrate_transaction_flow(node);

    // Show final status
    node.show_status();
}

fn demonstrate_transaction_flow(node: &mut BlockchainNode) {
    println!("--- Second Transaction (Alice → Bob) ---");

    let alice_utxos = node.utxo_set.get_utxos_for_address("Alice");

    if let Some(utxo) = alice_utxos.first() {
        let tx = Transaction::new(
            vec![TxInput {
                txid: utxo.txid.clone(),
                vout: utxo.vout,
                signature: "alice_signature".to_string(),
            }],
            vec![
                TxOutput {
                    address: "Bob".to_string(),
                    amount: 30_00000000, // 30 coins
                },
                TxOutput {
                    address: "Alice".to_string(), // Change
                    amount: 19_99000000, // 19.99 coins (0.01 fee)
                },
            ],
        );

        println!("Transaction: Alice → Bob (30 coins)");

        if let Err(e) = node.receive_transaction(tx) {
            println!("❌ Transaction failed: {}", e);
            return;
        }
        println!();

        // Mine another block
        let _block = node.mine_block("Miner2".to_string());
        println!();

        // Show final balances
        println!("--- Final Balances ---");
        println!("  genesis_address: {} coins", format_coins(node.get_balance("genesis_address")));
        println!("  Alice: {} coins", format_coins(node.get_balance("Alice")));
        println!("  Bob: {} coins", format_coins(node.get_balance("Bob")));
        println!("  Miner1: {} coins", format_coins(node.get_balance("Miner1")));
        println!("  Miner2: {} coins", format_coins(node.get_balance("Miner2")));
        println!();
    }
}

// ============================================================================
// UTILITIES
// ============================================================================

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn format_coins(satoshis: u64) -> String {
    let coins = satoshis as f64 / 100_000_000.0;
    format!("{:.2}", coins)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. OWNERSHIP IN BLOCKCHAIN
//    Blockchain owns Vec<Block>, each Block owns Vec<Transaction>.
//    When block is added, ownership transfers to the blockchain.
//    No garbage collection - memory freed when blockchain is dropped.
//
// 2. HASHMAP PERFORMANCE
//    UTXO set uses HashMap for O(1) lookup by key.
//    SipHash prevents collision attacks (security vs performance trade-off).
//    Alternative: BTreeMap for ordered iteration (used in Bitcoin Core).
//
// 3. CLONING CONSIDERATIONS
//    We clone() transactions when mining blocks - allocates new memory.
//    Could optimize with Arc<Transaction> for shared ownership.
//    Trade-off: simplicity vs memory efficiency.
//
// 4. STRING ALLOCATIONS
//    Hashes and addresses are String (heap-allocated).
//    Each clone allocates new memory - consider Rc/Arc for optimization.
//    In production, might use fixed-size arrays [u8; 32] for hashes.
//
// 5. VALIDATION PERFORMANCE
//    Block validation iterates all transactions - O(n) where n = tx count.
//    Signature verification is expensive (elliptic curve crypto).
//    Bitcoin Core parallelizes this across CPU cores.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Full node validates all blocks and transactions independently
// 2. UTXO set tracks all unspent outputs for validation
// 3. Mempool holds pending transactions waiting for confirmation
// 4. Consensus rules prevent invalid blocks/transactions
// 5. P2P network propagates transactions and blocks via gossip
// 6. Fork resolution uses longest chain rule (most proof-of-work)
// 7. Miners select transactions with highest fees
// 8. Coinbase transaction rewards miner (block reward + fees)
// 9. Merkle root commits to all transactions in block
// 10. This is a simplified node - real nodes are much more complex!

// ============================================================================
// PRODUCTION BLOCKCHAIN NODES
// ============================================================================
// Bitcoin Core adds:
// - Database persistence (LevelDB for UTXO set, blocks)
// - Full P2P protocol (version, ping/pong, inv, getdata, etc.)
// - Difficulty adjustment (every 2016 blocks)
// - Memory pool eviction (size limits, low-fee transaction removal)
// - UTXO cache (faster validation)
// - Compact block relay (save bandwidth)
// - Segregated Witness (SegWit)
// - Schnorr signatures (Taproot)
// - Fee estimation algorithms
// - Wallet integration
// - RPC server (JSON-RPC API)
// - Peer reputation system
// - Eclipse attack prevention
// - Bloom filters for SPV clients
// - ...and much more!
//
// Ethereum (Geth) adds different complexity:
// - Account model (not UTXO)
// - Smart contract execution (EVM)
// - Gas metering
// - State tries (Merkle Patricia Tries)
// - Proof-of-Stake consensus (post-merge)
// - Precompiled contracts
// - Transaction receipts
// - Event logs
// - ...and much more!

// ============================================================================
// COMMON MISTAKES TO AVOID
// ============================================================================
// ❌ Not validating transactions before adding to mempool
// ❌ Not checking for double-spends
// ❌ Forgetting to update UTXO set when adding blocks
// ❌ Not removing mined transactions from mempool
// ❌ Accepting blocks without proper PoW validation
// ❌ Not handling blockchain reorganizations (forks)
// ❌ Unbounded mempool size (DoS vector)
// ❌ Not persisting state to disk (lost on restart)
// ❌ Synchronous block validation (should be async)
// ❌ Not implementing checkpoints (slow initial sync)
