// Lab 25: Transaction Pool (Mempool)
//
// Implements a transaction pool for managing unconfirmed transactions.
// Demonstrates priority queues, hash maps for duplicate detection,
// and the core data structures behind blockchain transaction processing.
//
// ============================================================================
// OWNERSHIP & BORROWING COMMENTARY
// ============================================================================
// - Transaction uses String fields (owned data) so it can be stored independently
// - PriorityTransaction wraps Transaction to add Ord without orphan rule issues
// - TransactionPool owns both a BinaryHeap and HashMap (dual indexing)
// - get_top_transactions returns cloned Transactions (caller owns the copies)
// - remove_transaction returns Option<Transaction> (ownership transfer to caller)
// - stats() returns an owned PoolStats snapshot (no borrowing of pool internals)

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// ============================================================================
// TRANSACTION STRUCTURE
// ============================================================================

/// Represents an unconfirmed transaction waiting to be included in a block.
///
/// Each transaction has a unique ID, sender/receiver addresses, an amount,
/// a fee (used for priority ordering), a timestamp, and a nonce.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub nonce: u64,
}

impl Transaction {
    /// Creates a new transaction with the given parameters.
    ///
    /// The timestamp is provided as a parameter (rather than using SystemTime)
    /// to make the transaction deterministic and testable.
    pub fn new(id: &str, from: &str, to: &str, amount: u64, fee: u64, nonce: u64, timestamp: u64) -> Self {
        Transaction {
            id: id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee,
            timestamp,
            nonce,
        }
    }

    /// Validates a transaction against basic constraints.
    ///
    /// A valid transaction must have:
    /// - Non-empty id, from, and to fields
    /// - amount > 0
    /// - fee > 0
    /// - from != to (cannot send to yourself)
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.from.is_empty()
            && !self.to.is_empty()
            && self.amount > 0
            && self.fee > 0
            && self.from != self.to
    }
}

// ============================================================================
// PRIORITY TRANSACTION WRAPPER
// ============================================================================
// Wraps Transaction to implement Ord for BinaryHeap priority ordering.
// Higher fees get higher priority; ties broken by older timestamp first.

/// A wrapper around `Transaction` that implements ordering for use in a
/// `BinaryHeap`. Higher fees yield higher priority. When fees are equal,
/// older transactions (lower timestamp) are prioritized.
#[derive(Clone, Debug)]
pub struct PriorityTransaction(pub Transaction);

impl Eq for PriorityTransaction {}

impl PartialEq for PriorityTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0.fee == other.0.fee && self.0.timestamp == other.0.timestamp
    }
}

impl Ord for PriorityTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher fee = higher priority
        // If fees are equal, older transaction gets priority (lower timestamp first)
        self.0
            .fee
            .cmp(&other.0.fee)
            .then_with(|| other.0.timestamp.cmp(&self.0.timestamp))
    }
}

impl PartialOrd for PriorityTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// POOL STATISTICS
// ============================================================================

/// A snapshot of pool statistics at a point in time.
///
/// Returned by `TransactionPool::stats()` as an owned value, so the caller
/// does not borrow the pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolStats {
    pub total_transactions: usize,
    pub total_fees: u64,
    pub avg_fee: u64,
    pub min_fee: u64,
    pub max_fee: u64,
    pub capacity_used: usize,
    pub capacity_max: usize,
    pub rejected_count: u64,
}

// ============================================================================
// TRANSACTION POOL
// ============================================================================

/// A mempool that manages unconfirmed transactions using dual indexing:
/// - A `BinaryHeap<PriorityTransaction>` for efficient fee-based selection
/// - A `HashMap<String, Transaction>` for O(1) lookup and duplicate detection
///
/// The pool has a maximum capacity. Once full, new transactions are rejected.
pub struct TransactionPool {
    /// Priority queue for efficient fee-based selection
    priority_queue: BinaryHeap<PriorityTransaction>,
    /// HashMap for O(1) lookup and duplicate detection
    transactions: HashMap<String, Transaction>,
    /// Maximum number of transactions the pool can hold
    max_size: usize,
    /// Running total of all fees in the pool
    total_fees: u64,
    /// Count of rejected transactions (invalid, duplicate, or pool full)
    rejected_count: u64,
}

impl TransactionPool {
    /// Creates a new empty transaction pool with the given maximum capacity.
    pub fn new(max_size: usize) -> Self {
        TransactionPool {
            priority_queue: BinaryHeap::new(),
            transactions: HashMap::new(),
            max_size,
            total_fees: 0,
            rejected_count: 0,
        }
    }

    /// Adds a transaction to the pool.
    ///
    /// Returns `Ok(())` if the transaction was added successfully.
    /// Returns `Err(String)` if the transaction is invalid, a duplicate,
    /// or the pool is full.
    ///
    /// The rejected_count is incremented on any failure.
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Validate transaction
        if !tx.is_valid() {
            self.rejected_count += 1;
            return Err(format!("Invalid transaction: {}", tx.id));
        }

        // Check if already in pool (duplicate detection via HashMap)
        if self.transactions.contains_key(&tx.id) {
            self.rejected_count += 1;
            return Err(format!("Transaction already in pool: {}", tx.id));
        }

        // Check capacity
        if self.transactions.len() >= self.max_size {
            self.rejected_count += 1;
            return Err("Pool is full".to_string());
        }

