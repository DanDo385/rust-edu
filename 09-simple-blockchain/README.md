# Project 09 - Simple Blockchain (BLOCKCHAIN)

## What You're Building

Build a working blockchain with blocks, mining, and chain validation - the core of Bitcoin!

## Blockchain Context

This project implements:
- Block structure (header + data)
- Proof-of-Work mining (finding valid hashes)
- Chain validation (ensuring no tampering)
- Longest chain rule (consensus)

## Exercises

1. **Block**: Create block structure with hash, prev_hash, nonce
2. **mine_block**: Find valid hash with Proof-of-Work
3. **Blockchain**: Chain of blocks with validation
4. **add_block**: Mine and add new block
5. **is_valid**: Verify entire chain integrity

## How to Run

```bash
cargo test -p simple-blockchain
cargo run -p simple-blockchain
```
