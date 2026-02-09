// Project 40: File Encryption
//
// This program implements file encryption and decryption.
// It uses XOR cipher for educational purposes (NOT secure for production).
// For real applications, use AES-GCM, ChaCha20-Poly1305, or libsodium.
//
// IMPORTANT: This implementation is for LEARNING ONLY!
// Never use XOR cipher for protecting real data.

use std::fs;
use std::path::Path;

use file_encryption::solution::{decrypt_file, encrypt_file, xor_decrypt, xor_encrypt};
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

// Further commentary and diagrams live in solution.rs and README.md
// End of CLI encryption demo
