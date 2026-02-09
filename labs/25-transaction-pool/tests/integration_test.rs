// Lab 25: Transaction Pool - Integration Tests
//
// Tests the transaction pool (mempool) implementation including:
// - Transaction creation and validation
// - Adding transactions to the pool
// - Duplicate rejection
// - Invalid transaction rejection
// - Pool capacity enforcement
// - Priority ordering by fee
// - Transaction removal
// - Pool statistics
// - Pool clearing

use transaction_pool::solution::{PoolStats, PriorityTransaction, Transaction, TransactionPool};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Creates a valid test transaction with sensible defaults.
fn make_tx(id: &str, fee: u64) -> Transaction {
    Transaction::new(id, "Alice", "Bob", 100, fee, 1, 1000)
}

/// Creates a valid test transaction with a specific timestamp.
fn make_tx_with_timestamp(id: &str, fee: u64, timestamp: u64) -> Transaction {
    Transaction::new(id, "Alice", "Bob", 100, fee, 1, timestamp)
}

/// Creates a valid test transaction with specific sender and receiver.
fn make_tx_full(id: &str, from: &str, to: &str, amount: u64, fee: u64) -> Transaction {
    Transaction::new(id, from, to, amount, fee, 1, 1000)
}

// ============================================================================
// TRANSACTION CREATION TESTS
// ============================================================================

#[test]
fn test_transaction_new_stores_all_fields() {
    let tx = Transaction::new("tx_001", "Alice", "Bob", 500, 25, 3, 9999);
    assert_eq!(tx.id, "tx_001");
    assert_eq!(tx.from, "Alice");
    assert_eq!(tx.to, "Bob");
    assert_eq!(tx.amount, 500);
    assert_eq!(tx.fee, 25);
    assert_eq!(tx.nonce, 3);
    assert_eq!(tx.timestamp, 9999);
}

#[test]
fn test_transaction_clone() {
    let tx = make_tx("tx_clone", 10);
    let cloned = tx.clone();
    assert_eq!(tx.id, cloned.id);
    assert_eq!(tx.fee, cloned.fee);
    assert_eq!(tx.amount, cloned.amount);
}

// ============================================================================
// TRANSACTION VALIDATION TESTS
// ============================================================================

#[test]
fn test_valid_transaction() {
    let tx = make_tx("tx1", 10);
    assert!(tx.is_valid());
}

#[test]
fn test_invalid_empty_id() {
    let tx = Transaction::new("", "Alice", "Bob", 100, 10, 1, 1000);
    assert!(!tx.is_valid());
}

#[test]
fn test_invalid_empty_from() {
    let tx = Transaction::new("tx1", "", "Bob", 100, 10, 1, 1000);
    assert!(!tx.is_valid());
}

#[test]
fn test_invalid_empty_to() {
    let tx = Transaction::new("tx1", "Alice", "", 100, 10, 1, 1000);
    assert!(!tx.is_valid());
}

#[test]
fn test_invalid_zero_amount() {
    let tx = Transaction::new("tx1", "Alice", "Bob", 0, 10, 1, 1000);
    assert!(!tx.is_valid());
}

#[test]
fn test_invalid_zero_fee() {
    let tx = Transaction::new("tx1", "Alice", "Bob", 100, 0, 1, 1000);
    assert!(!tx.is_valid());
}

#[test]
fn test_invalid_same_sender_receiver() {
    let tx = Transaction::new("tx1", "Alice", "Alice", 100, 10, 1, 1000);
    assert!(!tx.is_valid());
}

#[test]
fn test_valid_minimum_values() {
    // Minimum valid: amount=1, fee=1, different from/to
    let tx = Transaction::new("x", "A", "B", 1, 1, 0, 0);
    assert!(tx.is_valid());
}

// ============================================================================
// POOL CREATION TESTS
// ============================================================================

#[test]
fn test_pool_new_is_empty() {
    let pool = TransactionPool::new(100);
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_pool_new_stats_are_zero() {
    let pool = TransactionPool::new(50);
    let stats = pool.stats();
    assert_eq!(stats.total_transactions, 0);
    assert_eq!(stats.total_fees, 0);
    assert_eq!(stats.avg_fee, 0);
    assert_eq!(stats.min_fee, 0);
    assert_eq!(stats.max_fee, 0);
    assert_eq!(stats.capacity_used, 0);
    assert_eq!(stats.capacity_max, 50);
    assert_eq!(stats.rejected_count, 0);
}

// ============================================================================
// ADDING TRANSACTIONS TESTS
// ============================================================================

#[test]
fn test_add_single_transaction() {
    let mut pool = TransactionPool::new(100);
    let tx = make_tx("tx1", 10);
    assert!(pool.add_transaction(tx).is_ok());
    assert_eq!(pool.len(), 1);
    assert!(!pool.is_empty());
}

#[test]
fn test_add_multiple_transactions() {
    let mut pool = TransactionPool::new(100);
    for i in 0..5 {
        let tx = make_tx(&format!("tx_{}", i), 10 + i);
        assert!(pool.add_transaction(tx).is_ok());
    }
    assert_eq!(pool.len(), 5);
}

#[test]
fn test_add_transaction_updates_total_fees() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();
    pool.add_transaction(make_tx("tx3", 30)).unwrap();
    let stats = pool.stats();
    assert_eq!(stats.total_fees, 60);
}