        // Add to both data structures
        self.total_fees += tx.fee;
        self.transactions.insert(tx.id.clone(), tx.clone());
        self.priority_queue.push(PriorityTransaction(tx));

        Ok(())
    }

    /// Removes a transaction from the pool by its ID.
    ///
    /// Returns `Some(Transaction)` if the transaction was found and removed,
    /// or `None` if no transaction with that ID exists in the pool.
    ///
    /// Note: This rebuilds the priority queue since BinaryHeap does not
    /// support efficient removal by value.
    pub fn remove_transaction(&mut self, tx_id: &str) -> Option<Transaction> {
        if let Some(tx) = self.transactions.remove(tx_id) {
            self.total_fees = self.total_fees.saturating_sub(tx.fee);
            self.rebuild_priority_queue();
            Some(tx)
        } else {
            None
        }
    }

    /// Rebuilds the priority queue from the transactions HashMap.
    ///
    /// Called after removal since BinaryHeap does not support O(1) deletion
    /// by value. In production, a more sophisticated structure (e.g., indexed
    /// priority queue) would avoid this O(n log n) rebuild.
    fn rebuild_priority_queue(&mut self) {
        self.priority_queue.clear();
        for tx in self.transactions.values() {
            self.priority_queue.push(PriorityTransaction(tx.clone()));
        }
    }

    /// Returns the top N transactions by fee (highest fee first).
    ///
    /// This clones the internal heap to avoid consuming the pool's state.
    /// The returned Vec contains up to `n` transactions in descending fee order.
    pub fn get_top_transactions(&self, n: usize) -> Vec<Transaction> {
        let mut result = Vec::new();
        let mut heap_copy = self.priority_queue.clone();

        for _ in 0..n {
            if let Some(PriorityTransaction(tx)) = heap_copy.pop() {
                result.push(tx);
            } else {
                break;
            }
        }

        result
    }

    /// Returns a snapshot of the pool's current statistics.
    pub fn stats(&self) -> PoolStats {
        let avg_fee = if !self.transactions.is_empty() {
            self.total_fees / self.transactions.len() as u64
        } else {
            0
        };

        let min_fee = self.transactions.values().map(|tx| tx.fee).min().unwrap_or(0);

        let max_fee = self.transactions.values().map(|tx| tx.fee).max().unwrap_or(0);

        PoolStats {
            total_transactions: self.transactions.len(),
            total_fees: self.total_fees,
            avg_fee,
            min_fee,
            max_fee,
            capacity_used: self.transactions.len(),
            capacity_max: self.max_size,
            rejected_count: self.rejected_count,
        }
    }

    /// Clears all transactions from the pool, resetting fees to zero.
    ///
    /// Note: rejected_count is NOT reset by clear, since it tracks the
    /// lifetime total of rejected transactions.
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.priority_queue.clear();
        self.total_fees = 0;
    }

    /// Returns the number of transactions currently in the pool.
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Returns true if the pool contains no transactions.
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Returns true if a transaction with the given ID exists in the pool.
    pub fn contains(&self, tx_id: &str) -> bool {
        self.transactions.contains_key(tx_id)
    }

    /// Returns a reference to the transaction with the given ID, if it exists.
    pub fn get(&self, tx_id: &str) -> Option<&Transaction> {
        self.transactions.get(tx_id)
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. BINARY HEAP
//    BinaryHeap is a max-heap implemented as a Vec.
//    Insert and remove: O(log n)
//    Peek at max: O(1)
//    Not efficient for arbitrary removal (requires rebuild)
//
// 2. HASHMAP
//    HashMap uses SipHash by default (cryptographically strong but slower).
//    Lookup, insert, delete: average O(1)
//    Stores key-value pairs using hash table with open addressing.
//
// 3. DUAL INDEXING PATTERN
//    Using both a HashMap and BinaryHeap together gives us:
//    - O(1) lookups and duplicate detection (HashMap)
//    - O(log n) priority-based insertion/removal (BinaryHeap)
//    The trade-off is memory (storing data twice) and sync cost.
//
// 4. CLONE SEMANTICS
//    get_top_transactions clones the heap to avoid mutation.
//    Transaction derives Clone, so deep copies are made.
//    In production, Rc<Transaction> or indices could reduce cloning.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_new() {
        let tx = Transaction::new("tx1", "Alice", "Bob", 100, 10, 1, 1000);
        assert_eq!(tx.id, "tx1");
        assert_eq!(tx.from, "Alice");
        assert_eq!(tx.to, "Bob");
        assert_eq!(tx.amount, 100);
        assert_eq!(tx.fee, 10);
        assert_eq!(tx.nonce, 1);
        assert_eq!(tx.timestamp, 1000);
    }

    #[test]
    fn test_transaction_valid() {
        let tx = Transaction::new("tx1", "Alice", "Bob", 100, 10, 1, 1000);
        assert!(tx.is_valid());
    }

    #[test]
    fn test_transaction_invalid_empty_id() {
        let tx = Transaction::new("", "Alice", "Bob", 100, 10, 1, 1000);
        assert!(!tx.is_valid());
    }

    #[test]
    fn test_transaction_invalid_same_sender_receiver() {
        let tx = Transaction::new("tx1", "Alice", "Alice", 100, 10, 1, 1000);
        assert!(!tx.is_valid());
    }

    #[test]
    fn test_pool_new() {
        let pool = TransactionPool::new(100);
        assert_eq!(pool.len(), 0);
        assert!(pool.is_empty());
    }
}
