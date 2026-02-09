//! # Wallet CLI Demo

use wallet_cli::solution::{format_btc, Wallet};

fn main() {
    println!("=== Wallet CLI Demo ===");

    let mut wallet = Wallet::new("DemoWallet".to_string());
    let receive_addr = wallet.generate_address();
    wallet.receive_funds("funding_tx_001".to_string(), 0, 150_000_000, receive_addr);

    println!("Wallet: {}", wallet.name);
    println!("Root address: {}", wallet.get_root_address());
    println!("Balance: {} BTC", format_btc(wallet.get_balance()));

    match wallet.create_transaction("bc1qrecipient00000000000000000000000000000".to_string(), 50_000_000, 10) {
        Ok(tx) => {
            println!("Created tx: {}", tx.txid);
            println!("Inputs: {}, Outputs: {}, Fee: {} sat", tx.inputs.len(), tx.outputs.len(), tx.fee);
        }
        Err(err) => {
            println!("Transaction failed: {}", err);
        }
    }
}
