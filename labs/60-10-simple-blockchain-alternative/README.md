# Project 10: Simple Blockchain

## Overview
Build a basic blockchain combining concepts from previous projects. Implement blocks, chains, proof-of-work, and transaction validation.

## Concepts Taught
- **Blockchain fundamentals**: blocks, chains, immutability
- **Cryptographic linking**: each block references the previous
- **Genesis block**: the first block in the chain
- **Chain validation**: ensuring integrity
- **Combining previous concepts**: structs, enums, hashing, vectors

## Why Blockchain Works

### Immutability Through Cryptography
Each block contains:
1. Data (transactions)
2. Hash of previous block
3. Its own hash

Changing any block invalidates all subsequent blocks!

### Distributed Consensus
In real blockchains, multiple nodes agree on the state. This simple version shows the data structure fundamentals.

## Running This Project

```bash
cd 10-simple-blockchain
cargo run
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
sha2 = "0.10"
chrono = "0.4"
```