#[test]
fn test_contains_after_add() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx_abc", 10)).unwrap();
    assert!(pool.contains("tx_abc"));
    assert!(!pool.contains("tx_xyz"));
}

#[test]
fn test_get_after_add() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx_full("tx1", "Alice", "Bob", 500, 25)).unwrap();
    let tx = pool.get("tx1").unwrap();
    assert_eq!(tx.amount, 500);
    assert_eq!(tx.fee, 25);
    assert_eq!(tx.from, "Alice");
}

#[test]
fn test_get_nonexistent_returns_none() {
    let pool = TransactionPool::new(100);
    assert!(pool.get("nonexistent").is_none());
}

// ============================================================================
// DUPLICATE REJECTION TESTS
// ============================================================================

#[test]
fn test_reject_duplicate_transaction() {
    let mut pool = TransactionPool::new(100);
    let tx1 = make_tx("tx1", 10);
    let tx2 = make_tx("tx1", 20); // Same ID, different fee

    assert!(pool.add_transaction(tx1).is_ok());
    let result = pool.add_transaction(tx2);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already in pool"));
}

#[test]
fn test_duplicate_increments_rejected_count() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    let _ = pool.add_transaction(make_tx("tx1", 20));
    assert_eq!(pool.stats().rejected_count, 1);
}

#[test]
fn test_pool_size_unchanged_after_duplicate() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    let _ = pool.add_transaction(make_tx("tx1", 20));
    assert_eq!(pool.len(), 1);
}

// ============================================================================
// INVALID TRANSACTION REJECTION TESTS
// ============================================================================

#[test]
fn test_reject_invalid_transaction() {
    let mut pool = TransactionPool::new(100);
    let invalid = Transaction::new("", "", "", 0, 0, 0, 0);
    let result = pool.add_transaction(invalid);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid transaction"));
}

#[test]
fn test_invalid_transaction_increments_rejected_count() {
    let mut pool = TransactionPool::new(100);
    let _ = pool.add_transaction(Transaction::new("tx1", "A", "A", 100, 10, 1, 1000));
    let _ = pool.add_transaction(Transaction::new("", "A", "B", 100, 10, 1, 1000));
    assert_eq!(pool.stats().rejected_count, 2);
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_reject_zero_amount() {
    let mut pool = TransactionPool::new(100);
    let tx = Transaction::new("tx1", "Alice", "Bob", 0, 10, 1, 1000);
    assert!(pool.add_transaction(tx).is_err());
}

#[test]
fn test_reject_zero_fee() {
    let mut pool = TransactionPool::new(100);
    let tx = Transaction::new("tx1", "Alice", "Bob", 100, 0, 1, 1000);
    assert!(pool.add_transaction(tx).is_err());
}

// ============================================================================
// POOL CAPACITY TESTS
// ============================================================================

#[test]
fn test_pool_full_rejects_new_transactions() {
    let mut pool = TransactionPool::new(3);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();
    pool.add_transaction(make_tx("tx3", 30)).unwrap();

    let result = pool.add_transaction(make_tx("tx4", 40));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Pool is full"));
}

#[test]
fn test_pool_full_increments_rejected_count() {
    let mut pool = TransactionPool::new(2);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();
    let _ = pool.add_transaction(make_tx("tx3", 30));
    assert_eq!(pool.stats().rejected_count, 1);
}

#[test]
fn test_pool_capacity_one() {
    let mut pool = TransactionPool::new(1);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    assert!(pool.add_transaction(make_tx("tx2", 20)).is_err());
    assert_eq!(pool.len(), 1);
}

#[test]
fn test_pool_accepts_after_removal_frees_space() {
    let mut pool = TransactionPool::new(2);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    // Pool is full
    assert!(pool.add_transaction(make_tx("tx3", 30)).is_err());

    // Remove one, now there's space
    pool.remove_transaction("tx1");
    assert!(pool.add_transaction(make_tx("tx3", 30)).is_ok());
    assert_eq!(pool.len(), 2);
}

// ============================================================================
// PRIORITY ORDERING TESTS
// ============================================================================

#[test]
fn test_top_transactions_ordered_by_fee_descending() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("low", 5)).unwrap();
    pool.add_transaction(make_tx("high", 50)).unwrap();
    pool.add_transaction(make_tx("mid", 25)).unwrap();

    let top = pool.get_top_transactions(3);
    assert_eq!(top.len(), 3);
    assert_eq!(top[0].id, "high");
    assert_eq!(top[0].fee, 50);
    assert_eq!(top[1].id, "mid");
    assert_eq!(top[1].fee, 25);
    assert_eq!(top[2].id, "low");
    assert_eq!(top[2].fee, 5);
}

