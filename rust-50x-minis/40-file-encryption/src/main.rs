// Project 40: File Encryption
//
// This program implements file encryption and decryption.
// It uses XOR cipher for educational purposes (NOT secure for production).
// For real applications, use AES-GCM, ChaCha20-Poly1305, or libsodium.
//
// IMPORTANT: This implementation is for LEARNING ONLY!
// Never use XOR cipher for protecting real data.

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    println!("=== File Encryption ===\n");

    // ============================================================================
    // WHAT IS FILE ENCRYPTION?
    // ============================================================================
    // File encryption transforms readable data (plaintext) into unreadable
    // data (ciphertext) using a secret key. Only someone with the key can
    // decrypt it back to plaintext.
    //
    // Types of encryption:
    // - Symmetric: Same key for encrypt/decrypt (AES, ChaCha20)
    // - Asymmetric: Different keys for encrypt/decrypt (RSA, Ed25519)
    //
    // This example uses XOR cipher (symmetric) for simplicity.
    // XOR is NOT SECURE but demonstrates core concepts clearly.

    println!("⚠️  WARNING: This uses XOR cipher for educational purposes only!");
    println!("   For real encryption, use AES-256-GCM or ChaCha20-Poly1305.\n");

    // ============================================================================
    // DEMONSTRATING ENCRYPTION/DECRYPTION
    // ============================================================================

    let original_text = "This is a secret message that needs to be protected!";
    let key = "super-secret-key-12345";

    println!("=== String Encryption Example ===\n");
    println!("Original:  \"{}\"", original_text);
    println!("Key:       \"{}\"", key);
    println!();

    // Encrypt
    let encrypted = xor_encrypt(original_text.as_bytes(), key.as_bytes());
    println!("Encrypted: {:?}", encrypted);
    println!("           (binary data, not readable)");
    println!();

    // Decrypt
    let decrypted = xor_decrypt(&encrypted, key.as_bytes());
    let decrypted_text = String::from_utf8_lossy(&decrypted);
    println!("Decrypted: \"{}\"", decrypted_text);
    println!();

    // Wrong key demonstration
    let wrong_key = "wrong-key-000000000000";
    let wrong_decrypt = xor_decrypt(&encrypted, wrong_key.as_bytes());
    let wrong_text = String::from_utf8_lossy(&wrong_decrypt);
    println!("With wrong key: \"{}\"", wrong_text);
    println!("                (gibberish - key is essential!)");

    println!();

    // ============================================================================
    // FILE ENCRYPTION EXAMPLE
    // ============================================================================

    println!("=== File Encryption Example ===\n");

    let test_file = "secret.txt";
    let encrypted_file = "secret.txt.enc";
    let decrypted_file = "secret_decrypted.txt";

    // Create a test file
    println!("Creating test file: {}", test_file);
    let test_content = "This is confidential data.\nIt must be protected.\n\
                       Account Number: 1234-5678-9012\n\
                       Password: super_secret_123";

    if let Err(e) = fs::write(test_file, test_content) {
        eprintln!("Error creating test file: {}", e);
        return;
    }
    println!("✓ Created test file with sensitive data\n");

    // Encrypt the file
    println!("Encrypting {} -> {}", test_file, encrypted_file);
    match encrypt_file(test_file, encrypted_file, key.as_bytes()) {
        Ok(size) => println!("✓ Encrypted {} bytes\n", size),
        Err(e) => {
            eprintln!("Error encrypting file: {}", e);
            return;
        }
    }

    // Show encrypted file is binary gibberish
    if let Ok(encrypted_content) = fs::read(encrypted_file) {
        println!("Encrypted file content (first 50 bytes):");
        println!("{:?}", &encrypted_content[..50.min(encrypted_content.len())]);
        println!("(unreadable binary data)\n");
    }

    // Decrypt the file
    println!("Decrypting {} -> {}", encrypted_file, decrypted_file);
    match decrypt_file(encrypted_file, decrypted_file, key.as_bytes()) {
        Ok(size) => println!("✓ Decrypted {} bytes\n", size),
        Err(e) => {
            eprintln!("Error decrypting file: {}", e);
            return;
        }
    }

    // Verify decryption
    match fs::read_to_string(decrypted_file) {
        Ok(content) => {
            println!("Decrypted file content:");
            println!("---");
            println!("{}", content);
            println!("---");
            println!();

            if content == test_content {
                println!("✓ Decryption successful - content matches original!");
            } else {
                println!("✗ Decryption failed - content doesn't match!");
            }
        }
        Err(e) => eprintln!("Error reading decrypted file: {}", e),
    }

    // Cleanup
    println!();
    println!("Cleaning up test files...");
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    println!("✓ Cleanup complete\n");

    // ============================================================================
    // SECURITY NOTES
    // ============================================================================

    println!("=== Security Considerations ===\n");
    println!("🔒 For production encryption, use:");
    println!("   1. AES-256-GCM (authenticated encryption)");
    println!("   2. ChaCha20-Poly1305 (modern alternative)");
    println!("   3. Argon2 for password-based key derivation");
    println!("   4. Cryptographically secure random IVs");
    println!();
    println!("⚠️  Never use XOR cipher for real data!");
    println!("   - Vulnerable to known-plaintext attacks");
    println!("   - No authentication (can be modified)");
    println!("   - Key reuse reveals patterns");
    println!();
    println!("📚 Recommended Rust crates:");
    println!("   - ring: High-performance, minimal API");
    println!("   - RustCrypto (aes-gcm): Pure Rust");
    println!("   - sodiumoxide: Easy to use correctly");
    println!();
}

