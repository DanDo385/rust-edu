//! # Blockchain Node Demo

use blockchain_node::solution::{
    format_coins, validate_proof_of_work, Block, Blockchain, Transaction,
};

fn main() {
    println!("=== Blockchain Node Demo ===");

    let mut chain = Blockchain::new(2, 1_700_000_000);
    println!("genesis height={}", chain.height());

    let tx = Transaction::coinbase("miner1".to_string(), 5_000_000_000, 1_700_000_001, "coinbase-1".to_string());
    let prev_hash = chain
        .get_latest_block()
        .map(|b| b.hash.clone())
        .unwrap_or_else(|| "0".to_string());
    let mut block = Block::new(1, 1_700_000_002, vec![tx], prev_hash);
    block.mine(2);
    println!("mined block hash={}", block.hash);
    println!("pow valid={}", validate_proof_of_work(&block, 2));

    chain.add_block(block);
    println!("new height={} reward={}", chain.height(), format_coins(5_000_000_000));
    println!("chain valid={}", chain.is_valid());
}