#[test]
fn test_top_transactions_with_n_greater_than_pool_size() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    let top = pool.get_top_transactions(10);
    assert_eq!(top.len(), 2);
}

#[test]
fn test_top_transactions_returns_empty_for_empty_pool() {
    let pool = TransactionPool::new(100);
    let top = pool.get_top_transactions(5);
    assert!(top.is_empty());
}

#[test]
fn test_top_transactions_zero_requested() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    let top = pool.get_top_transactions(0);
    assert!(top.is_empty());
}

#[test]
fn test_top_one_returns_highest_fee() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 50)).unwrap();
    pool.add_transaction(make_tx("tx3", 30)).unwrap();

    let top = pool.get_top_transactions(1);
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].fee, 50);
}

#[test]
fn test_equal_fee_older_timestamp_first() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx_with_timestamp("newer", 10, 2000)).unwrap();
    pool.add_transaction(make_tx_with_timestamp("older", 10, 1000)).unwrap();

    let top = pool.get_top_transactions(2);
    assert_eq!(top.len(), 2);
    // With equal fees, older timestamp should come first
    assert_eq!(top[0].id, "older");
    assert_eq!(top[1].id, "newer");
}

#[test]
fn test_get_top_does_not_modify_pool() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    let _ = pool.get_top_transactions(2);
    assert_eq!(pool.len(), 2); // Pool unchanged
    let top_again = pool.get_top_transactions(2);
    assert_eq!(top_again.len(), 2); // Same result
}

// ============================================================================
// PRIORITY TRANSACTION ORDERING TESTS
// ============================================================================

#[test]
fn test_priority_transaction_higher_fee_is_greater() {
    let pt_low = PriorityTransaction(make_tx("low", 5));
    let pt_high = PriorityTransaction(make_tx("high", 50));
    assert!(pt_high > pt_low);
}

#[test]
fn test_priority_transaction_equal_fee_equal() {
    let pt1 = PriorityTransaction(make_tx_with_timestamp("a", 10, 1000));
    let pt2 = PriorityTransaction(make_tx_with_timestamp("b", 10, 1000));
    assert_eq!(pt1, pt2);
}

// ============================================================================
// TRANSACTION REMOVAL TESTS
// ============================================================================

#[test]
fn test_remove_existing_transaction() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();

    let removed = pool.remove_transaction("tx1");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().id, "tx1");
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_remove_nonexistent_transaction() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();

    let removed = pool.remove_transaction("tx_nonexistent");
    assert!(removed.is_none());
    assert_eq!(pool.len(), 1);
}

#[test]
fn test_remove_updates_fees() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    pool.remove_transaction("tx1");
    assert_eq!(pool.stats().total_fees, 20);
}

#[test]
fn test_remove_updates_priority_queue() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("low", 5)).unwrap();
    pool.add_transaction(make_tx("high", 50)).unwrap();
    pool.add_transaction(make_tx("mid", 25)).unwrap();

    // Remove the highest-fee transaction
    pool.remove_transaction("high");

    let top = pool.get_top_transactions(3);
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].id, "mid");
    assert_eq!(top[1].id, "low");
}

#[test]
fn test_remove_then_contains_is_false() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.remove_transaction("tx1");
    assert!(!pool.contains("tx1"));
}

#[test]
fn test_can_readd_after_removal() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.remove_transaction("tx1");

    // Should be able to add same ID again
    assert!(pool.add_transaction(make_tx("tx1", 20)).is_ok());
    assert_eq!(pool.get("tx1").unwrap().fee, 20);
}

// ============================================================================
// STATISTICS TESTS
// ============================================================================

#[test]
fn test_stats_single_transaction() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 42)).unwrap();

    let stats = pool.stats();
    assert_eq!(stats.total_transactions, 1);
    assert_eq!(stats.total_fees, 42);
    assert_eq!(stats.avg_fee, 42);
    assert_eq!(stats.min_fee, 42);
    assert_eq!(stats.max_fee, 42);
    assert_eq!(stats.capacity_used, 1);
    assert_eq!(stats.capacity_max, 100);
    assert_eq!(stats.rejected_count, 0);
}

