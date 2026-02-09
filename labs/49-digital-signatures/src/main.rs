//! # Digital Signatures Demo

use digital_signatures::solution::{
    sign_transaction, verify_signed_message, verify_transaction, KeyPair, Transaction,
};

fn main() {
    println!("=== Digital Signatures Demo ===");

    let alice = KeyPair::generate();
    let msg = b"hello blockchain";
    let signed = alice.sign(msg);

    println!("Alice pubkey: {}", alice.public_key_hex());
    println!("Signature: {}", signed.signature_hex());
    println!(
        "Message verifies: {}",
        verify_signed_message(&signed, alice.verifying_key())
    );

    let tx = Transaction::new("Alice".into(), "Bob".into(), 42, 1);
    let signed_tx = sign_transaction(&tx, &alice);
    println!("Tx hash: {}", signed_tx.hash);
    println!(
        "Transaction verifies: {}",
        verify_transaction(&signed_tx, alice.verifying_key())
    );
}
