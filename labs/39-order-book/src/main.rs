//! # A Financial Order Book - Demo

use order_book::solution::{Order, OrderBook, Side};

fn main() {
    println!("=== Financial Order Book Demo ===");

    let mut book = OrderBook::new();

    book.add_order(Order::new(1, Side::Sell, 102, 10));
    book.add_order(Order::new(2, Side::Sell, 101, 5));
    book.add_order(Order::new(3, Side::Buy, 99, 8));
    book.add_order(Order::new(4, Side::Buy, 100, 12));

    println!("Initial book: {:#?}", book);

    let trades = book.add_order(Order::new(5, Side::Buy, 101, 7));
    println!("Trades after buy 7@101: {}", trades.len());
    for trade in &trades {
        println!("  {:?}", trade);
    }

    let trades = book.add_order(Order::new(6, Side::Sell, 98, 25));
    println!("Trades after sell 25@98: {}", trades.len());
    for trade in &trades {
        println!("  {:?}", trade);
    }

    println!("Final book: {:#?}", book);
}
