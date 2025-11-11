//! # SHA-256 Hashing Demo

use sha256_hashing::solution::*;

fn main() {
    println!("=== SHA-256 Hashing (Blockchain) Demo ===\n");

    // Demo 1: Basic hashing
    println!("1. Basic SHA-256 Hashing:");
    let data = "Hello, Blockchain!";
    let hash = hash_string(data);
    println!("   Data: \"{}\"", data);
    println!("   Hash: {}\n", hash);

    // Demo 2: Avalanche effect
    println!("2. Avalanche Effect (small change = totally different hash):");
    let data1 = "Hello, Blockchain!";
    let data2 = "Hello, Blockchain."; // Changed ! to .
    println!("   Data 1: \"{}\"", data1);
    println!("   Hash 1: {}", hash_string(data1));
    println!("   Data 2: \"{}\"", data2);
    println!("   Hash 2: {}\n", hash_string(data2));

    // Demo 3: Hashing with nonce
    println!("3. Hashing with Nonce (Mining Simulation):");
    for nonce in 0..3 {
        let hash = hash_with_nonce("block data", nonce);
        println!("   Nonce {}: {}", nonce, hash);
    }
    println!();

    // Demo 4: Proof-of-Work (finding hash with prefix)
    println!("4. Proof-of-Work Mining:");
    println!("   Finding hash with prefix '00'...");
    let start = std::time::Instant::now();
    let (nonce, hash) = find_hash_with_prefix("block data", "00");
    let duration = start.elapsed();
    println!("   Found! Nonce: {}, Hash: {}", nonce, hash);
    println!("   Time: {:?}\n", duration);

    println!("   Finding hash with prefix '000' (harder)...");
    let start = std::time::Instant::now();
    let (nonce, hash) = find_hash_with_prefix("block data", "000");
    let duration = start.elapsed();
    println!("   Found! Nonce: {}, Hash: {}", nonce, hash);
    println!("   Time: {:?}\n", duration);

    // Demo 5: Verification
    println!("5. Hash Verification:");
    let original = "transaction data";
    let hash = hash_string(original);
    println!("   Original: \"{}\"", original);
    println!("   Hash: {}", hash);
    println!("   Verify original: {}", verify_hash(original, &hash));
    println!("   Verify tampered: {}", verify_hash("tampered data", &hash));

    println!("\n=== Demo Complete! ===");
    println!("\n🔗 Blockchain Insight:");
    println!("   Bitcoin miners do this billions of times per second!");
    println!("   Current Bitcoin difficulty ≈ 19 leading zeros");
    println!("   That's about 1 in 10^22 chance per attempt!");
}
