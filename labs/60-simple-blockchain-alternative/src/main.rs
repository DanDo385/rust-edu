//! # Simple Blockchain - Demo

use simple_blockchain_alternative::solution::Blockchain;

fn main() {
    println!("=== Simple Blockchain Demo ===");

    let mut chain = Blockchain::new(2);
    chain.add_block("Alice -> Bob: 10".to_string());
    chain.add_block("Bob -> Charlie: 3".to_string());

    println!("blocks: {}", chain.chain.len());
    println!("latest hash: {}", chain.latest_block().hash);
    println!("valid: {}", chain.is_valid());

    chain.chain[1].data = "tampered".to_string();
    println!("valid after tamper: {}", chain.is_valid());
}
