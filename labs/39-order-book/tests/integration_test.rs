// Integration tests for Lab 39: Order Book (Trading Engine)
//
// Tests order addition, matching, cancellation, price-time priority,
// best bid/ask, spread, and order book snapshots.

use order_book::{Order, OrderBook, OrderSide};

// ============================================================================
// EMPTY ORDER BOOK
// ============================================================================

#[test]
fn test_empty_order_book() {
    let book = OrderBook::new("BTC/USD");
    assert_eq!(book.symbol(), "BTC/USD");
    assert_eq!(book.bid_levels(), 0);
    assert_eq!(book.ask_levels(), 0);
    assert!(book.best_bid().is_none());
    assert!(book.best_ask().is_none());
    assert!(book.spread().is_none());
    assert!(book.trades().is_empty());
}

// ============================================================================
// ADDING NON-MATCHING ORDERS
// ============================================================================

#[test]
fn test_add_single_buy_order() {
    let mut book = OrderBook::new("ETH/USD");
    let trades = book.add_order(Order::new(1, OrderSide::Buy, 3000, 10));
    assert!(trades.is_empty()); // No match
    assert_eq!(book.best_bid(), Some(3000));
    assert_eq!(book.bid_levels(), 1);
    assert_eq!(book.bid_depth_at(3000), 10);
}

#[test]
fn test_add_single_sell_order() {
    let mut book = OrderBook::new("ETH/USD");
    let trades = book.add_order(Order::new(1, OrderSide::Sell, 3100, 5));
    assert!(trades.is_empty());
    assert_eq!(book.best_ask(), Some(3100));
    assert_eq!(book.ask_levels(), 1);
    assert_eq!(book.ask_depth_at(3100), 5);
}

#[test]
fn test_add_multiple_buy_orders_different_prices() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 49900, 10));
    book.add_order(Order::new(2, OrderSide::Buy, 49950, 5));
    book.add_order(Order::new(3, OrderSide::Buy, 49800, 20));

    assert_eq!(book.bid_levels(), 3);
    assert_eq!(book.best_bid(), Some(49950)); // Highest bid
    assert_eq!(book.bid_depth_at(49900), 10);
    assert_eq!(book.bid_depth_at(49950), 5);
    assert_eq!(book.bid_depth_at(49800), 20);
}

#[test]
fn test_add_multiple_sell_orders_different_prices() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Sell, 50100, 10));
    book.add_order(Order::new(2, OrderSide::Sell, 50050, 5));
    book.add_order(Order::new(3, OrderSide::Sell, 50000, 20));

    assert_eq!(book.ask_levels(), 3);
    assert_eq!(book.best_ask(), Some(50000)); // Lowest ask
}

#[test]
fn test_add_orders_same_price_level() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));
    book.add_order(Order::new(2, OrderSide::Buy, 100, 20));
    book.add_order(Order::new(3, OrderSide::Buy, 100, 30));

    assert_eq!(book.bid_levels(), 1);
    assert_eq!(book.bid_depth_at(100), 60); // 10 + 20 + 30
}

#[test]
fn test_spread_calculation() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 49950, 10));
    book.add_order(Order::new(2, OrderSide::Sell, 50050, 10));

    assert_eq!(book.spread(), Some(100)); // 50050 - 49950 = 100
}

// ============================================================================
// ORDER MATCHING
// ============================================================================

#[test]
fn test_exact_match_buy_meets_sell() {
    let mut book = OrderBook::new("BTC/USD");
    // Place a sell order
    book.add_order(Order::new(1, OrderSide::Sell, 100, 10));
    // Place a buy order at the same price
    let trades = book.add_order(Order::new(2, OrderSide::Buy, 100, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].buy_order_id, 2);
    assert_eq!(trades[0].sell_order_id, 1);
    assert_eq!(trades[0].price, 100); // Maker's price
    assert_eq!(trades[0].quantity, 10);

    // Both orders fully filled -- book should be empty
    assert_eq!(book.bid_levels(), 0);
    assert_eq!(book.ask_levels(), 0);
}

#[test]
fn test_exact_match_sell_meets_buy() {
    let mut book = OrderBook::new("BTC/USD");
    // Place a buy order
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));
    // Place a sell order at the same price
    let trades = book.add_order(Order::new(2, OrderSide::Sell, 100, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].buy_order_id, 1);
    assert_eq!(trades[0].sell_order_id, 2);
    assert_eq!(trades[0].price, 100);
    assert_eq!(trades[0].quantity, 10);

    assert_eq!(book.bid_levels(), 0);
    assert_eq!(book.ask_levels(), 0);
}

