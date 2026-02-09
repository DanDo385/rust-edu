// Lab 43: File Encryption
//
// This module implements file encryption and decryption using XOR cipher.
// XOR cipher is used for EDUCATIONAL PURPOSES ONLY - it is NOT secure for production.
// For real applications, use AES-GCM, ChaCha20-Poly1305, or libsodium.
//
// IMPORTANT: This implementation is for LEARNING ONLY!
// Never use XOR cipher for protecting real data.

use std::fs;
use std::io::{self, Read, Write};

// ============================================================================
// XOR ENCRYPTION/DECRYPTION
// ============================================================================

/// Encrypt data using XOR cipher (EDUCATIONAL ONLY - NOT SECURE!)
///
/// # How XOR encryption works
/// Each byte of the plaintext is XOR'd with the corresponding byte of the key.
/// The key cycles if it is shorter than the data.
///
/// # Why XOR alone is insecure
/// - Vulnerable to known-plaintext attacks
/// - No authentication (ciphertext can be modified undetected)
/// - Key reuse reveals patterns
pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    xor_cipher(data, key)
}

/// Decrypt data using XOR cipher.
///
/// XOR encryption is symmetric -- the same operation is used for both
/// encryption and decryption: `(data ^ key) ^ key == data`.
pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    xor_cipher(data, key)
}

/// Core XOR cipher implementation.
///
/// For each byte in `data`, XOR it with the corresponding byte in `key`
/// (cycling if the key is shorter than the data).
///
/// Returns the original data unchanged if the key is empty.
pub fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8> {
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

/// Encrypt a file and write the ciphertext to an output path.
///
/// Reads the entire input file into memory, encrypts it with the given key,
/// and writes the result to `output_path`.
///
/// Returns the number of encrypted bytes written.
pub fn encrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<usize> {
    let plaintext = fs::read(input_path)?;
    let ciphertext = xor_encrypt(&plaintext, key);
    fs::write(output_path, &ciphertext)?;
    Ok(ciphertext.len())
}

/// Decrypt a file and write the plaintext to an output path.
///
/// Reads the entire encrypted file into memory, decrypts it with the given key,
/// and writes the result to `output_path`.
///
/// Returns the number of decrypted bytes written.
pub fn decrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<usize> {
    let ciphertext = fs::read(input_path)?;
    let plaintext = xor_decrypt(&ciphertext, key);
    fs::write(output_path, &plaintext)?;
    Ok(plaintext.len())
}

/// Encrypt a large file using buffered I/O (more memory-efficient).
///
/// Instead of reading the entire file into memory, this processes
/// the file in 4KB chunks. Suitable for files that may be larger
/// than available RAM.
///
/// Returns the total number of bytes processed.
pub fn encrypt_file_buffered(
    input_path: &str,
    output_path: &str,
    key: &[u8],
) -> io::Result<usize> {
    let mut input_file = fs::File::open(input_path)?;
    let mut output_file = fs::File::create(output_path)?;

    let mut buffer = vec![0u8; 4096]; // 4KB buffer
    let mut total_bytes = 0;

    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let encrypted = xor_encrypt(&buffer[..bytes_read], key);
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
//    - 0 ^ 0 = 0, 0 ^ 1 = 1, 1 ^ 0 = 1, 1 ^ 1 = 0
//    XOR is reversible: (a ^ b) ^ b = a
//
// 3. ITERATOR PERFORMANCE
//    .enumerate().map() creates an iterator chain.
//    .collect() allocates a Vec and fills it.
//    The compiler optimizes this to be nearly as fast as a for loop.
//
// 4. MEMORY SAFETY
//    Rust ensures no buffer overflows, no use-after-free, no data races.
//    This makes crypto code safer (fewer vulnerabilities).
