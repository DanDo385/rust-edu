# Project 22: Transaction Pool (Mempool)

## Overview
Implement a transaction pool (mempool) that manages pending transactions in a blockchain system. Learn about concurrent data structures, priority queues, and transaction validation.

## Concepts Taught
- **Mempool concept**: Temporary storage for unconfirmed transactions
- **Transaction queuing**: FIFO, priority-based ordering
- **Concurrency**: Thread-safe data structures with Arc and Mutex
- **Transaction validation**: Basic checks before adding to pool
- **Priority management**: Higher fees get priority
- **Capacity limits**: Preventing memory exhaustion
- **Transaction removal**: After confirmation or timeout

## Why Mempools Matter

### Real Blockchain Context
In Bitcoin, Ethereum, and other blockchains:
1. Transactions are broadcast to the network
2. Nodes collect them in their mempool
3. Miners/validators select transactions for the next block
4. Higher fee transactions get priority
5. Confirmed transactions are removed from the pool

### Concurrency Challenges
Mempools must handle:
- Multiple threads adding transactions simultaneously
- Miners reading transactions while others are added
- Transaction removal without data races
- Memory limits to prevent DoS attacks

## Comparison with Other Languages

| Aspect | Rust | Go | Python |
|--------|------|----|----|
| Thread safety | Arc<Mutex<T>> explicit | Channels + mutexes | GIL (limited parallelism) |
| Memory safety | Compile-time guaranteed | Runtime panics possible | Runtime errors |
| Performance | No GC, zero-cost abstraction | GC pauses | Slow (interpreted) |
| Concurrency model | Ownership prevents races | Goroutines + channels | Threading with GIL |

## Running This Project

```bash
cd 22-transaction-pool
cargo run
```

## Performance Considerations

**Mutex contention:**
- Multiple threads competing for lock
- Keep critical sections small
- Consider lock-free structures for high throughput

**Memory usage:**
- Each transaction: ~200-500 bytes
- 10,000 transactions ≈ 2-5 MB
- Need eviction policy for memory limits

**Priority queue:**
- Insertion: O(log n)
- Removal: O(log n)
- Peek: O(1)

## Real-World Mempool Features

1. **Fee estimation**: Analyze pool to suggest appropriate fees
2. **Replace-by-fee**: Higher fee transaction replaces lower one
3. **Transaction expiration**: Remove old unconfirmed transactions
4. **Dependency tracking**: Child transactions depend on parent confirmation
5. **Size limits**: Max number of transactions or total bytes
6. **Anti-spam**: Rate limiting, minimum fee requirements

## Beginner Pitfalls

### Pitfall 1: Deadlocks with Multiple Locks
```rust
// ❌ Can deadlock if another thread locks in opposite order
let pool1 = pool.lock().unwrap();
let pool2 = another_pool.lock().unwrap();
```
**Fix**: Always acquire locks in the same order, or use a single lock.

### Pitfall 2: Holding Lock Too Long
```rust
// ❌ Slow operation while holding lock
let mut pool = mempool.lock().unwrap();
expensive_validation(&pool);  // Others blocked!
```
**Fix**: Release lock before expensive operations.

### Pitfall 3: Not Handling Poison
```rust
// ❌ Unwrap panics if another thread panicked while holding lock
let pool = mempool.lock().unwrap();
```
**Fix**: Handle poison error or use `expect()` with clear message.

## Advanced Topics

1. **Lock-free data structures**: Using atomic operations
2. **Sharding**: Multiple pools to reduce contention
3. **Persistent mempool**: Save to disk and restore on restart
4. **Network propagation**: Gossiping transactions to peers
5. **Transaction dependency graph**: Topological sorting

## Additional Challenges

1. **Add fee estimation**: Calculate recommended fee based on pool state
2. **Implement RBF**: Replace-by-fee for higher fee transactions
3. **Add transaction expiration**: Remove transactions after timeout
4. **Create benchmarks**: Measure throughput under concurrent load
5. **Add transaction dependencies**: Track parent-child relationships

## Next Steps

- **Project 23**: Consensus simulation with voting
- **Project 26**: Thread pool implementation
- **Project 49**: Full blockchain node with networking

## Expected Output

```
=== Transaction Pool (Mempool) ===

Adding transactions from multiple threads...
Added tx_001 with fee 50 satoshis
Added tx_002 with fee 100 satoshis
[...]

Pool status:
- Total transactions: 100
- Total fees: 5,250 satoshis
- Capacity: 100/1000

Selecting transactions for block...
Selected 10 highest-fee transactions
Total fees in block: 850 satoshis
```