#[test]
fn test_partial_fill_buy_larger() {
    let mut book = OrderBook::new("BTC/USD");
    // Sell 5 at 100
    book.add_order(Order::new(1, OrderSide::Sell, 100, 5));
    // Buy 10 at 100 -- only 5 fills, 5 remains as resting bid
    let trades = book.add_order(Order::new(2, OrderSide::Buy, 100, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 5);

    assert_eq!(book.ask_levels(), 0); // Sell fully consumed
    assert_eq!(book.bid_levels(), 1); // Buy has 5 remaining
    assert_eq!(book.bid_depth_at(100), 5);
}

#[test]
fn test_partial_fill_sell_larger() {
    let mut book = OrderBook::new("BTC/USD");
    // Buy 5 at 100
    book.add_order(Order::new(1, OrderSide::Buy, 100, 5));
    // Sell 10 at 100 -- only 5 fills, 5 remains as resting ask
    let trades = book.add_order(Order::new(2, OrderSide::Sell, 100, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 5);

    assert_eq!(book.bid_levels(), 0);
    assert_eq!(book.ask_levels(), 1);
    assert_eq!(book.ask_depth_at(100), 5);
}

#[test]
fn test_buy_matches_multiple_sell_orders() {
    let mut book = OrderBook::new("BTC/USD");
    // Three sell orders at different prices
    book.add_order(Order::new(1, OrderSide::Sell, 100, 5));
    book.add_order(Order::new(2, OrderSide::Sell, 101, 5));
    book.add_order(Order::new(3, OrderSide::Sell, 102, 5));

    // Buy 12 at 102 -- should match all three sell levels
    let trades = book.add_order(Order::new(4, OrderSide::Buy, 102, 12));

    assert_eq!(trades.len(), 3);
    // First match at lowest ask price (100)
    assert_eq!(trades[0].price, 100);
    assert_eq!(trades[0].quantity, 5);
    // Second match at 101
    assert_eq!(trades[1].price, 101);
    assert_eq!(trades[1].quantity, 5);
    // Third match at 102 (partial: only 2 of 5)
    assert_eq!(trades[2].price, 102);
    assert_eq!(trades[2].quantity, 2);

    // Sell at 102 should have 3 remaining
    assert_eq!(book.ask_depth_at(102), 3);
    // Bid fully consumed
    assert_eq!(book.bid_levels(), 0);
}

#[test]
fn test_sell_matches_multiple_buy_orders() {
    let mut book = OrderBook::new("BTC/USD");
    // Three buy orders at different prices
    book.add_order(Order::new(1, OrderSide::Buy, 102, 5));
    book.add_order(Order::new(2, OrderSide::Buy, 101, 5));
    book.add_order(Order::new(3, OrderSide::Buy, 100, 5));

    // Sell 12 at 100 -- should match highest bids first
    let trades = book.add_order(Order::new(4, OrderSide::Sell, 100, 12));

    assert_eq!(trades.len(), 3);
    // First match at highest bid (102)
    assert_eq!(trades[0].price, 102);
    assert_eq!(trades[0].quantity, 5);
    // Second match at 101
    assert_eq!(trades[1].price, 101);
    assert_eq!(trades[1].quantity, 5);
    // Third match at 100 (partial: 2 of 5)
    assert_eq!(trades[2].price, 100);
    assert_eq!(trades[2].quantity, 2);

    assert_eq!(book.bid_depth_at(100), 3);
    assert_eq!(book.ask_levels(), 0);
}

#[test]
fn test_no_match_when_prices_dont_cross() {
    let mut book = OrderBook::new("BTC/USD");
    // Sell at 110
    book.add_order(Order::new(1, OrderSide::Sell, 110, 10));
    // Buy at 100 (below sell price -- no match)
    let trades = book.add_order(Order::new(2, OrderSide::Buy, 100, 10));

    assert!(trades.is_empty());
    assert_eq!(book.bid_levels(), 1);
    assert_eq!(book.ask_levels(), 1);
}

#[test]
fn test_trade_at_maker_price() {
    let mut book = OrderBook::new("BTC/USD");
    // Sell (maker) at 100
    book.add_order(Order::new(1, OrderSide::Sell, 100, 10));
    // Buy (taker) at 105 -- should trade at seller's price (100)
    let trades = book.add_order(Order::new(2, OrderSide::Buy, 105, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 100); // Maker's price, not taker's
}

// ============================================================================
// PRICE-TIME PRIORITY
// ============================================================================

#[test]
fn test_time_priority_fifo() {
    let mut book = OrderBook::new("BTC/USD");
    // Two sell orders at the same price, different IDs (timestamps)
    book.add_order(Order::new(1, OrderSide::Sell, 100, 10)); // First
    book.add_order(Order::new(2, OrderSide::Sell, 100, 10)); // Second

    // Buy 10 -- should match the FIRST sell order (time priority)
    let trades = book.add_order(Order::new(3, OrderSide::Buy, 100, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].sell_order_id, 1); // First order matched
    assert_eq!(book.ask_depth_at(100), 10); // Second order remains
}

#[test]
fn test_price_priority_best_price_first() {
    let mut book = OrderBook::new("BTC/USD");
    // Sell at 102 first, then sell at 100
    book.add_order(Order::new(1, OrderSide::Sell, 102, 10));
    book.add_order(Order::new(2, OrderSide::Sell, 100, 10));

    // Buy at 102 -- should match the CHEAPER sell first (price priority)
    let trades = book.add_order(Order::new(3, OrderSide::Buy, 102, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].sell_order_id, 2); // Cheaper order matched first
    assert_eq!(trades[0].price, 100);
}

#[test]
fn test_buy_price_priority() {
    let mut book = OrderBook::new("BTC/USD");
    // Buy at 98 first, then buy at 100
    book.add_order(Order::new(1, OrderSide::Buy, 98, 10));
    book.add_order(Order::new(2, OrderSide::Buy, 100, 10));

    // Sell at 98 -- should match the HIGHEST buy first
    let trades = book.add_order(Order::new(3, OrderSide::Sell, 98, 10));

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].buy_order_id, 2); // Higher bidder matched first
    assert_eq!(trades[0].price, 100); // At buyer's (maker) price
}

// ============================================================================
// ORDER CANCELLATION
// ============================================================================

#[test]
fn test_cancel_buy_order() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));
    assert_eq!(book.bid_levels(), 1);

    let cancelled = book.cancel_order(1);
    assert!(cancelled);
    assert_eq!(book.bid_depth_at(100), 0);
}

