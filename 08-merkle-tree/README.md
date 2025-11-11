# Project 08 - Merkle Tree (BLOCKCHAIN)

## What You're Building

Implement a Merkle tree, the data structure Bitcoin uses to efficiently verify transactions in blocks.

## Blockchain Context

Merkle trees allow Bitcoin to:
- Verify a transaction is in a block without downloading the entire block
- Create compact proofs (only log(n) hashes needed)
- Used in SPV (Simplified Payment Verification) wallets

## Exercises

1. **MerkleTree::new**: Build tree from transactions
2. **root_hash**: Get the root hash (block's transaction hash)
3. **verify_proof**: Verify a transaction is in the tree
4. **generate_proof**: Create proof for a transaction

## How to Run

```bash
cargo test -p merkle-tree
cargo run -p merkle-tree
```
