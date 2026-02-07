// Project 36: Order Book (Trading Engine)
//
// This program implements a limit order book matching engine.
// It demonstrates how exchanges match buy and sell orders using
// price-time priority and BTreeMap for efficient sorted storage.

use std::collections::{BTreeMap, VecDeque};

fn main() {
    println!("=== Order Book (Trading Engine) ===\n");

    // ============================================================================
    // WHAT IS AN ORDER BOOK?
    // ============================================================================
    // An order book maintains all buy and sell orders for an asset.
    // It automatically matches orders when:
    // - A buy price >= sell price (prices cross)
    // - Orders are filled based on price-time priority
    //
    // Structure:
    // - BID side (buy orders): Highest price first
    // - ASK side (sell orders): Lowest price first
    // - SPREAD: Difference between best bid and best ask

    let mut order_book = OrderBook::new("BTC/USD".to_string());

    println!("=== Adding Orders ===\n");

    // Add some sell orders (asks)
    order_book.add_order(Order::new(1, OrderSide::Sell, 50100, 10));
    println!("Added: Sell 1.0 BTC @ $50,100");

    order_book.add_order(Order::new(2, OrderSide::Sell, 50050, 5));
    println!("Added: Sell 0.5 BTC @ $50,050");

    order_book.add_order(Order::new(3, OrderSide::Sell, 50000, 20));
    println!("Added: Sell 2.0 BTC @ $50,000");

    println!();

    // Add some buy orders (bids)
    order_book.add_order(Order::new(4, OrderSide::Buy, 49900, 15));
    println!("Added: Buy 1.5 BTC @ $49,900");

    order_book.add_order(Order::new(5, OrderSide::Buy, 49950, 8));
    println!("Added: Buy 0.8 BTC @ $49,950");

    println!();
    order_book.display();

    println!("\n=== Adding Order That Crosses Spread ===\n");

    // This buy order will match with the lowest sell order
    order_book.add_order(Order::new(6, OrderSide::Buy, 50050, 12));
    println!("Added: Buy 1.2 BTC @ $50,050 (should trigger matches)\n");

    order_book.display();

    println!("\n=== Adding Aggressive Sell Order ===\n");

    // This sell order will match multiple buy orders
    order_book.add_order(Order::new(7, OrderSide::Sell, 49900, 25));
    println!("Added: Sell 2.5 BTC @ $49,900 (should trigger matches)\n");

    order_book.display();

    println!();
}

// ============================================================================
// ORDER STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
struct Order {
    id: u64,
    side: OrderSide,
    price: u64,    // Price in cents to avoid floating-point errors
    quantity: u64, // Quantity in 0.1 units (e.g., 10 = 1.0 BTC)
    timestamp: u64, // For time priority (FIFO within price level)
}

