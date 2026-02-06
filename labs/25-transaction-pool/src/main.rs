// Project 22: Transaction Pool (Mempool)
//
// Implements a transaction pool for managing unconfirmed transactions.
// Demonstrates thread-safe concurrent data structures, priority management,
// and the core component of blockchain transaction processing.

use colored::Colorize;
use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;

fn main() {
    println!("{}", "=== Transaction Pool (Mempool) ===".bright_blue().bold());
    println!();

    // Create a new transaction pool
    let pool = Arc::new(Mutex::new(TransactionPool::new(1000)));

    // Demonstrate basic operations
    demo_basic_operations(Arc::clone(&pool));
    println!();

    // Demonstrate concurrent access
    demo_concurrent_access(Arc::clone(&pool));
    println!();

    // Demonstrate transaction selection for mining
    demo_transaction_selection(Arc::clone(&pool));
    println!();

    // Show pool statistics
    print_pool_stats(&pool);
}

// ============================================================================
// TRANSACTION STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
struct Transaction {
    id: String,
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    timestamp: u64,
    nonce: u64,
}

impl Transaction {
    fn new(id: &str, from: &str, to: &str, amount: u64, fee: u64, nonce: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

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

    /// Basic transaction validation
    fn is_valid(&self) -> bool {
        // Check basic constraints
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
// Wraps Transaction to implement Ord for priority queue
// Higher fees get higher priority

#[derive(Clone)]
struct PriorityTransaction(Transaction);

impl Eq for PriorityTransaction {}

impl PartialEq for PriorityTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0.fee == other.0.fee
    }
}

impl Ord for PriorityTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher fee = higher priority
        // If fees are equal, older transaction gets priority
        self.0.fee.cmp(&other.0.fee)
            .then_with(|| other.0.timestamp.cmp(&self.0.timestamp))
    }
}

impl PartialOrd for PriorityTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// TRANSACTION POOL
// ============================================================================

struct TransactionPool {
    // Priority queue for efficient fee-based selection
    priority_queue: BinaryHeap<PriorityTransaction>,
    // HashMap for O(1) lookup and duplicate detection
    transactions: HashMap<String, Transaction>,
    // Maximum number of transactions
    max_size: usize,
    // Statistics
    total_fees: u64,
    rejected_count: u64,
}

impl TransactionPool {
    fn new(max_size: usize) -> Self {
        TransactionPool {
            priority_queue: BinaryHeap::new(),
            transactions: HashMap::new(),
            max_size,
            total_fees: 0,
            rejected_count: 0,
        }
    }

    /// Adds a transaction to the pool
    fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Validate transaction
        if !tx.is_valid() {
            self.rejected_count += 1;
            return Err(format!("Invalid transaction: {}", tx.id));
        }

        // Check if already in pool
        if self.transactions.contains_key(&tx.id) {
            self.rejected_count += 1;
            return Err(format!("Transaction already in pool: {}", tx.id));
        }

        // Check capacity
        if self.transactions.len() >= self.max_size {
            // Could implement eviction of lowest-fee transaction here
            self.rejected_count += 1;
            return Err("Pool is full".to_string());
        }

        // Add to both data structures
        self.total_fees += tx.fee;
        self.transactions.insert(tx.id.clone(), tx.clone());
        self.priority_queue.push(PriorityTransaction(tx.clone()));

