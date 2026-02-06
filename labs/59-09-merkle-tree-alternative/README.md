# Project 09: Merkle Tree

## Overview
Implement a Merkle tree for efficient data verification. Learn cryptographic hashing and tree structures, fundamental to blockchain technology.

## Concepts Taught
- **Cryptographic hashing** with SHA-256
- **Tree data structures** in Rust
- **Recursive algorithms**
- **Vec and heap allocation**
- **Using external crates** (sha2)
- **Data integrity verification**
- **Merkle proofs**

## Why Merkle Trees?

Merkle trees allow efficient verification that data belongs to a large set:
- **Bitcoin/Ethereum**: Verify transactions in a block without downloading the whole block
- **Git**: Track file changes efficiently
- **IPFS**: Content addressing

### Key Properties
1. Change any leaf → root hash changes
2. Can prove a leaf exists with O(log n) hashes
3. Efficient storage and verification

## Blockchain Context

Blockchains use Merkle trees to:
- Store transactions efficiently
- Allow light clients to verify transactions
- Ensure data hasn't been tampered with

## Running This Project

```bash
cd 09-merkle-tree
cargo run
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
sha2 = "0.10"
hex = "0.4"
```
