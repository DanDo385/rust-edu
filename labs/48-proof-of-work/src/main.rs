//! # Proof of Work - Demo

use proof_of_work::solution::{meets_difficulty, sha256_hex, Block, Blockchain};

fn main() {
    println!("=== Proof of Work Demo ===");

    let hash = sha256_hex(b"hello");
    println!("sha256(hello) = {}", hash);
    println!("meets difficulty 1? {}", meets_difficulty(&hash, 1));

    let mut block = Block::new(1, "transaction data".to_string(), "prev_hash".to_string(), 2);
    let result = block.mine();
    println!("mined hash={} attempts={} nonce={}", result.hash, result.attempts, result.nonce);

    let mut chain = Blockchain::new(2, 1);
    chain.add_block("Alice -> Bob: 10".to_string());
    chain.add_block("Bob -> Carol: 3".to_string());
    println!("chain len={} valid={}", chain.len(), chain.is_valid());
}