        Ok(())
    }

    /// Removes a transaction from the pool
    fn remove_transaction(&mut self, tx_id: &str) -> Option<Transaction> {
        if let Some(tx) = self.transactions.remove(tx_id) {
            self.total_fees = self.total_fees.saturating_sub(tx.fee);

            // Note: BinaryHeap doesn't support efficient removal by value
            // In production, you'd rebuild the heap or use a more sophisticated structure
            // For this demo, we'll rebuild the heap
            self.rebuild_priority_queue();

            Some(tx)
        } else {
            None
        }
    }

    /// Rebuilds the priority queue from transactions map
    fn rebuild_priority_queue(&mut self) {
        self.priority_queue.clear();
        for tx in self.transactions.values() {
            self.priority_queue.push(PriorityTransaction(tx.clone()));
        }
    }

    /// Gets top N transactions by fee
    fn get_top_transactions(&self, n: usize) -> Vec<Transaction> {
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

    /// Gets pool statistics
    fn stats(&self) -> PoolStats {
        let avg_fee = if !self.transactions.is_empty() {
            self.total_fees / self.transactions.len() as u64
        } else {
            0
        };

        let min_fee = self.transactions.values()
            .map(|tx| tx.fee)
            .min()
            .unwrap_or(0);

        let max_fee = self.transactions.values()
            .map(|tx| tx.fee)
            .max()
            .unwrap_or(0);

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

    /// Clears all transactions
    fn clear(&mut self) {
        self.transactions.clear();
        self.priority_queue.clear();
        self.total_fees = 0;
    }
}

// ============================================================================
// POOL STATISTICS
// ============================================================================

struct PoolStats {
    total_transactions: usize,
    total_fees: u64,
    avg_fee: u64,
    min_fee: u64,
    max_fee: u64,
    capacity_used: usize,
    capacity_max: usize,
    rejected_count: u64,
}

// ============================================================================
// DEMONSTRATIONS
// ============================================================================

fn demo_basic_operations(pool: Arc<Mutex<TransactionPool>>) {
    println!("{}", "Basic Operations:".bright_yellow());

    let transactions = vec![
        Transaction::new("tx_001", "Alice", "Bob", 100, 10, 1),
        Transaction::new("tx_002", "Bob", "Charlie", 50, 20, 1),
        Transaction::new("tx_003", "Charlie", "Alice", 75, 5, 1),
        Transaction::new("tx_004", "Alice", "Charlie", 200, 50, 2),
    ];

    let mut pool_guard = pool.lock().unwrap();

    for tx in transactions {
        match pool_guard.add_transaction(tx.clone()) {
            Ok(_) => {
                println!(
                    "  {} Added {} ({} -> {}): {} satoshis, fee: {}",
                    "✓".green(),
                    tx.id.bright_white(),
                    tx.from,
                    tx.to,
                    tx.amount,
                    tx.fee
                );
            }
            Err(e) => {
                println!("  {} Failed to add {}: {}", "✗".red(), tx.id, e);
            }
        }
    }

    // Try to add a duplicate
    println!();
    let duplicate = Transaction::new("tx_001", "Alice", "Bob", 100, 10, 1);
    match pool_guard.add_transaction(duplicate) {
        Ok(_) => println!("  {} Added duplicate (unexpected)", "✗".red()),
        Err(e) => println!("  {} Rejected duplicate: {}", "✓".green(), e),
    }

    // Try to add invalid transaction
    let invalid = Transaction::new("", "", "", 0, 0, 0);
    match pool_guard.add_transaction(invalid) {
        Ok(_) => println!("  {} Added invalid (unexpected)", "✗".red()),
        Err(e) => println!("  {} Rejected invalid: {}", "✓".green(), e),
    }
}

fn demo_concurrent_access(pool: Arc<Mutex<TransactionPool>>) {
    println!("{}", "Concurrent Access:".bright_yellow());
    println!("  Spawning 5 threads to add transactions concurrently...");

    let mut handles = vec![];

    // Spawn multiple threads that add transactions
    for thread_id in 0..5 {
        let pool_clone = Arc::clone(&pool);

        let handle = thread::spawn(move || {
            for i in 0..10 {
                let tx_id = format!("tx_t{}_n{}", thread_id, i);
                let fee = (thread_id * 10 + i + 1) as u64;
                let tx = Transaction::new(
                    &tx_id,
                    &format!("Sender_{}", thread_id),
                    &format!("Receiver_{}", i),
                    100 + i as u64,
                    fee,
                    i as u64,
                );

                let mut pool_guard = pool_clone.lock().unwrap();
                let _ = pool_guard.add_transaction(tx);

                // Release lock and sleep briefly to simulate realistic timing
                drop(pool_guard);
                thread::sleep(Duration::from_micros(100));
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("  {} All threads completed", "✓".green());
}

fn demo_transaction_selection(pool: Arc<Mutex<TransactionPool>>) {
    println!("{}", "Transaction Selection (Mining):".bright_yellow());

    let pool_guard = pool.lock().unwrap();

    // Select top 10 transactions by fee
    let top_txs = pool_guard.get_top_transactions(10);

    println!("  Selecting top 10 transactions for block:");
    println!();

    let mut block_fees = 0;
    for (i, tx) in top_txs.iter().enumerate() {
        block_fees += tx.fee;
        println!(
            "    {}. {} - fee: {} satoshis (from {} to {})",
            i + 1,
            tx.id.bright_white(),
            tx.fee.to_string().bright_yellow(),
            tx.from,
            tx.to
        );
    }

    println!();
    println!("  Total fees in block: {} satoshis", block_fees.to_string().bright_green());
}

fn print_pool_stats(pool: &Arc<Mutex<TransactionPool>>) {
    println!("{}", "Pool Statistics:".bright_yellow());

    let pool_guard = pool.lock().unwrap();
    let stats = pool_guard.stats();

    println!("  Total transactions: {}", stats.total_transactions.to_string().bright_white());
    println!("  Total fees: {} satoshis", stats.total_fees.to_string().bright_green());
    println!("  Average fee: {} satoshis", stats.avg_fee);
    println!("  Min fee: {} satoshis", stats.min_fee);
    println!("  Max fee: {} satoshis", stats.max_fee);
    println!(
        "  Capacity: {}/{}",
        stats.capacity_used.to_string().bright_cyan(),
        stats.capacity_max
    );
    println!("  Rejected: {}", stats.rejected_count);

    let utilization = (stats.capacity_used as f64 / stats.capacity_max as f64) * 100.0;
    println!("  Utilization: {:.1}%", utilization);
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ARC (ATOMIC REFERENCE COUNTING)
//    Arc<T> uses atomic operations to track how many references exist.
//    When the last Arc is dropped, the data is freed.
//    Atomics are slower than regular variables but safe across threads.
//
// 2. MUTEX (MUTUAL EXCLUSION)
//    Mutex<T> ensures only one thread can access T at a time.
//    lock() blocks until the lock is acquired.
//    The lock is automatically released when the MutexGuard goes out of scope.
//    This prevents data races at compile time!
//
// 3. BINARY HEAP
//    BinaryHeap is a max-heap implemented as a Vec.
//    Insert and remove: O(log n)
//    Peek at max: O(1)
//    Not efficient for arbitrary removal (requires rebuild)
//
// 4. HASHMAP
//    HashMap uses SipHash by default (cryptographically strong but slower).
//    Lookup, insert, delete: average O(1)
//    Stores key-value pairs using hash table with open addressing.
//
// 5. THREAD SPAWNING
//    std::thread::spawn creates an OS thread (expensive: ~2MB stack).
//    Rust ensures thread safety at compile time through ownership.
//    Thread must join() or it's detached (keeps running after main exits).

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Transaction pools manage unconfirmed transactions
// 2. Priority queue enables fee-based selection
// 3. HashMap provides fast duplicate detection
// 4. Arc<Mutex<T>> enables safe concurrent access
// 5. Multiple data structures serve different purposes
// 6. Real mempools need eviction policies
// 7. Lock contention can be a bottleneck
// 8. Rust prevents data races at compile time
// 9. BinaryHeap is efficient for priority operations
// 10. Thread safety comes with performance cost

// ============================================================================
// REAL-WORLD MEMPOOL OPTIMIZATIONS
// ============================================================================
// 1. SHARDING
//    Split pool into multiple shards to reduce lock contention
//    Each shard handles a subset of transactions
//
// 2. LOCK-FREE STRUCTURES
//    Use atomic operations instead of mutexes
//    crossbeam crate provides lock-free queues
//
// 3. EVICTION POLICY
//    Remove lowest-fee transactions when pool is full
//    Or remove oldest transactions (FIFO)
//
// 4. PERSISTENT MEMPOOL
//    Save pool to disk periodically
//    Restore on node restart
//
// 5. TRANSACTION DEPENDENCIES
//    Track parent-child relationships
//    Ensure parent is included before child
//
// 6. RATE LIMITING
//    Prevent spam by limiting transactions per address
//    Require minimum fee

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not releasing lock before expensive operations
// ❌ Holding multiple locks (deadlock risk)
// ❌ Using unwrap() on lock (panics on poison)
// ❌ Forgetting to join() threads (orphaned threads)
// ❌ Not checking capacity before adding
// ❌ Inefficient removal from BinaryHeap (need rebuild)
// ❌ Not validating transactions before adding
// ❌ Allowing duplicate transactions
// ❌ No memory limits (DoS vulnerability)
// ❌ Not prioritizing by fee (inefficient block creation)