#[test]
fn test_stats_multiple_transactions() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();
    pool.add_transaction(make_tx("tx3", 30)).unwrap();

    let stats = pool.stats();
    assert_eq!(stats.total_transactions, 3);
    assert_eq!(stats.total_fees, 60);
    assert_eq!(stats.avg_fee, 20); // 60 / 3 = 20
    assert_eq!(stats.min_fee, 10);
    assert_eq!(stats.max_fee, 30);
}

#[test]
fn test_stats_with_rejections() {
    let mut pool = TransactionPool::new(2);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    // These should all be rejected
    let _ = pool.add_transaction(make_tx("tx3", 30)); // Pool full
    let _ = pool.add_transaction(make_tx("tx1", 10)); // Pool full (checked before dup)
    let _ = pool.add_transaction(Transaction::new("", "", "", 0, 0, 0, 0)); // Invalid

    let stats = pool.stats();
    assert_eq!(stats.total_transactions, 2);
    assert_eq!(stats.rejected_count, 3);
}

#[test]
fn test_stats_capacity_tracking() {
    let mut pool = TransactionPool::new(5);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    let stats = pool.stats();
    assert_eq!(stats.capacity_used, 2);
    assert_eq!(stats.capacity_max, 5);
}

#[test]
fn test_stats_after_removal() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 30)).unwrap();
    pool.remove_transaction("tx1");

    let stats = pool.stats();
    assert_eq!(stats.total_transactions, 1);
    assert_eq!(stats.total_fees, 30);
    assert_eq!(stats.min_fee, 30);
    assert_eq!(stats.max_fee, 30);
}

// ============================================================================
// CLEAR TESTS
// ============================================================================

#[test]
fn test_clear_empties_pool() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    pool.clear();
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_clear_resets_fees() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.clear();
    assert_eq!(pool.stats().total_fees, 0);
}

#[test]
fn test_clear_does_not_reset_rejected_count() {
    let mut pool = TransactionPool::new(1);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    let _ = pool.add_transaction(make_tx("tx2", 20)); // Rejected: full

    pool.clear();
    assert_eq!(pool.stats().rejected_count, 1); // Still tracked
}

#[test]
fn test_can_add_after_clear() {
    let mut pool = TransactionPool::new(2);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.add_transaction(make_tx("tx2", 20)).unwrap();

    pool.clear();

    // Should be able to add again
    assert!(pool.add_transaction(make_tx("tx1", 10)).is_ok());
    assert!(pool.add_transaction(make_tx("tx2", 20)).is_ok());
    assert_eq!(pool.len(), 2);
}

#[test]
fn test_top_transactions_empty_after_clear() {
    let mut pool = TransactionPool::new(100);
    pool.add_transaction(make_tx("tx1", 10)).unwrap();
    pool.clear();
    assert!(pool.get_top_transactions(5).is_empty());
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_large_number_of_transactions() {
    let mut pool = TransactionPool::new(1000);
    for i in 0..100 {
        let tx = Transaction::new(
            &format!("tx_{}", i),
            &format!("sender_{}", i),
            &format!("receiver_{}", i),
            100 + i as u64,
            1 + i as u64,
            i as u64,
            1000 + i as u64,
        );
        assert!(pool.add_transaction(tx).is_ok());
    }
    assert_eq!(pool.len(), 100);

    // Top transaction should have the highest fee (100)
    let top = pool.get_top_transactions(1);
    assert_eq!(top[0].fee, 100);
}

#[test]
fn test_pool_stats_equality() {
    let stats1 = PoolStats {
        total_transactions: 5,
        total_fees: 100,
        avg_fee: 20,
        min_fee: 10,
        max_fee: 30,
        capacity_used: 5,
        capacity_max: 100,
        rejected_count: 0,
    };

    let stats2 = stats1.clone();
    assert_eq!(stats1, stats2);
}

#[test]
fn test_multiple_rejections_of_different_types() {
    let mut pool = TransactionPool::new(1);

    // Add one valid transaction
    pool.add_transaction(make_tx("tx1", 10)).unwrap();

    // Try invalid
    let _ = pool.add_transaction(Transaction::new("", "A", "B", 100, 10, 1, 1000));
    // Try duplicate (pool is also full, but invalid checked first would only matter
    // if pool weren't full - here pool is full so it hits capacity check)
    let _ = pool.add_transaction(make_tx("tx2", 20));

    // rejected_count should be 2 (one invalid, one pool full)
    assert_eq!(pool.stats().rejected_count, 2);
}
