//! # Transaction Validation Demo

use transaction_validation::solution::*;

fn main() {
    println!("=== Transaction Validation (Blockchain) Demo ===\n");

    // Create wallets (keypairs)
    println!("1. Creating Wallets:");
    let alice = Wallet::new();
    let bob = Wallet::new();
    println!("   Alice's address: {}...", &alice.address()[..16]);
    println!("   Bob's address: {}...\n", &bob.address()[..16]);

    // Create transaction
    println!("2. Creating Transaction:");
    let mut tx = Transaction {
        from: alice.address(),
        to: bob.address(),
        amount: 100,
        signature: None,
    };
    println!("   From: {}...", &tx.from[..16]);
    println!("   To: {}...", &tx.to[..16]);
    println!("   Amount: {} satoshis\n", tx.amount);

    // Sign transaction
    println!("3. Signing Transaction:");
    alice.sign_transaction(&mut tx);
    let sig_preview: String = tx.signature.as_ref().unwrap()[..8]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    println!("   Signature: {}...", sig_preview);
    println!("   Transaction signed with Alice's private key!\n");

    // Verify transaction
    println!("4. Verifying Transaction:");
    let is_valid = verify_transaction(&tx, &alice.verifying_key);
    println!("   Valid signature from Alice? {}", is_valid);

    // Try to verify with wrong key
    let is_valid_bob = verify_transaction(&tx, &bob.verifying_key);
    println!("   Valid signature from Bob? {} (should be false)\n", is_valid_bob);

    // Try to tamper with transaction
    println!("5. Tampering Detection:");
    let mut tampered_tx = tx.clone();
    tampered_tx.amount = 1000; // Change amount
    println!("   Changed amount from 100 to 1000");
    let is_valid_tampered = verify_transaction(&tampered_tx, &alice.verifying_key);
    println!("   Is tampered transaction valid? {} (should be false)\n", is_valid_tampered);

    println!("🔗 Blockchain Insights:");
    println!("   - Private key: Never share! Like password.");
    println!("   - Public key: Share freely! Like email address.");
    println!("   - Signature: Proves you authorized transaction.");
    println!("   - Tampering: Any change invalidates signature.");
    println!("   - This is how Bitcoin ensures only you can spend your coins!");

    println!("\n=== Demo Complete! ===");
}
