//! # Lab 60 Solution: Simple Blockchain
//!
//! ## Classroom Narrative
//!
//! 1. **Data layout**: Each `Block` packages index, timestamp, payload, previous hash, nonce, and hash together. These fields live inline (on the stack when the block is constructed) until the block is pushed into the chain’s `Vec<Block>`, which owns the data on the heap. Each `String` field (`data`, `previous_hash`, `hash`) points to heap bytes; the struct carries their addresses, lengths, and capacities on the stack.
//! 2. **Mining loop**: We change `nonce` and recompute `hash` until the digest satisfies the difficulty prefix. This requires `&mut self` to guarantee exclusive access to the block bytes. While the mutable borrow is active, Rust forbids shared borrows, which prevents data races or aliasing confusion.
//! 3. **Chain invariants**: Each new block clones (`clone()`) the previous block’s hash bytes. This is a value move of heap-owned bytes, not a borrow, so each block owns its own copy and no shared mutable state exists. Readers that call `latest_block(&self)` receive a shared borrow (`&Block`) and can inspect the chain without cloning anything.
//!
//! ### Symbol Drill
//!
//! - `&self` returns a shared borrow: only an address is passed, and the compiler makes sure the borrow doesn’t outlive the chain. Answering the original question: `&` doesn’t copy any data, it just hands the function an address to the existing block bytes on the heap and stack.
//! - `&mut self` establishes an exclusive borrow. The borrow checker enforces that no other references are alive when we mutate `nonce` and `hash`, so the mining loop cannot race with validation threads.
//! - `*` in this module is used for multiplication when constructing the prefix string (`"0".repeat(difficulty)`). It operates solely on counts and doesn’t dereference pointers; it acts on values, not addresses.
//!
//! ## Step-by-step Teaching Breakdown
//!
//! 1. **Genesis construction**: `Block::genesis()` calls `Block::new`, which samples `SystemTime` (a stack-local `Duration`), fills the fields, and immediately computes `hash`. Everything is moved into the block, showing how ownership and RAII work — when the genesis block is dropped, every heap `String` inside drops too.
//! 2. **Mining**: `mine(&mut self, difficulty)` loops while checking `self.hash.starts_with(&prefix)`. Each iteration mutates stack scalars (`nonce`) and recomputes the heap-owned `hash`. Because we hold `&mut self`, the borrow checker lets us mutate without clones. The SHA-256 context uses a small heap allocation inside `Sha256`, but the overall mutation stays confined to the block struct.
//! 3. **Appending**: `add_block` borrows the previous block with `let prev = self.latest_block()`, which gives an `&Block` shared borrow. We clone `prev.hash` so the new block receives its own owned copy. The new block is constructed, mined, and pushed into the `Vec<Block>` (heap). This shows how clones make sure ownership stays localized while reads remain cheap.
//! 4. **Validation**: `is_valid(&self)` loops over shared borrows (`&Block`). It re-calculates each block’s hash and checks links. Since the function only uses `&self`, the compiler allows multiple validations to run in parallel; the borrow checker knows we never mutate while reading.
//!
//! ## Mental Model Takeaways
//!
//! - Rust’s ownership system replaces GC: every block owns its strings, and the `Vec<Block>` owns the block sequence. When `Blockchain` is dropped, the heap bytes are freed automatically.
//! - Borrow rules keep miners and validators separate: miners take `&mut Block`, validators take `&Block`.
//! - `clone()` copies heap bytes when we need independent ownership, so earlier blocks remain immutable even after new blocks are added.
//! 
//! ## Test-verified invariants (integration_test.rs)
//! - Genesis block is immutable and always present (index 0, `previous_hash == "0"`).
//! - Adding a block increments the chain length and links via `previous_hash`, proving the `add_block` invariant.
//! - `is_valid` recomputes each hash and enforces difficulty per block, demonstrating that validation walks only shared borrows and never mutates the chain.

use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs();

        let mut block = Self {
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

    pub fn genesis() -> Self {
        Self::new(0, "Genesis Block".to_string(), "0".to_string())
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(
            format!(
                "{}{}{}{}{}",
                self.index, self.timestamp, self.data, self.previous_hash, self.nonce
            )
            .as_bytes(),
        );
        let digest = hasher.finalize();
        digest.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn mine(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce = self.nonce.saturating_add(1);
            self.hash = self.calculate_hash();
        }
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        Self {
            chain: vec![Block::genesis()],
            difficulty,
        }
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().expect("blockchain must have genesis")
    }

    pub fn add_block(&mut self, data: String) {
        let prev = self.latest_block();
        let mut block = Block::new(prev.index + 1, data, prev.hash.clone());
        block.mine(self.difficulty);
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
            if !current.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }
        }
        true
    }
}
