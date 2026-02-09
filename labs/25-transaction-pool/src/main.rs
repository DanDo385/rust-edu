//! # Transaction Pool Demo

use transaction_pool::solution::{Transaction, TransactionPool};

fn main() {
    println!("=== Transaction Pool Demo ===\n");

    let mut pool = TransactionPool::new(100);
    let tx1 = Transaction::new("tx1", "Alice", "Bob", 100, 10, 1, 1000);
    let tx2 = Transaction::new("tx2", "Carol", "Dave", 200, 30, 1, 1001);

    pool.add_transaction(tx1).unwrap();
    pool.add_transaction(tx2).unwrap();

    println!("pool size: {}", pool.len());
    println!("top txs: {:?}", pool.get_top_transactions(2).iter().map(|t| &t.id).collect::<Vec<_>>());
    println!("stats: {:?}", pool.stats());
}
