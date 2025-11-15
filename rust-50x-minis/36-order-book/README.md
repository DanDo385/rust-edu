# Project 36: Order Book (Trading Engine)

## Overview
Build a limit order book matching engine similar to those used in cryptocurrency and stock exchanges. Learn how trading systems match buyers with sellers using price-time priority, and explore Rust's BTreeMap for maintaining sorted price levels.

## Concepts Taught
- **BTreeMap** for sorted key-value storage
- **Price-time priority** algorithm
- **Order matching logic**
- **Enum for order types** (Buy/Sell)
- **Collections and iterators**
- **Decimal precision** handling (avoid floating-point errors)
- **Data structures** for financial systems
- **FIFO queues** within price levels

## Why Order Books?

Order books are the core of every modern exchange:
- **Stock markets**: NYSE, NASDAQ use order books
- **Crypto exchanges**: Coinbase, Binance, Kraken
- **Forex markets**: Currency trading platforms

### How It Works

1. **Limit Orders**: Traders specify price and quantity
2. **Price Levels**: Orders grouped by price, sorted best-to-worst
3. **Matching**: When buy price >= sell price, trade executes
4. **Priority**: At same price, older orders fill first (FIFO)

Example:
```
Sell Orders (Ask Side):
$102.00 | 50 shares
$101.50 | 100 shares
$101.00 | 25 shares
-------------------- Spread
$100.50 | 30 shares  <-- Best Bid
$100.00 | 75 shares
$99.50  | 200 shares
Buy Orders (Bid Side)
```

## Why BTreeMap?

A BTreeMap keeps keys sorted, which is perfect for order books:
- **O(log n)** insertions, lookups, and deletions
- **Automatic sorting** by price
- **Efficient iteration** in price order
- **Better cache locality** than HashMap for ordered data

## Trading Engine Context

Real exchanges process millions of orders per second. This simplified version demonstrates:
- How prices are matched
- Why sorted data structures matter
- How to maintain order priority
- The basics of market microstructure

## Running This Project

```bash
cd 36-order-book
cargo run
```

## Performance Considerations

**Order Book Complexity**:
- Insert order: O(log n) for price level + O(1) to append to queue
- Match orders: O(m) where m = number of matched orders
- View best bid/ask: O(log n) to get first/last key

**Real-world Optimizations**:
- Use price discretization (integer prices, not floats)
- Pre-allocate VecDeques for common price levels
- Use lock-free data structures for concurrency
- Memory-mapped I/O for persistence

**Memory Usage**:
- BTreeMap overhead: ~3-4 pointers per node
- VecDeque per price level: minimal when empty
- Total: O(n) where n = number of active orders

## Comparison: Rust vs C++ for Trading

| Feature | Rust | C++ |
|---------|------|-----|
| Memory safety | Guaranteed at compile-time | Manual, error-prone |
| Performance | Equivalent to C++ | Industry standard |
| BTreeMap | std::collections::BTreeMap | std::map (Red-Black tree) |
| Concurrency | Ownership prevents data races | Requires careful locking |
| Learning curve | Steeper initially | Familiar to many devs |

Most high-frequency trading systems are C++ for legacy reasons, but Rust is gaining adoption due to safety guarantees.

## Additional Challenges

1. **Market Orders**: Implement orders that execute at any price immediately

2. **Order Cancellation**: Add ability to cancel orders by ID

3. **Time-in-Force**: Support IOC (Immediate or Cancel), FOK (Fill or Kill), GTC (Good til Cancel)

4. **Order Book Depth**: Display the full order book with price levels and quantities

5. **Trading Statistics**: Track volume, number of trades, VWAP (Volume Weighted Average Price)

6. **Persistence**: Save/load order book state to disk

## Future Directions

- **Next**: Task scheduler for time-based execution (Project 37)
- **Later**: Build a high-frequency trading bot (Project 50)
- **Advanced**: Add concurrent order processing with lock-free structures (Project 27)

## Expected Output

You should see:
- Orders being added to the book
- Automatic matching when prices cross
- Best bid and ask prices
- Trade executions with price and quantity
- Remaining unfilled orders in the book
