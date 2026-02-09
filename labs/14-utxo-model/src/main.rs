//! # UTXO Model Demo

use utxo_model::solution::{self, Transaction, TxInput, TxOutput, UtxoSet};

fn main() {
    println!("=== UTXO Model Demo ===\n");

    let mut utxo_set = UtxoSet::new();
    solution::create_genesis_utxo(&mut utxo_set, "genesis:0", "Alice", 100);
    solution::create_genesis_utxo(&mut utxo_set, "genesis:1", "Bob", 50);

    println!("initial balances:");
    println!("  Alice: {}", solution::get_balance(&utxo_set, "Alice"));
    println!("  Bob: {}", solution::get_balance(&utxo_set, "Bob"));

    let tx = Transaction::new(
        "tx1".to_string(),
        vec![TxInput::new("genesis:0".to_string(), "Alice".to_string())],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
            TxOutput::new("Alice".to_string(), 70),
        ],
    );

    let fee = solution::apply_transaction(&mut utxo_set, &tx).expect("valid transfer");
    println!("\ntransfer fee: {fee}");
    println!("balances after tx1:");
    println!("  Alice: {}", solution::get_balance(&utxo_set, "Alice"));
    println!("  Bob: {}", solution::get_balance(&utxo_set, "Bob"));
    println!("  Charlie: {}", solution::get_balance(&utxo_set, "Charlie"));
}
