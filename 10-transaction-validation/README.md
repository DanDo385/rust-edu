# Project 10 - Transaction Validation (BLOCKCHAIN)

## What You're Building

Implement digital signatures and transaction validation - how Bitcoin ensures only you can spend your coins!

## Blockchain Context

This project implements:
- Public/private key pairs (wallets)
- Digital signatures (proving you own coins)
- Transaction structure (inputs/outputs)
- Signature verification (validating transactions)

## Exercises

1. **Wallet**: Generate keypair (public/private keys)
2. **Transaction**: Create transaction structure
3. **sign_transaction**: Sign with private key
4. **verify_signature**: Verify with public key
5. **validate_transaction**: Full transaction validation

## How to Run

```bash
cargo test -p transaction-validation
cargo run -p transaction-validation
```
