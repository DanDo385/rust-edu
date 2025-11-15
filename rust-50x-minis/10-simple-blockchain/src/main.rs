// Project 10: Simple Blockchain
//
// Combines hashing, structs, and vectors to create a basic blockchain.
// This demonstrates the core concepts behind Bitcoin, Ethereum, and other blockchains.

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== Simple Blockchain ===\n");

    // Create a new blockchain
    let mut blockchain = Blockchain::new();

    println!("Creating blockchain with genesis block...");
    println!("Genesis block hash: {}", blockchain.get_latest_block().hash);
    println!();

    // Add some blocks
    blockchain.add_block("Alice sends 10 BTC to Bob".to_string());
    blockchain.add_block("Bob sends 5 BTC to Charlie".to_string());
    blockchain.add_block("Charlie sends 2 BTC to Alice".to_string());

    // Display the blockchain
    println!("=== Blockchain Contents ===");
    blockchain.print();

    // Validate the chain
    println!("\n=== Chain Validation ===");
    println!("Is blockchain valid? {}", blockchain.is_valid());

    // Try to tamper with a block
    println!("\n=== Tampering Test ===");
    println!("Attempting to modify block 1...");
    blockchain.chain[1].data = "Alice sends 1000 BTC to Bob".to_string();
    println!("Is blockchain valid after tampering? {}", blockchain.is_valid());
}

// ============================================================================
// BLOCK STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,  // For proof of work (simple version)
}

impl Block {
    /// Creates a new block
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block
    }

    /// Creates the genesis block (first block in the chain)
    fn genesis() -> Block {
        Block::new(0, "Genesis Block".to_string(), "0".to_string())
    }

    /// Calculates the hash of this block
    fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let result = hasher.finalize();

        result.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    /// Simple proof of work (find hash with leading zeros)
    /// In real blockchains like Bitcoin, this is much more complex
    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined! Hash: {}", self.hash);
    }
}

// ============================================================================
// BLOCKCHAIN STRUCTURE
// ============================================================================

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    /// Creates a new blockchain with genesis block
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2,  // Require 2 leading zeros in hash
        };

        // Add the genesis block
        blockchain.chain.push(Block::genesis());
        blockchain
    }

    /// Gets the latest block in the chain
    fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Chain is empty")
    }

    /// Adds a new block to the chain
    fn add_block(&mut self, data: String) {
        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(
            previous_block.index + 1,
            data.clone(),
            previous_block.hash.clone(),
        );

        // Mine the block (proof of work)
        println!("Mining block {}...", new_block.index);
        new_block.mine(self.difficulty);

        self.chain.push(new_block);
    }

    /// Validates the entire blockchain
    fn is_valid(&self) -> bool {
        // Check each block (skip genesis)
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check if current block's hash is valid
            if current.hash != current.calculate_hash() {
                println!("❌ Block {} has invalid hash", i);
                return false;
            }

            // Check if current block references correct previous hash
            if current.previous_hash != previous.hash {
                println!("❌ Block {} has invalid previous_hash", i);
                return false;
            }

            // Check proof of work
            let target = "0".repeat(self.difficulty);
            if &current.hash[..self.difficulty] != target {
                println!("❌ Block {} doesn't meet difficulty requirements", i);
                return false;
            }
        }

        println!("✅ Blockchain is valid!");
        true
    }

    /// Prints the entire blockchain
    fn print(&self) {
        for block in &self.chain {
            println!("Block #{}:", block.index);
            println!("  Timestamp: {}", block.timestamp);
            println!("  Data: {}", block.data);
            println!("  Previous Hash: {}", block.previous_hash);
            println!("  Hash: {}", block.hash);
            println!("  Nonce: {}", block.nonce);
            println!();
        }
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. VECTOR GROWTH
//    Vec<Block> grows dynamically. Each Block is relatively large,
//    so the Vec stores them inline (not pointers).
//    Adding blocks may cause reallocation and copying.
//
// 2. STRING CLONING
//    We clone() hashes when creating blocks. Each clone allocates
//    new heap memory. Could optimize with &str lifetimes or Rc<String>.
//
// 3. OWNERSHIP IN THE CHAIN
//    The Blockchain OWNS the Vec, which OWNS each Block.
//    When Blockchain is dropped, entire chain is freed automatically.
//
// 4. PROOF OF WORK
//    The mining loop increments nonce and recalculates hash.
//    With difficulty=2, this averages ~256 iterations.
//    Bitcoin uses difficulty=~20 leading zeros (trillions of iterations!)
//
// 5. NO GARBAGE COLLECTION
//    All memory is managed by ownership. When a Block goes out of scope,
//    its Strings are freed. No GC pauses!

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Blockchain = chain of blocks linked by hashes
// 2. Each block references the previous block's hash
// 3. Changing any block invalidates all subsequent blocks
// 4. Genesis block is the first block (no previous hash)
// 5. Proof of work makes it expensive to modify the chain
// 6. Validation ensures chain integrity
// 7. Real blockchains add: networking, consensus, transactions, smart contracts
// 8. Rust's ownership prevents memory leaks in the chain

// ============================================================================
// WHY BLOCKCHAIN IS IMMUTABLE
// ============================================================================
// If you change a block's data:
// 1. Its hash changes
// 2. The next block's previous_hash no longer matches
// 3. You must recalculate the next block's hash
// 4. This cascades through the entire chain
// 5. You must re-mine every subsequent block (expensive!)
// 6. In a distributed network, other nodes reject your modified chain
//
// This makes blockchain tamper-evident and practically immutable.

// ============================================================================
// REAL BLOCKCHAIN ADDITIONS
// ============================================================================
// Bitcoin/Ethereum blockchains add:
// - Transactions (inputs, outputs, signatures)
// - Merkle trees (efficient transaction storage)
// - Peer-to-peer networking (gossip protocol)
// - Consensus algorithms (PoW, PoS)
// - Wallets (public/private key cryptography)
// - Smart contracts (Ethereum)
// - Difficulty adjustment (maintain ~10min block time)
// - Fork choice rules (handle chain splits)

// ============================================================================
// PERFORMANCE CONSIDERATIONS
// ============================================================================
// - Difficulty=2: ~256 hashes per block (~1ms on modern CPU)
// - Difficulty=4: ~65,536 hashes (~100ms)
// - Difficulty=6: ~16 million hashes (~25 seconds)
// - Bitcoin difficulty=~20 leading zeros (10 minutes with massive hardware)
//
// SHA-256 is FAST in Rust - comparable to C implementations

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not cloning previous_hash (ownership error)
// ❌ Forgetting to update hash after changing data
// ❌ Off-by-one errors in chain validation
// ❌ Not handling empty chain case
// ❌ Using weak hash function (MD5, CRC32)
// ❌ Forgetting proof of work validation
