//! # A Financial Order Book - Complete Solution
//!
//! This module models an exchange's order book, matching buy and sell orders
//! deterministically while preserving price-time priority and ownership safety.
//!
//! ## Classroom Narrative
//! 1. **Data layout**: We store bids/asks as `BTreeMap<u64, Vec<Order>>`. Each key is a price (u64 scalar on the stack), and each value is a `Vec<Order>` owning the queued orders (heap). This gives O(log n) price lookup while retaining FIFO order within each price level.
//! 2. **Matching loop**: `add_order` inspects the opposite book via mutable borrows (`&mut self`). The borrow checker ensures that while we mutate bids or asks, no other borrows exist.
//! 3. **Trades & drops**: When trades consume orders, we mutate quantities and remove zero-quantity entries. When a `Vec<Order>` becomes empty, we drop it; Rust frees the inner order stack data (IDs/prices) automatically.
//!
//! ### Symbol Drill
//! - `&mut self` is used for `add_order`, `match_buy_order`, and `match_sell_order`. These methods mutate the book, so the borrow checker blocks simultaneous readers.
//! - `&self` is not used here, because every public method mutates state. If we later expose read-only views, we would need shared borrows.
//! - `*` is absent; arithmetic uses integers and `min`/`max` but never pointer dereference.
//!
//! ## Step-by-step Teaching Breakdown
//! 1. **Order arrival**: `add_order` takes ownership of `Order` (moved into the function). It determines side, mutably borrows the opposite book, and calls the matching helper.
//! 2. **Matching helpers**: `match_buy_order`/`match_sell_order` loop through price levels with mutable references to the queued `Vec<Order>`. They reduce quantities, accumulate trades, and remove empty orders. Each trade pushes a `Trade` struct onto a local `Vec<Trade>` — a stack-allocated vector owning its elements via heap memory.
//! 3. **Cleanup**: After matching, `add_order` appends the remaining order (if any) to the appropriate book (`bids` or `asks`). The addition uses `entry().or_default()` to mutate the map safely under the borrow checker.

use std::collections::BTreeMap;

/// Represents the side of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// Represents a single limit order.
#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
}

impl Order {
    pub fn new(id: u64, side: Side, price: u64, quantity: u64) -> Self {
        Order { id, side, price, quantity }
    }
}

/// Represents a trade that occurred by matching two orders.
#[derive(Debug)]
pub struct Trade {
    /// The ID of the order that initiated the trade (the "taker").
    pub taker_order_id: u64,
    /// The ID of the order that was resting on the book (the "maker").
    pub maker_order_id: u64,
    /// The quantity of the asset traded.
    pub quantity: u64,
    /// The price at which the trade occurred.
    pub price: u64,
}

/// The order book for a single financial instrument.
#[derive(Debug)]
pub struct OrderBook {
    /// Buy orders (bids), sorted by price from low to high.
    pub bids: BTreeMap<u64, Vec<Order>>,
    /// Sell orders (asks), sorted by price from low to high.
    pub asks: BTreeMap<u64, Vec<Order>>,
}

impl OrderBook {
    /// Creates a new, empty `OrderBook`.
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    /// Adds a new order to the book and performs matching.
    ///
    /// This is the core logic of the matching engine.
    pub fn add_order(&mut self, mut order: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        if order.side == Side::Buy {
            self.match_buy_order(&mut order, &mut trades);
        } else {
            self.match_sell_order(&mut order, &mut trades);
        }

        // If the order is not fully filled, add it to the book.
        if order.quantity > 0 {
            let book_side = if order.side == Side::Buy { &mut self.bids } else { &mut self.asks };
            book_side.entry(order.price).or_default().push(order);
        }

        trades
    }

    /// Tries to match a new buy order against existing sell orders.
    fn match_buy_order(&mut self, buy_order: &mut Order, trades: &mut Vec<Trade>) {
        // We need a list of ask prices that have been fully filled to remove them later.
        let mut filled_ask_prices = Vec::new();

        // Iterate through the asks from the lowest price (best ask) upwards.
        for (&price, orders_at_price) in self.asks.iter_mut() {
            // If the new buy order's price is not high enough to meet the current
            // sell price, no more matches are possible.
            if buy_order.price < price {
                break;
            }

            // Keep track of how many orders at this price level are fully filled.
            let mut filled_order_count = 0;
            for maker_order in orders_at_price.iter_mut() {
                if buy_order.quantity == 0 {
                    break;
                }
                
                // Determine the trade quantity and price
                let trade_quantity = buy_order.quantity.min(maker_order.quantity);
                let trade_price = maker_order.price;

                trades.push(Trade {
                    taker_order_id: buy_order.id,
                    maker_order_id: maker_order.id,
                    quantity: trade_quantity,
                    price: trade_price,
                });

                // Update quantities
                buy_order.quantity -= trade_quantity;
                maker_order.quantity -= trade_quantity;

                if maker_order.quantity == 0 {
                    filled_order_count += 1;
                }
            }

            // Remove the fully filled orders from the front of the queue.
            orders_at_price.drain(0..filled_order_count);
            
            // If the queue at this price level is now empty, mark it for removal.
            if orders_at_price.is_empty() {
                filled_ask_prices.push(price);
            }
            
            if buy_order.quantity == 0 {
                break;
            }
        }
        
        // Clean up the empty price levels from the asks book.
        for price in filled_ask_prices {
            self.asks.remove(&price);
        }
    }

    /// Tries to match a new sell order against existing buy orders.
    fn match_sell_order(&mut self, sell_order: &mut Order, trades: &mut Vec<Trade>) {
        let mut filled_bid_prices = Vec::new();

        // Iterate through the bids from the highest price (best bid) downwards.
        // `.iter_mut().rev()` is how we get this descending order from a BTreeMap.
        for (&price, orders_at_price) in self.bids.iter_mut().rev() {
            if sell_order.price > price {
                break;
            }

            let mut filled_order_count = 0;
            for maker_order in orders_at_price.iter_mut() {
                if sell_order.quantity == 0 {
                    break;
                }
                
                let trade_quantity = sell_order.quantity.min(maker_order.quantity);
                let trade_price = maker_order.price;

                trades.push(Trade {
                    taker_order_id: sell_order.id,
                    maker_order_id: maker_order.id,
                    quantity: trade_quantity,
                    price: trade_price,
                });

                sell_order.quantity -= trade_quantity;
                maker_order.quantity -= trade_quantity;

                if maker_order.quantity == 0 {
                    filled_order_count += 1;
                }
            }
            
            orders_at_price.drain(0..filled_order_count);
            
            if orders_at_price.is_empty() {
                filled_bid_prices.push(price);
            }
            
            if sell_order.quantity == 0 {
                break;
            }
        }

        for price in filled_bid_prices {
            self.bids.remove(&price);
        }
    }
}
