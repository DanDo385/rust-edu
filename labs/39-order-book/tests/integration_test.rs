//! Integration tests for Lab 39: Order Book

use order_book::solution::{Order, OrderBook, Side};

#[test]
fn test_add_orders_no_match() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Buy, 99, 10));
    book.add_order(Order::new(2, Side::Sell, 101, 10));

    // The book should contain both orders as there's no price overlap.
    assert_eq!(book.bids.len(), 1);
    assert_eq!(book.asks.len(), 1);
}

#[test]
fn test_exact_match_buy_takes() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Sell, 100, 10)); // Maker

    // Taker order comes in and exactly matches the maker.
    let trades = book.add_order(Order::new(2, Side::Buy, 100, 10)); // Taker

    assert_eq!(trades.len(), 1);
    let trade = &trades[0];
    assert_eq!(trade.taker_order_id, 2);
    assert_eq!(trade.maker_order_id, 1);
    assert_eq!(trade.quantity, 10);
    assert_eq!(trade.price, 100);

    // The book should be empty now.
    assert!(book.bids.is_empty());
    assert!(book.asks.is_empty());
}

#[test]
fn test_exact_match_sell_takes() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Buy, 100, 10)); // Maker

    let trades = book.add_order(Order::new(2, Side::Sell, 100, 10)); // Taker

    assert_eq!(trades.len(), 1);
    // ... assertions similar to the buy test ...

    assert!(book.bids.is_empty());
    assert!(book.asks.is_empty());
}

#[test]
fn test_partial_fill_taker_is_larger() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Sell, 100, 5)); // Maker (5 shares)

    // Taker wants more than is available at this price.
    let trades = book.add_order(Order::new(2, Side::Buy, 100, 15)); // Taker (15 shares)

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 5); // Trade is for the maker's quantity

    // The ask side should now be empty.
    assert!(book.asks.is_empty());
    // The bid side should have the remainder of the taker order.
    assert_eq!(book.bids.len(), 1);
    let remaining_order = &book.bids.get(&100).unwrap()[0];
    assert_eq!(remaining_order.id, 2);
    assert_eq!(remaining_order.quantity, 10); // 15 - 5 = 10 remaining
}

#[test]
fn test_partial_fill_maker_is_larger() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Sell, 100, 20)); // Maker (20 shares)

    // Taker takes a portion of the available shares.
    let trades = book.add_order(Order::new(2, Side::Buy, 100, 8)); // Taker (8 shares)

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 8);

    // The bid side should be empty.
    assert!(book.bids.is_empty());
    // The ask side should have the maker order with reduced quantity.
    assert_eq!(book.asks.len(), 1);
    let remaining_order = &book.asks.get(&100).unwrap()[0];
    assert_eq!(remaining_order.id, 1);
    assert_eq!(remaining_order.quantity, 12); // 20 - 8 = 12 remaining
}

#[test]
fn test_buy_order_sweeps_multiple_asks() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Sell, 100, 5));
    book.add_order(Order::new(2, Side::Sell, 101, 5));

    // This buy order is large enough and priced high enough to fill both asks.
    let trades = book.add_order(Order::new(3, Side::Buy, 101, 12));

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].price, 100); // Matches best price first
    assert_eq!(trades[0].quantity, 5);
    assert_eq!(trades[1].price, 101);
    assert_eq!(trades[1].quantity, 5);

    // Ask book should be empty.
    assert!(book.asks.is_empty());
    // Remainder of the buy order is added to the bid book.
    assert_eq!(book.bids.get(&101).unwrap()[0].quantity, 2); // 12 - 5 - 5 = 2
}

#[test]
fn test_time_priority() {
    let mut book = OrderBook::new();
    book.add_order(Order::new(1, Side::Sell, 100, 5)); // Arrived first
    book.add_order(Order::new(2, Side::Sell, 100, 5)); // Arrived second

    // Taker order comes in. It should match with order 1 first.
    let trades = book.add_order(Order::new(3, Side::Buy, 100, 7));

    assert_eq!(trades.len(), 2);
    // First trade is with the first order in the queue (ID 1)
    assert_eq!(trades[0].maker_order_id, 1);
    assert_eq!(trades[0].quantity, 5);
    // Second trade is with the second order (ID 2)
    assert_eq!(trades[1].maker_order_id, 2);
    assert_eq!(trades[1].quantity, 2);

    // The first order is fully filled, the second is partially filled.
    assert_eq!(book.asks.get(&100).unwrap()[0].quantity, 3); // 5 - 2 = 3
    assert_eq!(book.asks.get(&100).unwrap()[0].id, 2);
}