// ============================================================================
// XOR ENCRYPTION/DECRYPTION
// ============================================================================

/// Encrypt data using XOR cipher (EDUCATIONAL ONLY - NOT SECURE!)
fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    xor_cipher(data, key)
}

/// Decrypt data using XOR cipher
fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // XOR encryption is symmetric - same operation for encrypt/decrypt
    xor_cipher(data, key)
}

/// XOR cipher implementation
///
/// How it works:
/// 1. For each byte in data
/// 2. XOR it with corresponding byte in key (cycling if key is shorter)
/// 3. Result is encrypted/decrypted byte
///
/// Why it's insecure:
/// - If attacker knows any plaintext, they can recover the key
/// - Statistical analysis can reveal patterns
/// - No authentication - can be modified without detection
fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }

    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

// ============================================================================
// FILE ENCRYPTION
// ============================================================================

/// Encrypt a file and write to output
fn encrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<usize> {
    // Read entire file into memory
    // For large files, use buffered reading/writing
    let plaintext = fs::read(input_path)?;

    // Encrypt the data
    let ciphertext = xor_encrypt(&plaintext, key);

    // Write encrypted data to file
    fs::write(output_path, &ciphertext)?;

    Ok(ciphertext.len())
}

/// Decrypt a file and write to output
fn decrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<usize> {
    // Read encrypted file
    let ciphertext = fs::read(input_path)?;

    // Decrypt the data
    let plaintext = xor_decrypt(&ciphertext, key);

    // Write decrypted data to file
    fs::write(output_path, &plaintext)?;

    Ok(plaintext.len())
}

/// Encrypt large file with buffered I/O (more efficient)
#[allow(dead_code)]
fn encrypt_file_buffered(
    input_path: &str,
    output_path: &str,
    key: &[u8],
) -> io::Result<usize> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    let mut buffer = vec![0u8; 4096]; // 4KB buffer
    let mut total_bytes = 0;

    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        // Encrypt the buffer
        let encrypted = xor_encrypt(&buffer[..bytes_read], key);

        // Write encrypted data
        output_file.write_all(&encrypted)?;

        total_bytes += bytes_read;
    }

    Ok(total_bytes)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. FILE I/O
