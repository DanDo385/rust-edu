# Project 40: File Encryption

## Overview
Build a file encryption and decryption tool using cryptographic algorithms. Learn about symmetric encryption, binary I/O, key derivation, and security best practices in Rust.

## Concepts Taught
- **Symmetric encryption** (AES)
- **Binary file I/O**
- **Byte manipulation**
- **Cryptographic libraries** (using simple XOR cipher for education)
- **Key derivation** and password hashing
- **Security considerations**
- **Error handling** for crypto operations
- **Buffer management**

## Why File Encryption?

File encryption protects sensitive data:
- **Personal data**: Protect documents, photos, backups
- **Business data**: Trade secrets, customer information
- **Communication**: Encrypted messaging, email
- **Storage**: Cloud storage, USB drives, hard drives

### Real-World Applications

- **FileVault** (macOS): Full disk encryption
- **BitLocker** (Windows): Volume encryption
- **VeraCrypt**: Cross-platform disk encryption
- **GPG**: Email and file encryption
- **7-Zip**: Archive encryption

## Encryption Concepts

### Symmetric Encryption
Same key for encryption and decryption:
- **AES**: Advanced Encryption Standard (industry standard)
- **ChaCha20**: Modern stream cipher
- **3DES**: Legacy (deprecated)

### Key Properties
- **Confidentiality**: Only authorized parties can read
- **Integrity**: Detect if data has been modified
- **Authentication**: Verify sender identity

### Security Considerations

1. **Never roll your own crypto**: Use well-tested libraries
2. **Key management**: Protect keys, use key derivation functions
3. **Initialization vectors (IV)**: Use unique IVs for each encryption
4. **Authenticated encryption**: Use AEAD (AES-GCM, ChaCha20-Poly1305)
5. **Secure random**: Use cryptographically secure RNG

## Running This Project

```bash
cd 40-file-encryption
cargo run

# For production encryption, add to Cargo.toml:
# [dependencies]
# aes-gcm = "0.10"
# argon2 = "0.5"
# rand = "0.8"
```

## Educational vs Production

This project uses **XOR cipher** for educational purposes:
- Easy to understand
- Demonstrates core concepts
- **NOT secure** for real data

For production, use:
- **AES-256-GCM**: Authenticated encryption
- **Argon2**: Password-based key derivation
- **ChaCha20-Poly1305**: Alternative to AES
- **libsodium**: High-level crypto library

## Security Best Practices

### DO
- Use established crypto libraries (ring, RustCrypto)
- Use authenticated encryption (AES-GCM, not AES-CBC)
- Derive keys with Argon2 or PBKDF2
- Use cryptographically secure random numbers
- Implement proper error handling
- Clear sensitive data from memory (use zeroize crate)

### DON'T
- Implement your own crypto algorithms
- Reuse IVs or nonces
- Use weak key derivation (plain hash)
- Store keys in source code
- Ignore authentication (MAC)
- Use deprecated algorithms (MD5, SHA1, DES)

## Performance Considerations

**Encryption Speed** (AES-128):
- Software: 100-500 MB/s per core
- Hardware (AES-NI): 1-4 GB/s per core

**Key Derivation** (Argon2):
- Intentionally slow to prevent brute force
- Configurable memory and time parameters
- ~100ms per key derivation (recommended)

**File I/O**:
- Bottleneck is usually disk, not encryption
- Use buffered I/O for large files
- Encrypt in chunks (4KB - 64KB blocks)

## Comparison: Rust vs Other Languages

| Feature | Rust | Python (cryptography) | OpenSSL (C) |
|---------|------|----------------------|-------------|
| Safety | Memory safe | Memory safe (GC) | Manual (unsafe) |
| Performance | Excellent | Moderate | Excellent |
| Ease of use | Moderate | Easy | Difficult |
| Dependencies | Minimal | Large | Minimal |
| Cross-compilation | Excellent | Difficult | Moderate |

## Cryptographic Libraries in Rust

### RustCrypto
Pure Rust implementations:
- `aes`, `chacha20poly1305`, `sha2`, `argon2`
- Memory safe, portable
- Slower than hardware-accelerated

### ring
High-performance, minimal API:
- Uses optimized assembly (AES-NI)
- Fewer algorithms, carefully selected
- Used by many production systems

### libsodium (via sodiumoxide)
Easy-to-use, hard-to-misuse:
- High-level API
- Opinionated choices
- Best for beginners

## Additional Challenges

1. **AES-GCM Encryption**: Use authenticated encryption with real crypto library

2. **Password-Based Encryption**: Derive keys from passwords with Argon2

3. **Directory Encryption**: Encrypt all files in a directory recursively

4. **Encrypted Archive**: Create encrypted tar/zip-like archives

5. **Streaming Encryption**: Encrypt large files without loading into memory

6. **Key Management**: Store and retrieve keys securely (OS keychain)

7. **File Shredding**: Securely delete original files after encryption

8. **Metadata Protection**: Encrypt filenames and file metadata

9. **Public Key Encryption**: Use RSA or Ed25519 for asymmetric encryption

10. **Encrypted Database**: Build SQLite-like encrypted database

## Future Directions

- **Next**: Build more advanced cryptographic systems
- **Later**: Digital signatures (Project 46), Blockchain wallet (Project 47)
- **Advanced**: Zero-knowledge proofs, homomorphic encryption

## Expected Output

You should see:
- File encryption with visual progress
- Successful decryption back to original
- Demonstration of key management
- Error handling for wrong keys
- Binary I/O operations
- Clear explanation of security considerations
