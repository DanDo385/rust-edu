# Project 49: Blockchain Node (CAPSTONE)

## Overview
Build a complete blockchain node that integrates all previous concepts: UTXO model, transaction mempool, proof-of-work consensus, peer-to-peer networking, and block validation. This is the culmination of the blockchain learning track.

## Concepts Taught
- **Full node architecture**: integrating all blockchain components
- **UTXO set management**: tracking unspent outputs efficiently
- **Mempool operations**: managing pending transactions
- **Consensus rules**: validating blocks and transactions
- **Peer-to-peer networking**: gossip protocol, block propagation
- **Chain synchronization**: catching up with the network
- **Fork resolution**: handling competing chains
- **State persistence**: saving blockchain to disk
- **API interface**: RPC for external interaction

## Why Full Nodes Matter

### The Backbone of Blockchain
Full nodes are critical for:
- **Decentralization**: No central authority needed
- **Security**: Independent validation prevents fraud
- **Trustlessness**: Don't trust, verify yourself
- **Censorship resistance**: Can't be shut down
- **Network health**: More nodes = more resilient network

### Node vs. Light Client

#### Full Node
- Downloads entire blockchain
- Validates all transactions and blocks
- Maintains complete UTXO set
- Can mine new blocks
- ~500GB storage (Bitcoin)
- High trust, high resource usage

#### Light Client (SPV)
- Downloads only block headers
- Trusts full nodes for validation
- Queries for relevant transactions
- Cannot mine
- ~1MB storage
- Lower trust, minimal resources

## Node Architecture

```
┌─────────────────────────────────────────┐
│         Blockchain Node                 │
├─────────────────────────────────────────┤
│                                         │
│  ┌──────────────┐    ┌──────────────┐  │
│  │   Mempool    │    │  UTXO Set    │  │
│  │  (Pending)   │    │ (Confirmed)  │  │
│  └──────┬───────┘    └──────┬───────┘  │
│         │                   │          │
│  ┌──────▼───────────────────▼───────┐  │
│  │      Consensus Engine            │  │
│  │  (Validation, PoW, Fork Choice)  │  │
│  └──────┬───────────────────────────┘  │
│         │                              │
│  ┌──────▼───────────┐                  │
│  │   Blockchain     │                  │
│  │  (Block Chain)   │                  │
│  └──────┬───────────┘                  │
│         │                              │
│  ┌──────▼───────────┐                  │
│  │  P2P Network     │                  │
│  │  (Gossip, Sync)  │                  │
│  └──────────────────┘                  │
│                                         │
│  ┌──────────────────┐                  │
│  │    RPC API       │                  │
│  │ (External Access)│                  │
│  └──────────────────┘                  │
└─────────────────────────────────────────┘
```

## Core Components

### 1. Blockchain
- Chain of blocks (genesis to tip)
- Each block contains transactions
- Linked by cryptographic hashes
- Provides immutability guarantee

### 2. UTXO Set
- All unspent transaction outputs
- Used to validate new transactions
- Updated when blocks are added
- Efficiently queryable (HashMap/BTreeMap)

### 3. Mempool
- Pending transactions waiting for confirmation
- Prioritized by fee (high fee = mined first)
- Evicted if invalid or double-spend
- Limited size (prevent DoS)

### 4. Consensus Engine
- Validates transactions (signatures, balances)
- Validates blocks (PoW, merkle root, size)
- Resolves forks (longest chain rule)
- Maintains consensus rules

### 5. P2P Network
- Discovers peers via DNS seeds
- Gossips transactions and blocks
- Syncs blockchain from peers
- Maintains peer connections

### 6. RPC API
- External interface for wallets
- Query balance, submit transactions
- Get block information
- Mine new blocks (in test mode)

## Transaction Flow

### 1. Submission
```
Wallet → RPC → Mempool
```
- User creates signed transaction
- Submits via RPC to node
- Node validates signature and funds
- Adds to mempool if valid

