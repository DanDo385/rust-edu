// Lab 48: Proof of Work
//
// This module implements a Proof of Work mining system for a blockchain.
// Demonstrates SHA-256 hashing, difficulty targeting, nonce search,
// and block validation.
//
// Key concepts:
// - SHA-256 cryptographic hashing
// - Brute-force nonce search
// - Difficulty scaling (leading zeros)
// - Block validation
// - Mining result reporting

use sha2::{Digest, Sha256};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// ============================================================================
// BLOCK STRUCTURE
// ============================================================================

/// A block in the blockchain with proof-of-work mining support.
#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub difficulty: usize,
}

impl Block {
    /// Create a new block (unmined -- hash and nonce are not set).
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            difficulty,
        }
    }

    /// Create a new block with an explicit timestamp (useful for testing).
    pub fn with_timestamp(
        index: u64,
        data: String,
        previous_hash: String,
        difficulty: usize,
        timestamp: u64,
    ) -> Block {
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            difficulty,
        }
    }

    /// Create the genesis block (the first block in the chain).
    pub fn genesis(difficulty: usize) -> Block {
        let mut block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        block.hash = block.calculate_hash();
        block
    }

    /// Calculate the SHA-256 hash for this block's contents.
    ///
    /// The hash is computed from: index + timestamp + data + previous_hash + nonce
    pub fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let result = hasher.finalize();

        result.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Mine the block by searching for a nonce that produces a hash
    /// starting with the required number of leading zeros.
    ///
    /// Returns a `MiningResult` with statistics about the mining process.
    pub fn mine(&mut self) -> MiningResult {
        let start = Instant::now();
        let target = "0".repeat(self.difficulty);
        let mut attempts = 0u64;

        loop {
            self.nonce += 1;
            self.hash = self.calculate_hash();
            attempts += 1;

            if self.hash.starts_with(&target) {
                break;
            }
        }

        let duration = start.elapsed();
        let hash_rate = if duration.as_secs_f64() > 0.0 {
            attempts as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        MiningResult {
            nonce: self.nonce,
            attempts,
            duration,
            hash_rate,
            hash: self.hash.clone(),
        }
    }

    /// Validate this block's hash.
    ///
    /// Checks that:
    /// 1. The hash starts with the required number of leading zeros (difficulty)
    /// 2. The hash matches the actual calculated hash
    pub fn is_valid(&self) -> bool {
        let target = "0".repeat(self.difficulty);
        if !self.hash.starts_with(&target) {
            return false;
        }

        self.hash == self.calculate_hash()
    }
}

// ============================================================================
// MINING RESULT
// ============================================================================

/// Statistics from a mining operation.
#[derive(Debug)]
pub struct MiningResult {
    pub nonce: u64,
    pub attempts: u64,
    pub duration: Duration,
    pub hash_rate: f64,
    pub hash: String,
}

// ============================================================================
// HASH UTILITIES
// ============================================================================

/// Compute the SHA-256 hash of arbitrary data, returned as a hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Check whether a hex hash string meets a given difficulty (leading zeros).
pub fn meets_difficulty(hash: &str, difficulty: usize) -> bool {
    let target = "0".repeat(difficulty);
    hash.starts_with(&target)
}

// ============================================================================
// BLOCKCHAIN
// ============================================================================

/// A simple blockchain with difficulty adjustment.
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub target_block_time: u64,
}

impl Blockchain {
    /// Create a new blockchain with a genesis block.
    pub fn new(initial_difficulty: usize, target_block_time: u64) -> Blockchain {
        let genesis = Block::genesis(initial_difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty: initial_difficulty,
            target_block_time,
        }
    }

    /// Add a new block to the chain with the given data.
    /// The block is mined automatically.
    pub fn add_block(&mut self, data: String) -> MiningResult {
        let previous_block = self.chain.last().expect("Chain is empty");
        let mut new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
        );

        let result = new_block.mine();
        self.chain.push(new_block);
        result
    }

    /// Validate the entire blockchain.
    ///
    /// Checks that each block (except genesis) has:
    /// 1. A valid hash (meets difficulty and matches calculated hash)
    /// 2. A `previous_hash` matching the preceding block's hash
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if !current.is_valid() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }

        true
    }

    /// Get the number of blocks in the chain.
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    /// Check if the blockchain is empty (should never be, due to genesis).
    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    /// Get the latest block in the chain.
    pub fn latest_block(&self) -> &Block {
        self.chain.last().expect("Chain is empty")
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. SHA-256 HASHING
//    - Cryptographic hash: deterministic, avalanche effect, one-way
//    - sha2 crate uses optimized assembly on supported platforms
//
// 2. BRUTE FORCE SEARCH
//    - Mining tries nonces sequentially until hash meets target
//    - Cannot predict which nonce will work (hash is pseudorandom)
//
// 3. DIFFICULTY SCALING
//    - Difficulty N means hash must start with N hex zeros
//    - Each +1 difficulty multiplies expected attempts by 16
//    - Difficulty 1: ~16 attempts, Difficulty 4: ~65,536 attempts
//
// 4. MEMORY MANAGEMENT
//    - String allocations for hashes (heap allocated)
//    - Vec<Block> grows dynamically for the blockchain
//    - All memory freed automatically when out of scope
