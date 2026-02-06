# Project 07 - SHA256 Hashing (BLOCKCHAIN)

## What You're Building

Learn cryptographic hashing with SHA-256, the foundation of Bitcoin and most blockchains.

## Blockchain Context

SHA-256 is used in Bitcoin for:
- Block hashing (creating block identifiers)
- Proof-of-Work mining (finding hashes with leading zeros)
- Transaction IDs
- Merkle tree construction

## Exercises

1. **hash_string**: Hash a string with SHA-256
2. **hash_with_nonce**: Add nonce to input before hashing (mining simulation)
3. **find_hash_with_prefix**: Mine a hash with N leading zeros (proof-of-work)
4. **verify_hash**: Check if hash is valid for given input

## How to Run

```bash
cargo test -p sha256-hashing
cargo run -p sha256-hashing
```
