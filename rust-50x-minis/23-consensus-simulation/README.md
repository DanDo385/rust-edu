# Project 23: Consensus Simulation

## Overview
Simulate distributed consensus using a simple voting algorithm. Learn how multiple nodes agree on a value through message passing, thread coordination, and Byzantine fault tolerance basics.

## Concepts Taught
- **Distributed consensus**: Multiple nodes agreeing on a single value
- **Voting algorithm**: Simple majority-based consensus
- **Message passing**: Communication between threads using channels
- **Thread coordination**: Synchronization using barriers and channels
- **Byzantine failures**: Handling faulty or malicious nodes
- **Leader election**: Selecting a coordinator node
- **Fault tolerance**: System continues despite node failures

## Why Consensus Matters

### Distributed Systems Problem
In blockchain and distributed databases:
- Nodes must agree on the current state
- Network delays and failures are common
- Some nodes may be malicious (Byzantine failures)
- No single source of truth

### Famous Consensus Algorithms
- **Paxos**: Classic algorithm, complex but proven
- **Raft**: More understandable, widely used (etcd, Consul)
- **PBFT**: Practical Byzantine Fault Tolerance (Hyperledger)
- **Proof of Work**: Bitcoin's consensus (longest chain)
- **Proof of Stake**: Ethereum 2.0 (validator selection)

## Comparison with Other Languages

| Aspect | Rust | Go | Python |
|--------|------|----|----|
| Channels | mpsc::channel | Built-in channels | queue.Queue |
| Thread safety | Compile-time guaranteed | Runtime panics | GIL limitations |
| Performance | No GC overhead | GC pauses | Slow (interpreted) |
| Message passing | Type-safe | Interface{} casting | Dynamic typing |

## Running This Project

```bash
cd 23-consensus-simulation
cargo run

# Try different configurations
cargo run -- --nodes 7 --faulty 2
```

## Consensus Algorithm (Simplified)

1. **Proposal Phase**: Leader proposes a value
2. **Voting Phase**: All nodes vote on the proposal
3. **Commit Phase**: If majority agrees, commit the value
4. **Fault Handling**: Retry if leader fails or no majority

**Byzantine tolerance**: Can tolerate f faulty nodes with 3f+1 total nodes
- 4 nodes: tolerate 1 fault
- 7 nodes: tolerate 2 faults
- 10 nodes: tolerate 3 faults

## Performance Considerations

**Message overhead:**
- N nodes = O(N²) messages for full consensus
- Leader-based reduces to O(N)

**Latency:**
- Network delay dominates
- Each phase adds one round-trip time

**Throughput:**
- Consensus is inherently sequential
- Batch multiple decisions to improve throughput

## Real-World Consensus Features

1. **View changes**: New leader if current fails
2. **Checkpointing**: Periodic state snapshots
3. **Recovery**: Nodes can rejoin after failure
4. **Reconfiguration**: Add/remove nodes dynamically
5. **Cryptographic verification**: Sign messages to detect forgery
6. **Timeout handling**: Detect and recover from network partitions

## Beginner Pitfalls

### Pitfall 1: Channel Deadlock
```rust
// ❌ Sender waits for receiver, receiver waits for sender
let (tx, rx) = mpsc::channel();
tx.send(data).unwrap();  // Blocks if channel full
let result = rx.recv().unwrap();  // Never reached!
```
**Fix**: Use bounded channels or spawn threads.

### Pitfall 2: Not Handling Disconnection
```rust
// ❌ Panics if sender is dropped
let data = rx.recv().unwrap();
```
**Fix**: Use `recv()` with Result handling or `try_recv()`.

### Pitfall 3: Infinite Waiting
```rust
// ❌ Waits forever if no message comes
let msg = rx.recv().unwrap();
```
**Fix**: Use `recv_timeout()` with Duration.

## Advanced Topics

1. **Practical Byzantine Fault Tolerance (PBFT)**: 3-phase commit protocol
2. **Raft consensus**: Leader election, log replication
3. **Tendermint**: BFT consensus for blockchain
4. **Gossip protocols**: Efficient message propagation
5. **Network partitions**: Handling split-brain scenarios

## Additional Challenges

1. **Implement leader election**: Automatic selection when leader fails
2. **Add view changes**: Rotate leader on failure
3. **Byzantine fault injection**: Random malicious nodes
4. **Network simulation**: Add delays and packet loss
5. **Persistent log**: Save consensus decisions to disk

## Next Steps

- **Project 24**: GUI with egui
- **Project 30**: Message bus for pub/sub
- **Project 49**: Full blockchain node

## Expected Output

```
=== Distributed Consensus Simulation ===

Configuration:
- Total nodes: 5
- Faulty nodes: 1
- Byzantine tolerance: 1 fault (3f+1 = 4 minimum)

Round 1: Proposing value 42
  Node 0 (Leader): proposed 42
  Node 1: voted YES
  Node 2: voted YES
  Node 3: voted NO (faulty)
  Node 4: voted YES

Result: CONSENSUS REACHED
  Value: 42
  Votes: 4/5 (80%)
  Faulty nodes detected: 1
```
