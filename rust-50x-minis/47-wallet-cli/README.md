# Project 47: Wallet CLI (CAPSTONE)

## Overview
Build a complete cryptocurrency wallet CLI that manages keys, creates transactions, selects UTXOs, and signs transactions. Integrates concepts from digital signatures, UTXO model, and blockchain projects.

## Concepts Taught
- **Wallet architecture**: key management, address generation, balance tracking
- **Key derivation**: HD wallets (BIP32/BIP44-inspired)
- **UTXO selection**: coin selection algorithms
- **Transaction construction**: inputs, outputs, change addresses
- **Fee estimation**: calculating appropriate transaction fees
- **Transaction signing**: integrating digital signatures
- **Persistence**: saving wallet state to disk
- **CLI design**: building user-friendly command-line interfaces

## Why Wallets Matter

### The Core Problem
Without wallets:
- Users must manually track their UTXOs
- No easy way to check balance
- Complex transaction construction
- Risk of losing private keys
- Difficult to spend coins

### What Wallets Provide
1. **Key Management**: Generate, store, and secure private keys
2. **Address Generation**: Create receiving addresses from keys
3. **Balance Tracking**: Monitor available funds across all UTXOs
4. **Transaction Building**: Automatically select UTXOs and create transactions
5. **Fee Management**: Calculate and include appropriate fees
6. **History**: Track sent and received transactions

## Wallet Types

### Non-Deterministic Wallets (Type 0)
- Each key is randomly generated
- Must backup each key individually
- Hard to manage many addresses
- Legacy approach (Bitcoin Core pre-0.13)

### Hierarchical Deterministic Wallets (HD)
- Single seed generates infinite keys
- One backup protects all future keys
- Organized key structure (accounts, change addresses)
- Standard: BIP32, BIP44
- Modern approach (all current wallets)

### Hardware Wallets
- Private keys never leave secure device
- Signs transactions in isolated environment
- Examples: Ledger, Trezor
- Gold standard for security

## UTXO Selection Algorithms

### 1. Largest First
- Select largest UTXOs first
- Minimizes number of inputs
- Creates smaller transactions
- Risk: leaves only small UTXOs (dust)

### 2. Smallest First
- Select smallest UTXOs first
- Consolidates dust
- May create larger transactions
- Good for cleaning up wallet

### 3. Random Selection
- Improves privacy
- Harder to link transactions
- May not be optimal for fees

### 4. Branch and Bound (Optimal)
- Finds exact match if possible
- Minimizes change output
- Best for privacy and fees
- More complex to implement

## Transaction Construction

### Components
1. **Inputs**: References to UTXOs being spent
   - Previous transaction hash
   - Output index
   - Signature/unlocking script
   - Amount (for fee calculation)

2. **Outputs**: New UTXOs being created
   - Recipient address
   - Amount
   - Locking script

3. **Metadata**:
   - Version
   - Locktime
   - Fees (implicit: sum(inputs) - sum(outputs))

### Change Addresses
When input amount > payment amount + fee:
- Create change output to return excess
- Use new address for privacy
- Add to wallet's UTXO set

## Fee Estimation

### Fee Models
- **Per-byte**: Common in Bitcoin (satoshis/byte)
- **Gas price**: Ethereum approach (gwei * gas_limit)
- **Fixed**: Simple blockchains

### Fee Calculation
```
total_input = sum of all input amounts
total_output = payment_amount + change_amount
fee = total_input - total_output

Required:
fee >= transaction_size * fee_rate
```

### Fee Strategies
- **Low priority**: Cheap but slow confirmation
- **Medium**: Balanced approach
- **High priority**: Fast confirmation, expensive
- **Custom**: User specifies exact fee

## Security Considerations

### Key Storage
❌ **Plaintext**: Never store private keys unencrypted
✅ **Encrypted**: Use password-derived encryption (scrypt, Argon2)
✅ **Hardware**: Best option for large amounts

### Seed Phrases
- 12 or 24 word mnemonic (BIP39)
- Human-readable backup
- Must be stored securely offline
- Anyone with seed controls all funds!

### Address Reuse
❌ **Bad**: Using same address multiple times
- Reduces privacy
- Links transactions together
- Exposes public key (pre-quantum risk)

✅ **Good**: Generate new address for each transaction
- Improves privacy
- Harder to track balances
- Standard practice

## Running This Project

```bash
cd 47-wallet-cli
cargo run
```

**Dependencies** (add to `Cargo.toml`):
```toml
[dependencies]
k256 = { version = "0.13", features = ["ecdsa"] }
sha2 = "0.10"
rand = "0.8"
hex = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Expected Output
```
=== Cryptocurrency Wallet CLI ===

