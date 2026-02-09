# Project 10 - Transaction Validation

## What You Will Build

You will build a minimal wallet + transaction signing flow with verification checks.

Pipeline:

1. Create wallet keypair
2. Build unsigned transaction
3. Sign transaction payload
4. Verify signature against public key

## Why This Lab Matters (First Principles)

Validation separates trusted and untrusted inputs.

- Signatures prove a holder of a private key approved specific bytes.
- Verification rejects tampered payloads.
- Address derivation connects identity to public-key bytes.

This lab uses an educational mock crypto model for teaching API flow.

## Memory Model and Ownership

### High-level ownership flow

```text
Wallet owns signing + verifying keys
   |
   | sign_transaction(&self, tx: &mut Transaction)
   v
transaction mutated in place with signature bytes
   |
   | verify_transaction(tx: &Transaction, key: &VerifyingKey)
   v
read-only validation over borrowed data
```

### Stack vs heap in this lab

- Stack:
  - key handles, numeric amount, references
- Heap:
  - transaction `String` fields (`from`, `to`)
  - optional signature bytes (`Option<Vec<u8>>`)
  - key byte vectors

### Concrete memory sketch

```text
Before signing:
  tx.signature = None

After signing:
  tx.signature = Some(Vec<u8>)

If tx.amount changes later, verify() recomputes payload hash and fails.
```

### Borrow checker behavior

- `sign_transaction(&self, transaction: &mut Transaction)`:
  - `&self` read-only borrow of wallet keys
  - `&mut Transaction` exclusive mutable borrow for writing signature
- `verify_transaction(&Transaction, &VerifyingKey)` is read-only and non-destructive.

## Rust Mental Models in This Lab

- Immutability by default protects most fields from accidental edits.
- Mutability is explicit at one sanctioned mutation point: attaching signature.
- Speed: verification compares compact byte digests; no heavy runtime reflection.
- Safety: signature presence modeled with `Option`, eliminating null misuse.

## Symbol Deep Dive

### `&` and `&mut`

- `&mut Transaction` means one writer at a time.
- `&Transaction` in verification means no mutation during checks.
- Misconception: mutable borrow is temporary; once scope ends, immutable borrows are allowed again.

### `*`

- `*` is arithmetic only when present in helper routines.
- No raw-pointer dereference is needed for transaction validation logic.

### Additional symbols used here

- `::` for constructors (`Wallet::new`)
- `->` for return contracts (`bool`, `String`, etc.)
- `Option<Vec<u8>>` to represent unsigned vs signed state explicitly

## Exercises

1. `Wallet::new` and `address`
- Goal: initialize keypair and derive deterministic address string.
- Constraints: address length and encoding consistency.
- Edge cases: uniqueness across wallets.
- Success: address tests pass.

2. `Wallet::sign_transaction`
- Goal: attach signature that covers transaction payload fields.
- Constraints: mutate only signature field.
- Edge cases: signing already-signed tx.
- Success: signature becomes `Some(...)`.

3. `verify_transaction`
- Goal: verify signature against transaction bytes and verifying key.
- Constraints: unsigned tx must fail.
- Edge cases: tampered amount, wrong key.
- Success: all valid/invalid verification tests pass.

## What Tests Prove

- Wallet tests prove key/address generation and uniqueness.
- Signing tests prove controlled mutation of transaction state.
- Verification tests prove integrity and key-binding invariants.

Failure interpretation:

- False positives for tampered tx indicate payload mismatch in signing vs verification paths.
- False negatives for valid tx indicate non-deterministic serialization/hashing.

## Performance Notes

- Signing and verification are O(n) in serialized transaction byte length.
- Borrowed verification path avoids extra copies of transaction strings.
- This mock crypto is lightweight for learning; production crypto has higher but predictable costs.

## How to Run

```bash
cargo run -p transaction-validation
cargo test -p transaction-validation
cargo check -p transaction-validation
```
