//! # Simple Blockchain - Complete Solution
//!
//! ## What is a Blockchain?
//!
//! A blockchain is a linked list of blocks where:
//! - Each block contains data (transactions in Bitcoin)
//! - Each block has a hash of the previous block
//! - Changing any block breaks the chain (requires re-mining all subsequent blocks)
//!
//! ## Security Properties
//!
//! 1. **Immutability**: Can't change past blocks without detection
//! 2. **Proof-of-Work**: Expensive to create blocks (prevents spam)
//! 3. **Chain validation**: Anyone can verify entire history
//! 4. **Consensus**: Longest valid chain wins (51% attack protection)
//!
//! NOTE: This implementation uses a simple hash function for educational purposes.
//! In production, use a proper cryptographic hash library like sha2.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

/// A single block in the blockchain.
///
/// ## Structure (similar to Bitcoin)
/// - index: Position in chain (block height)
/// - timestamp: When block was created
/// - data: Block contents (transactions in Bitcoin)
/// - previous_hash: Hash of previous block (creates the "chain")
/// - hash: This block's hash (block ID)
/// - nonce: Proof-of-Work counter (mining)
#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Create a new unmined block.
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("System time before UNIX_EPOCH (1970-01-01); check system clock")
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        // Calculate initial hash (before mining)
        block.hash = block.calculate_hash();
        block
    }

    /// Mine the block (Proof-of-Work).
    ///
    /// ## How Mining Works
    /// 1. Try different nonce values
    /// 2. Calculate hash for each nonce
    /// 3. Check if hash starts with required zeros
    /// 4. If yes, done! If no, increment nonce and try again
    ///
    /// ## Difficulty
    /// - difficulty = number of leading zeros required
    /// - Bitcoin: ~19 leading zeros (as of 2024)
    /// - Each additional zero makes it ~16x harder
    ///
    /// ## Parameters
    /// - difficulty: Number of leading zeros required
    pub fn mine(&mut self, difficulty: usize) {
        // Create target prefix (e.g., "00" for difficulty 2)
        let target = "0".repeat(difficulty);

        // Mining loop - keep trying until valid hash found
        loop {
            self.hash = self.calculate_hash();

            // Check if hash meets difficulty requirement
            if self.hash.starts_with(&target) {
                // Found valid hash! Mining complete.
                println!("Block mined! Nonce: {}, Hash: {}", self.nonce, self.hash);
                break;
            }

            // Not valid yet, try next nonce
            self.nonce += 1;
        }
    }

    /// Calculate block hash (includes all fields except hash itself).
    ///
    /// ## What Gets Hashed (similar to Bitcoin block header)
    /// - index
    /// - timestamp
    /// - data (in Bitcoin: Merkle root of transactions)
    /// - previous_hash (links to previous block)
    /// - nonce (the mining variable)
    pub fn calculate_hash(&self) -> String {
        // Combine all fields into single string
        let block_data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );

        // Hash the combined data
        let hash_bytes = simple_hash(block_data.as_bytes());
        bytes_to_hex(&hash_bytes)
    }
}

/// The blockchain - a chain of blocks.
///
/// ## Properties
/// - chain: Vector of blocks (ordered from genesis to latest)
/// - difficulty: Mining difficulty (number of leading zeros)
///
/// ## Invariant
/// The chain is never empty. It always contains at least the genesis block.
/// This invariant is maintained by only creating Blockchain via Blockchain::new().
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// Create a new blockchain with genesis block.
    ///
    /// ## Genesis Block
    /// - First block in chain (index 0)
    /// - Has no previous block (previous_hash = "0")
    /// - Usually created manually (not mined in our implementation)
    /// - Bitcoin genesis block: Jan 3, 2009
    pub fn new(difficulty: usize) -> Self {
        // Create genesis block
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());

        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    /// Add a new block to the chain.
    ///
    /// ## Process
    /// 1. Get hash of last block (to link new block)
    /// 2. Create new block
    /// 3. Mine the block (Proof-of-Work)
    /// 4. Add to chain
    ///
    /// ## Parameters
    /// - data: Block contents (transactions in Bitcoin)
    ///
    /// ## Safety
    /// Relies on the invariant that the chain is never empty (always has genesis).
    /// This is guaranteed by only creating Blockchain via Blockchain::new().
    pub fn add_block(&mut self, data: String) {
        // Get previous block's hash
        // The invariant ensures chain is never empty, so last() always succeeds
        let previous_hash = self.chain.last().unwrap().hash.clone();

        // Create new block
        let index = self.chain.len() as u64;
        let mut new_block = Block::new(index, data, previous_hash);

        // Mine the block (find valid hash)
        new_block.mine(self.difficulty);

        // Add to chain
        self.chain.push(new_block);
    }

    /// Validate the entire blockchain.
    ///
    /// ## Checks
    /// 1. Each block's hash is correct (recalculate and compare)
    /// 2. Each block links to previous block correctly
    /// 3. All blocks meet difficulty requirement
    ///
    /// ## Returns
    /// true if chain is valid, false if corrupted/invalid
    pub fn is_valid(&self) -> bool {
        // Check each block (skip genesis)
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check 1: Current block's hash is correct
            if current.hash != current.calculate_hash() {
                println!("Block {} hash is invalid!", i);
                return false;
            }

            // Check 2: Links to previous block correctly
            if current.previous_hash != previous.hash {
                println!("Block {} doesn't link to previous block!", i);
                return false;
            }

            // Check 3: Meets difficulty requirement
            let target = "0".repeat(self.difficulty);
            if !current.hash.starts_with(&target) {
                println!("Block {} doesn't meet difficulty requirement!", i);
                return false;
            }
        }

        true
    }
}

// ============================================================================
// BLOCKCHAIN SECURITY ANALYSIS
// ============================================================================
//
// Q: Why is blockchain secure?
// A: Several layers of security:
//
// 1. **Cryptographic Hashing**
//    - Can't forge hashes
//    - Any change in block changes its hash
//    - Breaks link to next block
//
// 2. **Proof-of-Work**
//    - Expensive to mine blocks
//    - Attacker needs massive computational power
//    - Makes tampering economically infeasible
//
// 3. **Chain of Hashes**
//    - Each block links to previous
//    - Changing block N requires re-mining blocks N+1, N+2, ...
//    - Deeper in chain = more secure
//
// 4. **Consensus (not implemented here)**
//    - Multiple nodes maintain copies
//    - Longest valid chain wins
//    - 51% attack: need majority of network power
//
// Q: Can someone tamper with the blockchain?
// A: Theoretically yes, practically no (for Bitcoin):
//
// To tamper with a block:
// 1. Change the block's data
// 2. Re-mine that block (expensive!)
// 3. Re-mine all subsequent blocks (very expensive!)
// 4. Beat the rest of the network (need 51% of hash power)
//
// Bitcoin network hash rate: ~400 EH/s
// Cost to match: billions of dollars
// Plus: You'd need to sustain it as network continues mining
//
// Q: What if difficulty is low?
// A: Easy to mine, but also easy to attack!
// - Bitcoin difficulty adjusts every 2016 blocks
// - Targets 10-minute block time
// - More miners = higher difficulty
// - Fewer miners = lower difficulty