### 2. Propagation
```
Mempool → P2P Network → Peer Mempools
```
- Node broadcasts transaction to peers
- Peers validate and add to their mempools
- Transaction spreads across network
- Eventually reaches miners

### 3. Mining
```
Mempool → Block → Blockchain
```
- Miner selects transactions (highest fees first)
- Creates block with transactions
- Performs proof-of-work
- Broadcasts block to network

### 4. Confirmation
```
Blockchain → UTXO Set Update
```
- Nodes validate new block
- Add block to chain if valid
- Update UTXO set (consume inputs, create outputs)
- Remove transactions from mempool
- Wait for more confirmations (6+ recommended)

## Consensus Rules

### Block Validation
✅ **Must be valid:**
- Proof-of-work meets difficulty target
- Merkle root matches transactions
- Block size under limit
- Timestamp reasonable
- References valid previous block
- All transactions valid

❌ **Invalid block rejected:**
- Wrong PoW (too easy)
- Invalid merkle root
- Double-spend transaction
- Invalid signatures
- Coinbase amount too high

### Transaction Validation
✅ **Must be valid:**
- All input UTXOs exist
- Signatures are correct
- Sum(outputs) ≤ Sum(inputs)
- No double-spending
- Fees are reasonable

### Fork Resolution
When multiple valid blocks:
1. **Longest chain rule**: Most proof-of-work wins
2. **Orphan blocks**: Losing chain discarded
3. **Reorganization**: Switch to longer chain if needed
4. **Confirmation depth**: Wait for 6 blocks (Bitcoin standard)

## P2P Networking

### Peer Discovery
1. **DNS Seeds**: Query hardcoded DNS servers for peer IPs
2. **Peer Exchange**: Ask peers for more peer addresses
3. **Manual**: Specify peer IPs in configuration

### Message Types
- **version**: Handshake with peer capabilities
- **inv**: Inventory (I have these blocks/transactions)
- **getdata**: Request specific blocks/transactions
- **tx**: Transaction data
- **block**: Block data
- **ping/pong**: Keep-alive

### Gossip Protocol
```
Node A mines block →
  Send inv to peers (B, C, D) →
    B, C, D request getdata →
      A sends block →
        B, C, D validate →
          B, C, D gossip to their peers →
            ...network-wide propagation...
```

## Chain Synchronization

### Initial Block Download (IBD)
New node joining network:
1. **Download headers**: Get all block headers (fast)
2. **Validate headers**: Check PoW and difficulty
3. **Download blocks**: Get full blocks with transactions
4. **Validate blocks**: Full validation of all transactions
5. **Build UTXO set**: Process all transactions to current state

### Optimization: Checkpoints
- Hardcoded block hashes at certain heights
- Skip full validation before checkpoint
- Faster sync for new nodes
- Bitcoin uses checkpoints

## State Persistence

### What to Store
- **Blocks**: Full block data (headers + transactions)
- **UTXO set**: All unspent outputs (for validation)
- **Block index**: Height → block hash mapping
- **Mempool**: Pending transactions (volatile)
- **Peers**: Known peer addresses

### Storage Options
- **LevelDB/RocksDB**: Key-value store (Bitcoin uses LevelDB)
- **SQLite**: Relational database (easier queries)
- **Flat files**: Simple but less efficient

## Running This Project

```bash
cd 49-blockchain-node
cargo run
```

**Dependencies** (add to `Cargo.toml`):
```toml
[dependencies]
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
bincode = "1.3"
```

## Expected Output
```
=== Blockchain Node ===

Initializing node...
✅ Genesis block created
✅ UTXO set initialized
✅ Mempool created
✅ P2P network starting on port 8333

--- Node Status ---
Chain height: 1
UTXO count: 1
Mempool size: 0
Connected peers: 0

--- Receiving Transaction ---
Transaction received: Alice → Bob (50 coins)
✅ Transaction validated
✅ Added to mempool
📡 Broadcasting to peers...

--- Mining Block ---
Mining block 2...
Mining... (nonce: 45821)
✅ Block mined! Hash: 0000a3f7...

--- Block Validation ---
✅ Proof-of-work valid
✅ Merkle root valid
✅ All transactions valid
✅ Block added to chain

--- Updated Status ---
Chain height: 2
UTXO count: 2
Mempool size: 0
Balance changes:
  Alice: 100 → 50 coins
  Bob: 0 → 50 coins

--- P2P Simulation ---
📡 Broadcasting block to network...
Peer [127.0.0.1:8334] received block
Peer [127.0.0.1:8335] received block
✅ Block propagated to 2 peers
```

