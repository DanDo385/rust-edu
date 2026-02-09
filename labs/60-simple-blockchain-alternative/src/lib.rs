//! # Lab 60: Simple Blockchain - Your Implementation

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
    pub fn new(_index: u64, _data: String, _previous_hash: String) -> Self {
        todo!("Create a block and compute initial hash")
    }

    pub fn genesis() -> Self {
        todo!("Create genesis block")
    }

    pub fn calculate_hash(&self) -> String {
        todo!("Hash block fields with SHA-256")
    }

    pub fn mine(&mut self, _difficulty: usize) {
        todo!("Find nonce so hash satisfies difficulty")
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(_difficulty: usize) -> Self {
        todo!("Initialize chain with genesis block")
    }

    pub fn latest_block(&self) -> &Block {
        todo!("Return latest block")
    }

    pub fn add_block(&mut self, _data: String) {
        todo!("Mine and append new block")
    }

    pub fn is_valid(&self) -> bool {
        todo!("Validate hashes, links, and PoW")
    }
}

#[doc(hidden)]
pub mod solution;
