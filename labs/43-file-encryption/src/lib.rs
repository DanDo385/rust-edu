//! # File Encryption - Student API
//!
//! Implement XOR-based helper functions and file I/O operations.
//! Keep this module minimal; students will fill the logic below and
//! compare to the heavily commented reference in `src/solution.rs`.

use std::io;

/// XOR encrypt bytes using the provided key.
pub fn xor_encrypt(_data: &[u8], _key: &[u8]) -> Vec<u8> {
    todo!("Implement XOR encryption")
}

/// XOR decrypt bytes using the same key.
pub fn xor_decrypt(_data: &[u8], _key: &[u8]) -> Vec<u8> {
    todo!("Implement XOR decryption")
}

/// Core XOR cipher used by both encryption and decryption.
pub fn xor_cipher(_data: &[u8], _key: &[u8]) -> Vec<u8> {
    todo!("Implement the XOR cipher core")
}

/// Encrypt an entire file and write ciphertext to `output_path`.
pub fn encrypt_file(_input_path: &str, _output_path: &str, _key: &[u8]) -> io::Result<usize> {
    todo!("Read input, XOR encrypt, write output file")
}

/// Decrypt an encrypted file and write plaintext to `output_path`.
pub fn decrypt_file(_input_path: &str, _output_path: &str, _key: &[u8]) -> io::Result<usize> {
    todo!("Read encrypted file, XOR decrypt, write plaintext")
}

/// Encrypt large files buffer-by-buffer for lower memory usage.
pub fn encrypt_file_buffered(
    _input_path: &str,
    _output_path: &str,
    _key: &[u8],
) -> io::Result<usize> {
    todo!("Stream file in chunks and encrypt each chunk")
}

#[doc(hidden)]
pub mod solution;
