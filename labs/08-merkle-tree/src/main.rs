//! # Merkle Tree Demo

use merkle_tree::solution::*;

fn main() {
    println!("=== Merkle Tree (Blockchain) Demo ===\n");

    // Simulate Bitcoin transactions
    let transactions = vec![
        "Alice pays Bob 1 BTC".to_string(),
        "Bob pays Charlie 0.5 BTC".to_string(),
        "Charlie pays Alice 0.3 BTC".to_string(),
        "Dave pays Eve 2 BTC".to_string(),
    ];

    println!("Transactions:");
    for (i, tx) in transactions.iter().enumerate() {
        println!("   {}. {}", i + 1, tx);
    }
    println!();

    // Build Merkle tree
    let tree = MerkleTree::new(transactions);

    println!("Merkle Tree:");
    println!("   Root hash: {}", tree.root_hash());
    println!("   This root goes in the Bitcoin block header!\n");

    println!("Leaf hashes (transaction IDs):");
    for (i, leaf) in tree.leaves.iter().enumerate() {
        println!("   {}. {}...", i + 1, &leaf[..16]);
    }

    println!("\n🔗 Blockchain Insight:");
    println!("   - Bitcoin blocks can have 2000+ transactions");
    println!("   - Merkle root: single hash representing ALL transactions");
    println!("   - SPV wallets verify transactions with log(n) proof size");
    println!("   - Example: 2048 transactions → only 11 hashes needed for proof!");

    println!("\n=== Demo Complete! ===");
}