--- Creating New Wallet ---
✅ Wallet created successfully!
Master key generated
Root address: bc1q7x9w8...

--- Wallet Status ---
Balance: 0.00000000 BTC
Addresses: 1
UTXOs: 0

--- Receiving Funds ---
Generated receiving address: bc1qm4k3...
Simulating receiving 1.5 BTC...
✅ Received 1.50000000 BTC

--- Updated Balance ---
Balance: 1.50000000 BTC
UTXOs: 1

--- Creating Transaction ---
Sending 0.8 BTC to bc1qxyz...
Fee rate: 10 sat/vB

UTXO Selection:
  Selected 1 UTXO(s)
  Total input: 1.50000000 BTC
  Payment: 0.80000000 BTC
  Change: 0.69990000 BTC
  Fee: 0.00010000 BTC (100 sat)

✅ Transaction created and signed
Transaction ID: a7b3f...

--- Final Balance ---
Balance: 0.69990000 BTC
UTXOs: 1 (change output)
```

## CLI Commands

### Basic Operations
```bash
# Create new wallet
wallet create --name mywallet

# Show wallet info
wallet info

# Generate new receiving address
wallet receive

# Check balance
wallet balance

# Send transaction
wallet send --to <address> --amount <btc> --fee <sat/vB>

# Show transaction history
wallet history

# List all UTXOs
wallet list-utxos

# Export wallet (encrypted)
wallet export --file backup.json

# Import wallet
wallet import --file backup.json
```

## Wallet File Format

### Structure (JSON)
```json
{
  "version": 1,
  "name": "mywallet",
  "master_key": "encrypted_key_data",
  "addresses": [
    {
      "address": "bc1q...",
      "index": 0,
      "used": true
    }
  ],
  "utxos": [
    {
      "txid": "abc123...",
      "vout": 0,
      "amount": 150000000,
      "address": "bc1q...",
      "confirmations": 6
    }
  ],
  "transactions": [
    {
      "txid": "def456...",
      "type": "received",
      "amount": 150000000,
      "timestamp": 1699564800
    }
  ]
}
```

## Privacy Features

### Coin Control
- Manually select which UTXOs to spend
- Avoid linking different income sources
- Advanced feature for privacy-conscious users

### Change Address Management
- Always use new change address
- Prevents address reuse
- Improves transaction privacy

### Gap Limit
- Stop scanning for addresses after N unused ones
- Prevents infinite scanning
- Standard: 20 addresses (BIP44)

## Advanced Features

### Multi-Signature (Future)
- Require M-of-N signatures to spend
- Enhanced security for large amounts
- Common: 2-of-3 (you + backup + third party)

### Time Locks
- Lock funds until specific time/block height
- Useful for vesting, escrow
- nLockTime field in transactions

### Replace-by-Fee (RBF)
- Replace unconfirmed transaction with higher fee
- Useful if initial fee was too low
- Must be enabled when creating transaction

## Error Handling

### Common Errors
```rust
WalletError::InsufficientFunds
WalletError::InvalidAddress
WalletError::FeeTooHigh
WalletError::UTXONotFound
WalletError::KeyDerivationFailed
WalletError::SigningFailed
```

### Safety Checks
- Verify address format before sending
- Check sufficient balance before transaction
- Confirm fee is reasonable (< 5% of amount)
- Prevent sending to self unnecessarily
- Validate transaction size

## Testing Strategy

### Unit Tests
- UTXO selection algorithms
- Transaction construction
- Fee calculation
- Address generation

### Integration Tests
- End-to-end wallet operations
- Persistence and recovery
- Multi-transaction scenarios

### Property-Based Tests
- Input/output balance equality (minus fee)
- UTXO set consistency
- Address uniqueness

## Challenge Extensions
1. Implement BIP39 mnemonic seed phrases
2. Add BIP32 hierarchical deterministic key derivation
3. Support multiple accounts (BIP44 path structure)
4. Implement branch-and-bound UTXO selection
5. Add transaction fee estimation API
6. Support Partially Signed Bitcoin Transactions (PSBTs)
7. Implement coin control feature
8. Add multi-signature wallet support
9. Create QR code generation for addresses
10. Build a simple GUI frontend

## Resources
- [BIP32: Hierarchical Deterministic Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP39: Mnemonic Code](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP44: Multi-Account Hierarchy](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)
- [Bitcoin Transaction Structure](https://en.bitcoin.it/wiki/Transaction)
- [Mastering Bitcoin Book](https://github.com/bitcoinbook/bitcoinbook)