impl Order {
    fn new(id: u64, side: OrderSide, price: u64, quantity: u64) -> Self {
        // In production, timestamp would be actual time
        Order {
            id,
            side,
            price,
            quantity,
            timestamp: id, // Using ID as timestamp for simplicity
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderSide {
    Buy,
    Sell,
}

// ============================================================================
// ORDER BOOK STRUCTURE
// ============================================================================

struct OrderBook {
    symbol: String,
    // BTreeMap keeps prices sorted automatically
    // For BIDS: Higher prices come first (reverse order)
    // For ASKS: Lower prices come first (natural order)
    bids: BTreeMap<u64, VecDeque<Order>>, // Buy orders by price
    asks: BTreeMap<u64, VecDeque<Order>>, // Sell orders by price
    next_trade_id: u64,
}

impl OrderBook {
    fn new(symbol: String) -> Self {
        OrderBook {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            next_trade_id: 1,
        }
    }

    /// Add an order and attempt to match it
    fn add_order(&mut self, mut order: Order) {
        match order.side {
            OrderSide::Buy => {
                // Try to match with sell orders
                self.match_buy_order(&mut order);

                // If there's remaining quantity, add to book
                if order.quantity > 0 {
                    self.bids
                        .entry(order.price)
                        .or_insert_with(VecDeque::new)
                        .push_back(order);
                }
            }
            OrderSide::Sell => {
                // Try to match with buy orders
                self.match_sell_order(&mut order);

                // If there's remaining quantity, add to book
                if order.quantity > 0 {
                    self.asks
                        .entry(order.price)
                        .or_insert_with(VecDeque::new)
                        .push_back(order);
                }
            }
        }
    }

    /// Match a buy order against sell orders
    fn match_buy_order(&mut self, buy_order: &mut Order) {
        // Get sell prices in ascending order (lowest first)
        let ask_prices: Vec<u64> = self.asks.keys().copied().collect();

        for ask_price in ask_prices {
            // If buy price < sell price, no match possible
            if buy_order.price < ask_price {
                break;
            }

            // Match against all orders at this price level
            if let Some(ask_queue) = self.asks.get_mut(&ask_price) {
                while let Some(mut sell_order) = ask_queue.pop_front() {
                    if buy_order.quantity == 0 {
                        // Put the sell order back and exit
                        ask_queue.push_front(sell_order);
                        break;
                    }

                    // Execute trade at the sell price (taker pays maker's price)
                    let trade_quantity = buy_order.quantity.min(sell_order.quantity);

                    // Inline trade execution to avoid double mutable borrow
                    let price_display = ask_price as f64 / 100.0;
                    let quantity_display = trade_quantity as f64 / 10.0;
                    println!(
                        "TRADE #{}: Buy Order #{} <-> Sell Order #{} | {} BTC @ ${:.2}",
                        self.next_trade_id, buy_order.id, sell_order.id,
                        quantity_display, price_display
                    );
                    self.next_trade_id += 1;

                    buy_order.quantity -= trade_quantity;
                    sell_order.quantity -= trade_quantity;

                    // If sell order still has quantity, put it back
                    if sell_order.quantity > 0 {
                        ask_queue.push_front(sell_order);
                        break;
                    }
                }

                // Remove empty price level
                if ask_queue.is_empty() {
                    self.asks.remove(&ask_price);
                }
            }

            if buy_order.quantity == 0 {
                break;
            }
        }
    }

    /// Match a sell order against buy orders
    fn match_sell_order(&mut self, sell_order: &mut Order) {
        // Get buy prices in descending order (highest first)
        let bid_prices: Vec<u64> = self.bids.keys().rev().copied().collect();

        for bid_price in bid_prices {
            // If sell price > buy price, no match possible
            if sell_order.price > bid_price {
                break;
            }

            // Match against all orders at this price level
            if let Some(bid_queue) = self.bids.get_mut(&bid_price) {
                while let Some(mut buy_order) = bid_queue.pop_front() {
                    if sell_order.quantity == 0 {
                        // Put the buy order back and exit
                        bid_queue.push_front(buy_order);
                        break;
                    }

                    // Execute trade at the buy price (taker pays maker's price)
                    let trade_quantity = sell_order.quantity.min(buy_order.quantity);

                    // Inline trade execution to avoid double mutable borrow
                    let price_display = bid_price as f64 / 100.0;
                    let quantity_display = trade_quantity as f64 / 10.0;
                    println!(
                        "TRADE #{}: Buy Order #{} <-> Sell Order #{} | {} BTC @ ${:.2}",
                        self.next_trade_id, buy_order.id, sell_order.id,
                        quantity_display, price_display
                    );
                    self.next_trade_id += 1;

                    sell_order.quantity -= trade_quantity;
                    buy_order.quantity -= trade_quantity;

                    // If buy order still has quantity, put it back
                    if buy_order.quantity > 0 {
                        bid_queue.push_front(buy_order);
                        break;
                    }
                }

                // Remove empty price level
                if bid_queue.is_empty() {
                    self.bids.remove(&bid_price);
                }
            }

            if sell_order.quantity == 0 {
                break;
            }
        }
    }

    /// Display the current order book
    fn display(&self) {
        println!("=== Order Book: {} ===", self.symbol);
        println!();

        // Display asks (sell orders) in reverse (highest to lowest)
        println!("ASKS (Sell Orders):");
        let ask_prices: Vec<u64> = self.asks.keys().rev().copied().collect();

        if ask_prices.is_empty() {
            println!("  (none)");
        } else {
            for price in ask_prices {
                if let Some(orders) = self.asks.get(&price) {
                    let total_quantity: u64 = orders.iter().map(|o| o.quantity).sum();
                    let price_display = price as f64 / 100.0;
                    let quantity_display = total_quantity as f64 / 10.0;
                    println!("  ${:>8.2} | {:>6.1} BTC ({} orders)",
                             price_display, quantity_display, orders.len());
                }
            }
        }

        println!("  -------------------- SPREAD");

        // Display bids (buy orders) in natural order (highest to lowest)
        println!("BIDS (Buy Orders):");
        let bid_prices: Vec<u64> = self.bids.keys().rev().copied().collect();

        if bid_prices.is_empty() {
            println!("  (none)");
        } else {
            for price in bid_prices {
                if let Some(orders) = self.bids.get(&price) {
                    let total_quantity: u64 = orders.iter().map(|o| o.quantity).sum();
                    let price_display = price as f64 / 100.0;
                    let quantity_display = total_quantity as f64 / 10.0;
                    println!("  ${:>8.2} | {:>6.1} BTC ({} orders)",
                             price_display, quantity_display, orders.len());
                }
            }
        }

        println!();

        // Display best bid and ask
        if let Some(best_bid) = self.bids.keys().rev().next() {
            println!("Best Bid: ${:.2}", *best_bid as f64 / 100.0);
        }
        if let Some(best_ask) = self.asks.keys().next() {
            println!("Best Ask: ${:.2}", *best_ask as f64 / 100.0);
        }
        if let (Some(best_bid), Some(best_ask)) =
            (self.bids.keys().rev().next(), self.asks.keys().next()) {
            let spread = (*best_ask as i64 - *best_bid as i64) as f64 / 100.0;
            println!("Spread:   ${:.2}", spread);
        }
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. BTREEMAP STRUCTURE
//    BTreeMap uses a B-tree (not binary tree) for better cache locality.
//    Each node contains multiple keys (typically 11-31) to fit in cache lines.
//    This gives O(log n) operations but with small constant factors.
//
// 2. VECDEQUE FOR FIFO
//    VecDeque is a double-ended queue implemented as a ring buffer.
//    push_back() and pop_front() are both O(1).
//    Perfect for maintaining time priority within a price level.
//
// 3. ENTRY API
//    .entry(price).or_insert_with(VecDeque::new)
//    This API avoids double lookup (check existence, then insert).
//    It's a zero-cost abstraction - compiles to optimal code.
//
// 4. INTEGER PRICES
//    We use u64 for prices (cents) instead of f64 to avoid:
//    - Rounding errors in financial calculations
//    - NaN and infinity edge cases
//    - Inconsistent equality comparisons
//    Real exchanges use fixed-point or decimal types.
//
// 5. MEMORY LAYOUT
//    - BTreeMap node: ~128-512 bytes per node
//    - VecDeque: 3 words (pointer, length, capacity) on stack
//    - Order: 40 bytes (5 * u64)
//    Total: O(n) where n = number of active orders
//
// 6. PERFORMANCE
//    - Add order: O(log p + m) where p = price levels, m = matched orders
//    - Best bid/ask: O(log p) to get first/last key
//    - Full book display: O(p) to iterate all price levels
//
//    For 1 million orders across 1000 price levels:
//    - Add: ~10 microseconds
//    - Match: Depends on number of fills
//    - Display: ~1 millisecond

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. BTreeMap keeps keys SORTED automatically (perfect for order books)
// 2. VecDeque provides O(1) FIFO operations for time priority
// 3. Price-time priority: Best price first, then oldest order
// 4. Use INTEGER arithmetic for money (avoid floating-point errors)
// 5. Entry API prevents redundant lookups
// 6. Matching algorithm crosses the spread when prices overlap
// 7. Maker-taker pricing: Trade executes at resting order's price
// 8. Real exchanges handle millions of orders per second

// ============================================================================
// PRICE-TIME PRIORITY ALGORITHM
// ============================================================================
// 1. Orders sorted by PRICE first:
//    - Bids: Highest price first (willing to pay more)
//    - Asks: Lowest price first (willing to sell for less)
//
// 2. Within same price, sorted by TIME (FIFO):
//    - Older orders fill first
//    - VecDeque maintains insertion order
//
// 3. Matching happens when:
//    - Buy price >= Sell price (prices cross)
//    - Trade executes at the MAKER's price (resting order)
//    - This incentivizes providing liquidity

// ============================================================================
// WHY THIS MATTERS FOR TRADING
// ============================================================================
// Order books are the foundation of:
// - All centralized exchanges (stocks, crypto, forex)
// - Market making strategies
// - High-frequency trading algorithms
// - Price discovery mechanisms
//
// Understanding order books helps you:
// - Read market depth and liquidity
// - Predict short-term price movements
// - Build trading bots
// - Understand slippage and market impact

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Use atomic types for concurrent access
// 2. Add order cancellation by ID
// 3. Implement market orders (execute immediately at any price)
// 4. Support time-in-force (IOC, FOK, GTC)
// 5. Add order book snapshots for recovery
// 6. Use memory pools to avoid allocations
// 7. Implement price discretization (round to tick size)
// 8. Add comprehensive error handling
// 9. Track order book metrics (depth, volume, etc.)
// 10. Use lock-free data structures for HFT

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Using f64 for prices (rounding errors in financial math)
// ❌ Not handling partial fills (order may match multiple times)
// ❌ Forgetting to remove empty price levels (memory leak)
// ❌ Wrong iteration order for bids vs asks
// ❌ Not implementing time priority (FIFO within price level)
// ❌ Matching at wrong price (should be maker's price, not taker's)
// ❌ Not checking if order is fully filled before adding to book