#[test]
fn test_cancel_sell_order() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Sell, 100, 10));
    assert_eq!(book.ask_levels(), 1);

    let cancelled = book.cancel_order(1);
    assert!(cancelled);
    assert_eq!(book.ask_depth_at(100), 0);
}

#[test]
fn test_cancel_nonexistent_order() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));

    let cancelled = book.cancel_order(999);
    assert!(!cancelled);
    // Original order still exists
    assert_eq!(book.bid_depth_at(100), 10);
}

#[test]
fn test_cancel_one_of_multiple_at_same_price() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));
    book.add_order(Order::new(2, OrderSide::Buy, 100, 20));
    assert_eq!(book.bid_depth_at(100), 30);

    book.cancel_order(1);
    assert_eq!(book.bid_depth_at(100), 20);
    assert_eq!(book.bid_levels(), 1); // Price level still exists
}

// ============================================================================
// SNAPSHOTS
// ============================================================================

#[test]
fn test_bid_snapshot_order() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));
    book.add_order(Order::new(2, OrderSide::Buy, 102, 20));
    book.add_order(Order::new(3, OrderSide::Buy, 101, 15));

    let snapshot = book.bid_snapshot();
    // Should be sorted highest to lowest
    assert_eq!(snapshot, vec![(102, 20), (101, 15), (100, 10)]);
}

#[test]
fn test_ask_snapshot_order() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Sell, 103, 10));
    book.add_order(Order::new(2, OrderSide::Sell, 101, 20));
    book.add_order(Order::new(3, OrderSide::Sell, 102, 15));

    let snapshot = book.ask_snapshot();
    // Should be sorted lowest to highest
    assert_eq!(snapshot, vec![(101, 20), (102, 15), (103, 10)]);
}

#[test]
fn test_empty_snapshots() {
    let book = OrderBook::new("BTC/USD");
    assert!(book.bid_snapshot().is_empty());
    assert!(book.ask_snapshot().is_empty());
}

// ============================================================================
// TRADE HISTORY
// ============================================================================