## API Endpoints

### Query Operations
```bash
# Get blockchain info
curl http://localhost:8080/info

# Get balance
curl http://localhost:8080/balance/{address}

# Get block by height
curl http://localhost:8080/block/{height}

# Get transaction
curl http://localhost:8080/tx/{txid}

# Get mempool
curl http://localhost:8080/mempool
```

### Mutation Operations
```bash
# Submit transaction
curl -X POST http://localhost:8080/tx \
  -d '{"from":"Alice","to":"Bob","amount":50,"signature":"..."}'

# Mine block (test mode)
curl -X POST http://localhost:8080/mine
```

## Security Considerations

### Attack Vectors

#### 1. Double-Spend Attack
- Attacker tries to spend same coins twice
- **Defense**: UTXO set tracks spent outputs
- **Mitigation**: Wait for confirmations (6+)

#### 2. 51% Attack
- Attacker controls majority of mining power
- Can reverse recent transactions
- **Defense**: Distributed mining, large network

#### 3. Eclipse Attack
- Attacker surrounds node with malicious peers
- Feeds fake blockchain data
- **Defense**: Diverse peer connections, checkpoints

#### 4. Sybil Attack
- Attacker creates many fake identities
- Tries to control network topology
- **Defense**: Proof-of-work, IP diversity

#### 5. Selfish Mining
- Miner withholds blocks to gain advantage
- Publishes at strategic times
- **Defense**: Network protocol improvements

## Performance Optimization

### UTXO Set
- Use in-memory database (RocksDB)
- Index by output ID (txid:vout)
- Periodic snapshots to disk
- Bitcoin's UTXO set: ~100M entries, ~5GB

### Block Validation
- Parallel signature verification
- Cache merkle tree computations
- Skip validation for old blocks (checkpoints)

### Mempool Management
- Limit size (prevent DoS)
- Evict low-fee transactions first
- Track transaction ancestors (CPFP)

### Network Optimization
- Connection pooling
- Bloom filters for SPV clients
- Compact block relay (only send transaction IDs)

## Challenge Extensions
1. Implement UTXO commitments (authenticated data structures)
2. Add Segregated Witness (SegWit) support
3. Implement Lightning Network payment channels
4. Add Schnorr signatures and Taproot
5. Implement GHOST protocol (Ethereum's fork choice)
6. Add difficulty adjustment algorithm
7. Implement peer reputation system
8. Add bloom filter support for SPV clients
9. Implement fee estimation algorithm
10. Build a block explorer web interface

## Real Blockchain Comparison

### Bitcoin Core
- C++, ~500k lines of code
- ~500GB blockchain (2024)
- ~100M UTXO entries
- ~1MB blocks (4MB with SegWit)
- 10 minute block time
- SHA-256 proof-of-work

### Ethereum (Geth)
- Go, ~300k lines of code
- ~900GB full node (2024)
- Account model (not UTXO)
- ~15 second block time
- Ethash → PoS (after merge)
- Smart contract support

### This Educational Node
- Rust, ~2k lines of code
- Simplified UTXO model
- No smart contracts
- Configurable block time
- For learning only!

## Resources
- [Bitcoin Developer Guide](https://developer.bitcoin.org/devguide/)
- [Bitcoin Core Source Code](https://github.com/bitcoin/bitcoin)
- [Mastering Bitcoin Book](https://github.com/bitcoinbook/bitcoinbook)
- [Bitcoin Protocol Documentation](https://en.bitcoin.it/wiki/Protocol_documentation)
- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [Satoshi Nakamoto's Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)
