# Project 39 - A Financial Order Book

## What You're Building (Plain English)

You're building the core component of a financial exchange, like a stock or cryptocurrency exchange. This component is called an "order book." It's a record of all the "buy" and "sell" orders that traders have placed for a particular asset (e.g., "RUST" coin).

-   **Buy Orders (Bids)**: "I want to buy 10 shares at $99."
-   **Sell Orders (Asks)**: "I want to sell 5 shares at $101."

Your order book will maintain two lists: one for all the buy orders and one for all the sell orders, sorted by price. The main job of the order book is to "match" a new incoming order with existing orders to create a "trade." For example, if a new "buy" order comes in at a price of $101, it will immediately match with the sell order at $101, and a trade occurs.

## New Rust Concepts in This Project

-   **`BTreeMap`**: A map where the keys are sorted. This is perfect for an order book, as we need to keep bids sorted from highest to lowest price and asks from lowest to highest. It gives us O(log n) access to the best prices.
-   **Complex Data Modeling**: You'll design several structs (`Order`, `Trade`, `OrderBook`) to accurately model the real-world concepts of a financial market.
-   **Stateful Logic**: The order book is a stateful system. Each new order can mutate the state by either being added to the book or by creating a trade and removing another order.
-   **Price-Time Priority**: You'll implement the fundamental rule of matching engines: orders are prioritized first by the best price, and then by the time they were submitted (first-in, first-out).

## Rust Syntax You'll See

```rust
use std::collections::BTreeMap;

// Bids are sorted high-to-low, but BTreeMap sorts low-to-high.
// A common trick is to store bid prices as their negative, or use
// a custom wrapper struct that reverses the ordering.
// For asks, the natural sort order is correct.
struct OrderBook {
    bids: BTreeMap<u64, Vec<Order>>, // Price -> Orders at that price
    asks: BTreeMap<u64, Vec<Order>>,
}

struct Order {
    id: u64,
    quantity: u64,
    // ... other fields
}

// Adding a buy order
// let price_level = book.bids.entry(order.price).or_default();
// price_level.push(order);

// Getting the best ask (lowest price)
// let best_ask = book.asks.iter_mut().next();
```

## How to Run

```bash
# Run the main binary (a demo of the order book in action)
cargo run -p order-book

# Run the tests
cargo test -p order-book

# Check if code compiles
cargo check -p order-book
```

## The Exercises

You will implement the `OrderBook` and its matching logic.

1.  **`Order` and `Trade` Structs**: Define the data structures for a single order (with an ID, side, price, and quantity) and a trade record.
2.  **`OrderBook` Struct**: The main struct. It will contain two `BTreeMap`s: one for bids and one for asks. The maps will go from a price level to a `Vec<Order>` (a queue of all orders at that price).
3.  **`add_order()`**: The main entry point. This function takes a new order.
    -   It first determines if the order is a `Buy` or a `Sell`.
    -   It then checks if this new order can be "matched" against the other side of the book.
        -   A new buy order is matched against the asks, starting from the *lowest* price ask.
        -   A new sell order is matched against the bids, starting from the *highest* price bid.
    -   The `add_order` function should loop, creating trades and consuming quantity from orders on both sides until the incoming order is either fully filled or no more matches can be made.
    -   If any quantity from the new order remains, it is added to the book.

## Solution Explanation (No Code - Just Ideas)

**Matching a New Buy Order**:
1.  Receive a new buy order: "Buy 10 units at $102."
2.  Look at the best ask (the sell order with the lowest price). Let's say it's "Sell 5 units at $101."
3.  Since the buy price ($102) is greater than or equal to the sell price ($101), a trade can happen!
4.  A trade is created for 5 units at $101 (the price of the order that was already on the book).
5.  The sell order is now fully filled and is removed.
6.  The incoming buy order still has `10 - 5 = 5` units left to fill.
7.  The matching engine looks at the *next* best ask. Let's say it's "Sell 8 units at $103."
8.  The buy price ($102) is *less than* the sell price ($103). No more matches can be made.
9.  The remaining 5 units of the buy order are added to the bid side of the book at its price of $102.

**Data Structures**:
-   **Bids (`BTreeMap<u64, ...>`)**: We want to match against the *highest* price bid first. A `BTreeMap` sorts keys in ascending order. So, to easily get the highest bid, we can use `.iter().rev().next()`.
-   **Asks (`BTreeMap<u64, ...>`)**: We want to match against the *lowest* price ask first. The natural iteration order of a `BTreeMap` (`.iter().next()`) gives us this.

## Where Rust Shines

-   **Data Structures**: `BTreeMap` provides the sorted-map functionality that is essential for an efficient order book.
-   **Enums**: Clearly modeling `Buy` vs `Sell` sides is a perfect use case for an enum.
-   **Ownership and Mutability**: Rust's rules make it clear how the `OrderBook`'s state is being mutated. The `&mut self` on `add_order` tells you that this method will change the book.
-   **Performance**: Rust's speed is critical for financial applications where matching engine performance is paramount.

## Common Beginner Mistakes

1.  **Price-Time Priority**: Forgetting that if two orders are at the same price, the one that arrived first should be filled first. This is why we use a `Vec` (acting as a FIFO queue) for each price level.
2.  **Partial Fills**: Not correctly handling the case where an incoming order is only partially filled. The remaining quantity needs to be handled correctly.
3.  **Floating Point for Money**: Using `f64` for prices is generally a bad idea due to precision issues. It's better to use fixed-point decimal types or, for simplicity in this lab, integers representing cents or basis points. We'll use `u64`.

This project is a fantastic simulation of a core financial technology system and a great exercise in complex, stateful data structure management.