#[test]
fn test_trades_accumulate() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Sell, 100, 10));
    book.add_order(Order::new(2, OrderSide::Buy, 100, 10)); // Trade 1

    book.add_order(Order::new(3, OrderSide::Sell, 200, 5));
    book.add_order(Order::new(4, OrderSide::Buy, 200, 5)); // Trade 2

    assert_eq!(book.trades().len(), 2);
    assert_eq!(book.trades()[0].trade_id, 1);
    assert_eq!(book.trades()[1].trade_id, 2);
}

#[test]
fn test_trade_ids_are_sequential() {
    let mut book = OrderBook::new("BTC/USD");

    // Create multiple matches
    book.add_order(Order::new(1, OrderSide::Sell, 100, 5));
    book.add_order(Order::new(2, OrderSide::Sell, 101, 5));
    // This buy matches both sells
    book.add_order(Order::new(3, OrderSide::Buy, 101, 10));

    let trades = book.trades();
    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].trade_id, 1);
    assert_eq!(trades[1].trade_id, 2);
}

// ============================================================================
// COMPLEX SCENARIOS
// ============================================================================

#[test]
fn test_full_scenario_from_main() {
    // Reproduce the scenario from main.rs
    let mut book = OrderBook::new("BTC/USD");

    // Add sell orders
    book.add_order(Order::new(1, OrderSide::Sell, 50100, 10));
    book.add_order(Order::new(2, OrderSide::Sell, 50050, 5));
    book.add_order(Order::new(3, OrderSide::Sell, 50000, 20));

    // Add buy orders
    book.add_order(Order::new(4, OrderSide::Buy, 49900, 15));
    book.add_order(Order::new(5, OrderSide::Buy, 49950, 8));

    // No matches yet (spread exists)
    assert_eq!(book.trades().len(), 0);
    assert_eq!(book.best_bid(), Some(49950));
    assert_eq!(book.best_ask(), Some(50000));
    assert_eq!(book.spread(), Some(50)); // 50000 - 49950

    // Buy order that crosses spread
    let trades = book.add_order(Order::new(6, OrderSide::Buy, 50050, 12));
    // Should match: 12 units against sell at 50000 (qty 20)
    // Then nothing more since next sell is at 50050 and buy is at 50050 (still matches)
    assert!(!trades.is_empty());

    // Aggressive sell order
    let _trades = book.add_order(Order::new(7, OrderSide::Sell, 49900, 25));
}

#[test]
fn test_aggressive_buy_sweeps_entire_ask_side() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Sell, 100, 5));
    book.add_order(Order::new(2, OrderSide::Sell, 101, 5));
    book.add_order(Order::new(3, OrderSide::Sell, 102, 5));

    // Buy everything
    let trades = book.add_order(Order::new(4, OrderSide::Buy, 999, 15));
    assert_eq!(trades.len(), 3);
    assert_eq!(book.ask_levels(), 0);
    assert_eq!(book.bid_levels(), 0); // Fully consumed
}

#[test]
fn test_aggressive_sell_sweeps_entire_bid_side() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 102, 5));
    book.add_order(Order::new(2, OrderSide::Buy, 101, 5));
    book.add_order(Order::new(3, OrderSide::Buy, 100, 5));

    // Sell everything
    let trades = book.add_order(Order::new(4, OrderSide::Sell, 1, 15));
    assert_eq!(trades.len(), 3);
    assert_eq!(book.bid_levels(), 0);
    assert_eq!(book.ask_levels(), 0);
}

#[test]
fn test_depth_at_nonexistent_price() {
    let book = OrderBook::new("BTC/USD");
    assert_eq!(book.bid_depth_at(12345), 0);
    assert_eq!(book.ask_depth_at(12345), 0);
}

#[test]
fn test_spread_with_only_bids() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Buy, 100, 10));
    assert!(book.spread().is_none());
}

#[test]
fn test_spread_with_only_asks() {
    let mut book = OrderBook::new("BTC/USD");
    book.add_order(Order::new(1, OrderSide::Sell, 100, 10));
    assert!(book.spread().is_none());
}

#[test]
fn test_cancel_after_partial_fill() {
    let mut book = OrderBook::new("BTC/USD");
    // Sell 20 at 100
    book.add_order(Order::new(1, OrderSide::Sell, 100, 20));
    // Buy 5 at 100 -- fills 5, leaving 15 on sell side
    book.add_order(Order::new(2, OrderSide::Buy, 100, 5));
    assert_eq!(book.ask_depth_at(100), 15);

    // Cancel the remaining sell order
    let cancelled = book.cancel_order(1);
    assert!(cancelled);
    assert_eq!(book.ask_depth_at(100), 0);
}
