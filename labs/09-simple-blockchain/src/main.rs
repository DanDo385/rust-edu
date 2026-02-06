//! # Simple Blockchain Demo

use simple_blockchain::solution::*;

fn main() {
    println!("=== Simple Blockchain Demo ===\n");

    // Create blockchain with difficulty 2 (2 leading zeros)
    println!("Creating blockchain (difficulty: 2)...");
    let mut blockchain = Blockchain::new(2);
    println!("Genesis block created!\n");

    // Add some blocks
    println!("Mining block 1...");
    blockchain.add_block("Alice pays Bob 10 BTC".to_string());

    println!("\nMining block 2...");
    blockchain.add_block("Bob pays Charlie 5 BTC".to_string());

    println!("\nMining block 3...");
    blockchain.add_block("Charlie pays Dave 3 BTC".to_string());

    // Display blockchain
    println!("\n=== Blockchain Contents ===");
    for block in &blockchain.chain {
        println!("\nBlock #{}:", block.index);
        println!("  Timestamp: {}", block.timestamp);
        println!("  Data: {}", block.data);
        println!("  Previous: {}...", &block.previous_hash[..16]);
        println!("  Hash: {}...", &block.hash[..16]);
        println!("  Nonce: {}", block.nonce);
    }

    // Validate blockchain
    println!("\n=== Validation ===");
    println!("Is blockchain valid? {}", blockchain.is_valid());

    // Try to tamper with blockchain
    println!("\n=== Tampering Attempt ===");
    println!("Changing block 2 data...");
    blockchain.chain[2].data = "Bob pays Charlie 50 BTC (tampered)".to_string();
    println!("Is blockchain valid? {}", blockchain.is_valid());

    println!("\n🔗 Blockchain Insight:");
    println!("   - Tampering breaks the chain!");
    println!("   - Would need to re-mine all subsequent blocks");
    println!("   - In real blockchain, need to beat the network");
    println!("   - This is why Bitcoin is secure!");

    println!("\n=== Demo Complete! ===");
}
