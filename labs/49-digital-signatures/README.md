# Project 46: Digital Signatures (CAPSTONE)

## Overview
Implement cryptographic digital signatures using both Ed25519 and secp256k1 algorithms. Understand public-key cryptography, signing/verification, and their critical role in blockchain systems.

## Concepts Taught
- **Public-key cryptography**: asymmetric encryption fundamentals
- **Ed25519**: modern elliptic curve signature scheme (Curve25519)
- **secp256k1**: Bitcoin/Ethereum signature scheme
- **Digital signatures**: proving message authenticity and integrity
- **Key generation**: creating secure keypairs
- **Message signing**: creating unforgeable signatures
- **Signature verification**: validating message authenticity
- **Blockchain context**: how signatures enable trustless transactions

## Why Digital Signatures Matter

### The Problem They Solve
Without digital signatures:
- Anyone could forge transactions
- No way to prove message authenticity
- Centralized authorities needed to verify identity
- Transactions require trust

### How They Work
1. **Key Generation**: Create a random private key (secret) and derive a public key
2. **Signing**: Use private key to sign a message (creates a signature)
3. **Verification**: Anyone can verify the signature using the public key
4. **Security**: Only the private key holder can create valid signatures

### Real-World Use Cases
- **Bitcoin/Ethereum**: Every transaction is signed with private key
- **SSL/TLS**: Website certificates use digital signatures
- **Code Signing**: Software publishers sign executables
- **Document Signing**: PDF signatures, electronic contracts
- **SSH Keys**: Secure remote authentication

## Ed25519 vs secp256k1

### Ed25519 (Modern Choice)
- **Speed**: Very fast signing and verification
- **Security**: 128-bit security level (256-bit key)
- **Simplicity**: Deterministic, no RNG needed for signing
- **Used by**: SSH, Signal, Monero, Polkadot
- **Advantages**: Immune to timing attacks, smaller signatures

### secp256k1 (Bitcoin Standard)
- **Security**: 128-bit security level (256-bit key)
- **Adoption**: Bitcoin, Ethereum, and most cryptocurrencies
- **Recovery**: Supports public key recovery from signature
- **Compatibility**: ECDSA standard, widely supported
- **Trade-offs**: Slower, more complex, requires good RNG

## Cryptographic Concepts

### Elliptic Curve Cryptography (ECC)
Both algorithms use elliptic curves - algebraic curves over finite fields:
- Much smaller keys than RSA for same security
- Faster computation
- Based on discrete logarithm problem (hard to reverse)

### Message Hashing
Before signing, messages are hashed:
- Ensures fixed-size input to signature algorithm
- Provides collision resistance
- Common: SHA-256, SHA-512, Blake2

### Signature Components
- **r value**: Random point on the curve
- **s value**: Computed using private key, message hash, and r
- Together (r,s) form the signature
- Verification uses public key to check if signature matches message

## Security Properties

### Unforgeability
- Computationally infeasible to create valid signature without private key
- Even with millions of message-signature pairs

### Non-repudiation
- Signer cannot deny creating a signature
- Proof of authenticity

### Integrity
- Any modification to signed message invalidates signature
- Tamper-evident

### Authentication
- Proves message came from holder of private key
- Identity verification without revealing the key

## Running This Project

```bash
cd 46-digital-signatures
cargo run
```

**Dependencies** (add to `Cargo.toml`):
```toml
[dependencies]
ed25519-dalek = "2.1"
k256 = { version = "0.13", features = ["ecdsa"] }
sha2 = "0.10"
rand = "0.8"
hex = "0.4"
```

## Expected Output
```
=== Digital Signatures Demonstration ===

--- Ed25519 Signatures ---
Generated Ed25519 keypair
Public key: a1b2c3d4...
Signing message: "Hello, Blockchain!"
Signature: 4f5e6d7c...
✅ Signature verified successfully!
Verifying tampered message...
❌ Signature verification failed (as expected)

--- secp256k1 Signatures (Bitcoin-style) ---
Generated secp256k1 keypair
Public key: 03f8a9b7...
Signing transaction: "Alice sends 10 BTC to Bob"
Signature: 3045022100...
✅ Signature verified successfully!
Testing signature recovery...
✅ Public key recovered from signature!

--- Performance Comparison ---
Ed25519 signing: 1000 signatures in 15ms
Ed25519 verification: 1000 verifications in 45ms
secp256k1 signing: 1000 signatures in 120ms
secp256k1 verification: 1000 verifications in 180ms
```

## Blockchain Context

### Transaction Signing
In Bitcoin/Ethereum:
1. User creates transaction (send X coins to address Y)
2. Transaction is hashed
3. User signs hash with private key
4. Transaction + signature is broadcast to network
5. Nodes verify signature using sender's public key
6. Valid signatures prove ownership of coins

### Address Derivation
- Public key is hashed to create address
- Address is like account number (shareable)
- Private key proves ownership (never share!)

### Multi-signature Wallets
- Require N-of-M signatures
- Enhanced security for large amounts
- Used by exchanges, DAOs, escrow

## Common Pitfalls

### Security Mistakes
❌ **Reusing nonce/randomness** - catastrophic for ECDSA!
❌ **Weak random number generator** - predictable keys
❌ **Signing without hashing** - vulnerable to attacks
❌ **Exposing private keys** - total loss of funds
❌ **Not verifying signatures** - accepting invalid transactions

### Implementation Mistakes
❌ **Comparing signatures with ==** - timing attacks
❌ **Not handling errors** - crashes on invalid input
❌ **Forgetting to hash message** - signing raw data
❌ **Wrong curve parameters** - incompatible signatures

## Advanced Topics

### Deterministic Signatures (RFC 6979)
- Ed25519 is always deterministic
- secp256k1 can use deterministic k-value
- Same message + key = same signature
- Eliminates RNG dependency

### Schnorr Signatures
- Alternative to ECDSA on secp256k1
- Used in Bitcoin Taproot upgrade
- Supports signature aggregation
- More efficient for multi-sig

### Threshold Signatures
- Split private key among N parties
- Require M-of-N to sign
- No single point of failure
- Used in institutional custody

## Next Steps
- Integrate signatures into blockchain project
- Build wallet with key management
- Implement multi-signature schemes
- Add hardware wallet support
- Explore zero-knowledge proofs

## Challenge Extensions
1. Implement batch signature verification
2. Add BIP32/BIP44 hierarchical deterministic keys
3. Create a simple PKI (Public Key Infrastructure)
4. Build a message signing/verification CLI tool
5. Implement Schnorr signatures on secp256k1
6. Add support for signature aggregation
7. Create a multi-signature wallet
8. Implement threshold signatures (2-of-3, 3-of-5)

## Resources
- [Ed25519 Paper](https://ed25519.cr.yp.to/)
- [secp256k1 Specifications](https://www.secg.org/sec2-v2.pdf)
- [Bitcoin's Use of ECDSA](https://en.bitcoin.it/wiki/Elliptic_Curve_Digital_Signature_Algorithm)
- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [Understanding Cryptography](https://www.crypto-textbook.com/)