//    fs::read() reads entire file into Vec<u8> (heap allocation).
//    fs::write() writes byte slice to file atomically (on most systems).
//    For large files, use File::open + BufReader for streaming.
//
// 2. BYTE MANIPULATION
//    XOR (^) is a bitwise operation:
//    - 0 ^ 0 = 0
//    - 0 ^ 1 = 1
//    - 1 ^ 0 = 1
//    - 1 ^ 1 = 0
//    XOR is reversible: (a ^ b) ^ b = a
//
// 3. ITERATOR PERFORMANCE
//    .enumerate().map() creates an iterator chain.
//    .collect() allocates a Vec and fills it.
//    The compiler optimizes this to be nearly as fast as a for loop.
//
// 4. MEMORY SAFETY
//    Rust ensures:
//    - No buffer overflows (bounds checking)
//    - No use-after-free (ownership system)
//    - No data races (borrow checker)
//    This makes crypto code safer (fewer vulnerabilities).
//
// 5. PERFORMANCE
//    XOR cipher: ~1-4 GB/s (CPU-bound, no crypto instructions needed)
//    AES-NI (hardware): ~4-10 GB/s per core
//    File I/O: ~100-500 MB/s (SSD) or ~50-150 MB/s (HDD)
//    Bottleneck is usually disk, not encryption.
//
// 6. MEMORY LAYOUT
//    - Vec<u8>: 24 bytes on stack (ptr, len, cap) + data on heap
//    - File: File descriptor (integer), buffering metadata
//    - Total memory: O(file size) for full-file approach
//                   O(buffer size) for buffered approach

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. XOR cipher is EDUCATIONAL ONLY - not secure for real use
// 2. File encryption requires binary I/O (Vec<u8>, not String)
// 3. Symmetric encryption uses same key for encrypt/decrypt
// 4. Real crypto should use: AES-GCM, ChaCha20-Poly1305
// 5. Always use authenticated encryption (detects tampering)
// 6. Key management is critical (key derivation, storage)
// 7. Rust's memory safety prevents crypto vulnerabilities
// 8. Use established libraries, never roll your own crypto

// ============================================================================
// REAL-WORLD ENCRYPTION (AES-GCM EXAMPLE)
// ============================================================================
// With the aes-gcm crate, production code would look like:
//
// use aes_gcm::{Aes256Gcm, KeyInit};
// use aes_gcm::aead::{Aead, OsRng};
//
// // Generate random key
// let key = Aes256Gcm::generate_key(&mut OsRng);
// let cipher = Aes256Gcm::new(&key);
//
// // Generate random nonce (must be unique per encryption)
// let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
//
// // Encrypt
// let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
//     .expect("encryption failure!");
//
// // Decrypt
// let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())
//     .expect("decryption failure!");
//
// Key points:
// - Uses 256-bit AES in GCM mode (authenticated)
// - Random nonce (never reuse with same key!)
// - Detects tampering (authentication tag)
// - Hardware-accelerated (AES-NI)

// ============================================================================
// WHY THIS MATTERS
// ============================================================================
// File encryption is essential for:
// - **Privacy**: Protect personal data, communications
// - **Compliance**: GDPR, HIPAA, PCI-DSS require encryption
// - **Security**: Prevent data theft, ransomware protection
// - **Blockchain**: Wallet encryption, private key storage
//
// Understanding encryption helps you:
// - Build secure applications
// - Protect user data
// - Implement secure storage
// - Create encrypted messaging systems

// ============================================================================
// SECURITY BEST PRACTICES
// ============================================================================
// DO:
// ✓ Use AES-256-GCM or ChaCha20-Poly1305
// ✓ Use Argon2 for password-based key derivation
// ✓ Generate random IVs/nonces for each encryption
// ✓ Use authenticated encryption (AEAD)
// ✓ Clear sensitive data from memory (zeroize crate)
// ✓ Use constant-time comparisons (subtle crate)
// ✓ Get security audits for production systems
//
// DON'T:
// ✗ Implement your own crypto algorithms
// ✗ Reuse IVs/nonces
// ✗ Use weak key derivation (plain SHA-256)
// ✗ Use unauthenticated encryption (AES-CBC without MAC)
// ✗ Use deprecated algorithms (DES, 3DES, MD5, SHA1)
// ✗ Store keys in source code or version control
// ✗ Trust user input as cryptographic keys

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Use AES-GCM or ChaCha20-Poly1305 (authenticated encryption)
// 2. Implement proper key derivation (Argon2, PBKDF2)
// 3. Add salt and IV to encrypted file format
// 4. Use streaming encryption for large files
// 5. Implement file format versioning
// 6. Add integrity checks (HMAC or AEAD)
// 7. Secure key storage (OS keychain, HSM)
// 8. Implement key rotation
// 9. Add metadata encryption (filenames, sizes)
// 10. Comprehensive error handling and logging

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Using XOR or simple ciphers for real data
// ❌ Reusing IVs/nonces (catastrophic failure for AES-GCM)
// ❌ Not authenticating encrypted data (vulnerable to tampering)
// ❌ Using weak passwords directly as keys
// ❌ Storing keys alongside encrypted data
// ❌ Not handling errors properly (silent failures)
// ❌ Encrypting in ECB mode (reveals patterns)
// ❌ Not using constant-time comparisons (timing attacks)
