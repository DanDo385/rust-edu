//! # A Financial Order Book - Your Implementation
//!
//! This project involves building the core data structure of a financial
//! exchange: an order book.
//!
//! ## Your Task
//!
//! Implement the `OrderBook` and its matching logic.
//!
//! 1.  **Data Structures**: Define `Side`, `Order`, `Trade`, and `OrderBook`.
//!     -   `Order` should contain all the necessary information about an order.
//!     -   `OrderBook` should hold the `BTreeMap`s for bids and asks.
//!
//! 2.  **`new()`**: A constructor for the `OrderBook`.
//!
//! 3.  **`add_order()`**: This is the main engine. It takes a new order and
//!     attempts to match it against the book.
//!     -   If it's a buy order, match against asks (lowest price first).
//!     -   If it's a sell order, match against bids (highest price first).
//!     -   Generate `Trade`s for any matches.
//!     -   If the incoming order is not fully filled, add the remainder to the book.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p order-book
//! cargo run -p order-book
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use std::collections::BTreeMap;

// TODO: Define the Side enum (Buy or Sell)
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Side { ... }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

// TODO: Define the Order struct
// It should contain: id, side, price, quantity
// #[derive(Debug, Clone, Copy)]
// pub struct Order { ... }
#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
}

impl Order {
    pub fn new(id: u64, side: Side, price: u64, quantity: u64) -> Self {
        Self {
            id,
            side,
            price,
            quantity,
        }
    }
}

// TODO: Define the Trade struct
// It should contain: taker_order_id, maker_order_id, quantity, price
// #[derive(Debug)]
// pub struct Trade { ... }
#[derive(Debug)]
pub struct Trade {
    pub taker_order_id: u64,
    pub maker_order_id: u64,
    pub quantity: u64,
    pub price: u64,
}

// TODO: Define the OrderBook struct
// It should contain:
// - bids: A BTreeMap for buy orders
// - asks: A BTreeMap for sell orders
// - next_order_id: A counter for assigning unique order IDs
//
// The keys of the BTreeMaps should be the price, and the values
// should be a collection of all orders at that price level,
// for example, a `Vec<Order>`.
//
// pub struct OrderBook { ... }
pub struct OrderBook {
    pub bids: BTreeMap<u64, Vec<Order>>,
    pub asks: BTreeMap<u64, Vec<Order>>,
}


impl OrderBook {
    /// Creates a new, empty `OrderBook`.
    pub fn new() -> Self {
        todo!("Initialize the OrderBook");
    }

    /// Adds a new order to the book and performs matching.
    ///
    /// Returns a vector of trades that were executed.
    pub fn add_order(&mut self, mut order: Order) -> Vec<Trade> {
        // TODO: Implement the matching logic.
        // 1. Determine if the order is a Buy or Sell.
        //
        // 2. If it's a Buy order:
        //    - Look at the `asks` book (the sellers).
        //    - The best ask is the one with the LOWEST price.
        //    - Loop through the asks as long as the best ask price is less than
        //      or equal to the new order's price.
        //    - In the loop, create trades, update quantities, and remove
        //      filled orders.
        //
        // 3. If it's a Sell order:
        //    - Look at the `bids` book (the buyers).
        //    - The best bid is the one with the HIGHEST price.
        //    - A `BTreeMap` is sorted low-to-high, so you'll need to
        //      iterate in reverse (`.iter_mut().rev()`).
        //    - Loop and create trades just like for a buy order.
        //
        // 4. After the matching loop, if the incoming order still has quantity
        //    left (`order.quantity > 0`), add it to the correct side of the book.
        //
        // 5. Return the list of trades you generated.
        todo!("Implement the order matching engine");
    }
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
