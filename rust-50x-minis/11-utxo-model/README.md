# Project 11: UTXO Model

## Overview
This project implements the UTXO (Unspent Transaction Output) model used by Bitcoin and many other cryptocurrencies. You'll learn how UTXOs differ from account-based models, how to track unspent outputs, create transactions, and prevent double-spending attacks. This is a fundamental concept in blockchain design.

## Concepts Taught
- **UTXO (Unspent Transaction Output)** model architecture
- **Transaction inputs and outputs**
- **Double-spend prevention**
- **HashMap for efficient UTXO lookups**
- **Ownership transfer** in blockchain context
- **Transaction validation**
- **Balance calculation** from UTXOs
- **Change addresses** and transaction construction

## Why Rust Behaves This Way

### Ownership Maps to Real-World Asset Transfer
Rust's ownership model naturally maps to how UTXOs work! When you spend a UTXO:
1. The UTXO is **consumed** (moved out of the UTXO set)
2. New UTXOs are **created** (added to the UTXO set)
3. Once spent, the old UTXO **cannot be used again** (just like Rust's move semantics)

This makes Rust ideal for implementing blockchain systems - the compiler enforces the same rules that blockchain protocols require!

### UTXO vs Account-Based Models

**UTXO Model (Bitcoin, Cardano)**:
- Each transaction consumes inputs and creates outputs
- No global account balances - balance = sum of your UTXOs
- Parallel transaction processing (different UTXOs don't conflict)
- Better privacy (each transaction uses different addresses)
- More complex to understand initially

**Account-Based Model (Ethereum, traditional banking)**:
- Global state tracks each account's balance
- Transactions modify account balances directly
- Simpler mental model (like a bank account)
- Less privacy (balance tied to one address)
- Sequential processing (nonce prevents replay attacks)

## Why Use UTXO Model?

### Advantages
1. **Parallelization**: Transactions touching different UTXOs can be validated in parallel
2. **Stateless validation**: Each UTXO is independent, no need to track global state
3. **Privacy**: Each transaction can use new addresses
4. **Simplicity**: No complex state transitions, just set operations
5. **Auditability**: Easy to prove ownership of specific coins

### Disadvantages
1. **Complexity**: Harder for beginners to understand than accounts
2. **UTXO bloat**: Many small UTXOs can make transactions expensive
3. **Smart contracts**: Harder to implement than account-based (but possible - see Bitcoin Script, Cardano Plutus)

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting to Remove Spent UTXOs
```rust
// ❌ WRONG - spent UTXO still in the set
utxo_set.insert(new_utxo_id, new_utxo);
// Forgot to remove the input UTXO!
```
**Fix**: Always remove inputs when creating outputs:
```rust
// ✅ CORRECT
utxo_set.remove(&input_utxo_id);  // Remove spent
utxo_set.insert(new_utxo_id, new_utxo);  // Add new
```

### Pitfall 2: Not Validating Input Sum ≥ Output Sum
```rust
// ❌ WRONG - creating money from thin air
let total_input = 100;
let total_output = 150;  // More than input!
```
**Fix**: Always validate:
```rust
// ✅ CORRECT
if total_input < total_output {
    return Err("Insufficient funds");
}
```

### Pitfall 3: Double-Spending
```rust
// ❌ WRONG - using the same UTXO twice
create_transaction(utxo1, alice);
create_transaction(utxo1, bob);  // utxo1 already spent!
```
**Fix**: Check UTXO exists before spending:
```rust
// ✅ CORRECT
if !utxo_set.contains_key(&utxo_id) {
    return Err("UTXO already spent or doesn't exist");
}
```

### Pitfall 4: Ownership and Borrowing with HashMaps
```rust
// ❌ WRONG - trying to use value after moving it
let utxo = utxo_set.get(&id).unwrap();
utxo_set.remove(&id);  // ERROR: can't borrow as mutable while borrowed
```
**Fix**: Use separate lookups or clone:
```rust
// ✅ CORRECT - Option 1: Separate operations
if utxo_set.contains_key(&id) {
    utxo_set.remove(&id);
}

// ✅ CORRECT - Option 2: Remove returns the value
if let Some(utxo) = utxo_set.remove(&id) {
    // Use utxo here
}
```

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. UTXO data structure with unique IDs
2. Transaction creation and validation
3. UTXO set management (add/remove)
4. Balance calculation across multiple UTXOs
5. Change handling in transactions
6. Double-spend prevention
7. Transaction chain validation

## Performance Considerations

**HashMap Performance**:
- O(1) average case for UTXO lookup, insertion, deletion
- Critical for blockchain performance - Bitcoin has millions of UTXOs
- In production, use databases (RocksDB, LevelDB) for persistence

**Memory Usage**:
- Each UTXO requires memory (transaction ID + index + amount + script)
- UTXO set size is critical for node operators
- Bitcoin UTXO set is ~5GB (as of 2024)

**Validation Speed**:
- UTXO model allows parallel validation
- Each transaction can be validated independently
- Rust's zero-cost abstractions mean validation is as fast as C

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| HashMap | `HashMap<K, V>` (ownership-aware) | `map[K]V` (reference-based) | `dict` (reference-based) |
| UTXO ownership | Enforced by compiler | Manual tracking | Manual tracking |
| Remove from map | Returns `Option<V>` | Returns (value, bool) | Raises KeyError or returns None |
| Concurrency | Fearless (Send/Sync traits) | Goroutines + mutexes | GIL limitations |
| Double-spend prevention | Type system helps | Runtime checks | Runtime checks |
| Performance | Zero-cost abstractions | Garbage collection overhead | Interpreted, slower |

## Blockchain Context

### How Bitcoin Uses UTXOs

1. **Genesis Block**: Creates first UTXO (coinbase)
2. **Mining**: Each block creates coinbase UTXO (block reward)
3. **Transactions**: Spend UTXOs → Create new UTXOs
4. **Validation**: Full nodes maintain UTXO set, validate all new transactions
5. **Light Clients**: Use SPV (Simplified Payment Verification) with Merkle proofs

### Real-World Implementation Details

In production blockchain systems:
- UTXOs have **locking scripts** (Bitcoin Script, Plutus, etc.)
- **Unlocking** requires signature proving ownership
- **Dust limit**: Minimum UTXO value to prevent spam
- **UTXO consolidation**: Combining small UTXOs to reduce future tx fees

## Additional Challenges

1. **Script System**: Add locking/unlocking scripts to UTXOs (simplified version of Bitcoin Script)

2. **UTXO Consolidation**: Implement logic to combine many small UTXOs into fewer large ones

3. **Transaction Fees**: Implement fee calculation where `fee = sum(inputs) - sum(outputs)`

4. **Multi-signature**: Create UTXOs that require multiple signatures to spend

5. **Time-locked transactions**: UTXOs that can only be spent after a certain block height

6. **Persistence**: Save UTXO set to disk and reload it (use serde + JSON or bincode)

## Future Directions

- **Next**: Explore advanced lifetime annotations (Project 12)
- **Later**: Build a full transaction pool (Project 22)
- **Advanced**: Implement a complete blockchain node (Project 49)

## Running This Project

```bash
cd 11-utxo-model
cargo run
```

## Expected Output

You should see:
- Initial UTXO set (coinbase)
- Transaction creation and validation
- UTXO set updates (spending and creating)
- Balance calculations
- Double-spend prevention demonstration
- Final UTXO set state

The output will demonstrate how UTXOs are created, spent, and tracked throughout multiple transactions.
