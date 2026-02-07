// Integration tests for Lab 43: File Encryption
//
// Tests XOR cipher encrypt/decrypt roundtrips, edge cases,
// different key sizes, and file-based encryption.

use file_encryption::*;
use std::fs;

// ============================================================================
// XOR CIPHER BASIC TESTS
// ============================================================================

#[test]
fn test_xor_encrypt_basic() {
    let data = b"Hello, World!";
    let key = b"secret";
    let encrypted = xor_encrypt(data, key);

    // Encrypted data should differ from original
    assert_ne!(encrypted, data);
    // Encrypted data should be same length as original
    assert_eq!(encrypted.len(), data.len());
}

#[test]
fn test_xor_decrypt_reverses_encrypt() {
    let data = b"Hello, World!";
    let key = b"secret";
    let encrypted = xor_encrypt(data, key);
    let decrypted = xor_decrypt(&encrypted, key);

    assert_eq!(decrypted, data);
}

#[test]
fn test_xor_roundtrip_long_message() {
    let data = b"This is a longer message that tests the cycling behavior of the XOR key across multiple repetitions of the key material.";
    let key = b"key";
    let encrypted = xor_encrypt(data, key);
    let decrypted = xor_decrypt(&encrypted, key);

    assert_eq!(decrypted, data.to_vec());
}

#[test]
fn test_xor_roundtrip_key_longer_than_data() {
    let data = b"Hi";
    let key = b"a-very-long-key-that-is-much-longer-than-the-data";
    let encrypted = xor_encrypt(data, key);
    let decrypted = xor_decrypt(&encrypted, key);

    assert_eq!(decrypted, data.to_vec());
}

#[test]
fn test_xor_roundtrip_key_same_length_as_data() {
    let data = b"ABCDE";
    let key = b"12345";
    let encrypted = xor_encrypt(data, key);
    let decrypted = xor_decrypt(&encrypted, key);

    assert_eq!(decrypted, data.to_vec());
}

// ============================================================================
// XOR CIPHER EDGE CASES
// ============================================================================

#[test]
fn test_xor_empty_data() {
    let data: &[u8] = b"";
    let key = b"secret";
    let encrypted = xor_encrypt(data, key);

    assert!(encrypted.is_empty());
}

#[test]
fn test_xor_empty_key_returns_original() {
    let data = b"Hello, World!";
    let key: &[u8] = b"";
    let result = xor_encrypt(data, key);

    // Empty key should return data unchanged
    assert_eq!(result, data.to_vec());
}

#[test]
fn test_xor_single_byte_data() {
    let data = &[0xAB_u8];
    let key = &[0xFF_u8];
    let encrypted = xor_encrypt(data, key);

    // 0xAB ^ 0xFF = 0x54
    assert_eq!(encrypted, vec![0xAB ^ 0xFF]);

    let decrypted = xor_decrypt(&encrypted, key);
    assert_eq!(decrypted, data.to_vec());
}

#[test]
fn test_xor_single_byte_key() {
    let data = b"AAAA";
    let key = &[0xFF_u8];
    let encrypted = xor_encrypt(data, key);

    // Each byte XOR'd with 0xFF
    for (i, &byte) in encrypted.iter().enumerate() {
        assert_eq!(byte, data[i] ^ 0xFF);
    }
}

#[test]
fn test_xor_cipher_is_symmetric() {
    // XOR cipher: encrypting twice with the same key returns the original
    let data = b"Symmetric test data";
    let key = b"mykey";
    let once = xor_cipher(data, key);
    let twice = xor_cipher(&once, key);

    assert_eq!(twice, data.to_vec());
}

#[test]
fn test_xor_all_zeros_key() {
    let data = b"Hello";
    let key = &[0u8; 5];
    let encrypted = xor_encrypt(data, key);

    // XOR with zero leaves data unchanged
    assert_eq!(encrypted, data.to_vec());
}

#[test]
fn test_xor_all_zeros_data() {
    let data = &[0u8; 10];
    let key = b"secret";
    let encrypted = xor_encrypt(data, key);

    // XOR of zero with key byte is the key byte
    for (i, &byte) in encrypted.iter().enumerate() {
        assert_eq!(byte, key[i % key.len()]);
    }
}

#[test]
fn test_xor_binary_data() {
    let data: Vec<u8> = (0..=255).collect();
    let key = b"binary-test-key";
    let encrypted = xor_encrypt(&data, key);
    let decrypted = xor_decrypt(&encrypted, key);

    assert_eq!(decrypted, data);
}

// ============================================================================
// WRONG KEY TESTS
// ============================================================================

#[test]
fn test_wrong_key_does_not_decrypt() {
    let data = b"Secret message!";
    let correct_key = b"correct-key";
    let wrong_key = b"wrong-key!!";

    let encrypted = xor_encrypt(data, correct_key);
    let bad_decrypt = xor_decrypt(&encrypted, wrong_key);

    assert_ne!(bad_decrypt, data.to_vec());
}

#[test]
fn test_different_keys_produce_different_ciphertext() {
    let data = b"Same plaintext";
    let key1 = b"key-one";
    let key2 = b"key-two";

    let encrypted1 = xor_encrypt(data, key1);
    let encrypted2 = xor_encrypt(data, key2);

    assert_ne!(encrypted1, encrypted2);
}

// ============================================================================
// XOR MATHEMATICAL PROPERTIES
// ============================================================================

#[test]
fn test_xor_self_produces_zeros() {
    // data ^ data == all zeros
    let data = b"Test data";
    let result = xor_cipher(data, data);

    assert!(result.iter().all(|&b| b == 0));
}

#[test]
fn test_xor_key_cycling() {
    let data = b"ABCDEF";
    let key = b"XY";
    let encrypted = xor_encrypt(data, key);

    // Manual verification of key cycling: X, Y, X, Y, X, Y
    assert_eq!(encrypted[0], b'A' ^ b'X');
    assert_eq!(encrypted[1], b'B' ^ b'Y');
    assert_eq!(encrypted[2], b'C' ^ b'X');
    assert_eq!(encrypted[3], b'D' ^ b'Y');
    assert_eq!(encrypted[4], b'E' ^ b'X');
    assert_eq!(encrypted[5], b'F' ^ b'Y');
}

// ============================================================================
// FILE ENCRYPTION TESTS
// ============================================================================

#[test]
fn test_file_encrypt_decrypt_roundtrip() {
    let dir = tempdir("test_file_roundtrip");
    let input = format!("{}/plain.txt", dir);
    let encrypted_path = format!("{}/encrypted.bin", dir);
    let decrypted_path = format!("{}/decrypted.txt", dir);

    let original = "This is test content for file encryption.\nLine 2.\nLine 3.";
    let key = b"file-test-key";

    fs::write(&input, original).unwrap();

    let enc_size = encrypt_file(&input, &encrypted_path, key).unwrap();
    assert_eq!(enc_size, original.len());

    let dec_size = decrypt_file(&encrypted_path, &decrypted_path, key).unwrap();
    assert_eq!(dec_size, original.len());

    let recovered = fs::read_to_string(&decrypted_path).unwrap();
    assert_eq!(recovered, original);

    // Cleanup
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_file_encrypted_differs_from_original() {
    let dir = tempdir("test_file_differs");
    let input = format!("{}/plain.txt", dir);
    let encrypted_path = format!("{}/encrypted.bin", dir);

    let original = "Sensitive data that must be protected!";
    let key = b"protection-key";

    fs::write(&input, original).unwrap();
    encrypt_file(&input, &encrypted_path, key).unwrap();

    let original_bytes = fs::read(&input).unwrap();
    let encrypted_bytes = fs::read(&encrypted_path).unwrap();

    assert_ne!(original_bytes, encrypted_bytes);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_file_encrypt_empty_file() {
    let dir = tempdir("test_empty_file");
    let input = format!("{}/empty.txt", dir);
    let encrypted_path = format!("{}/encrypted.bin", dir);
    let decrypted_path = format!("{}/decrypted.txt", dir);

    fs::write(&input, "").unwrap();
    let key = b"empty-key";

    let enc_size = encrypt_file(&input, &encrypted_path, key).unwrap();
    assert_eq!(enc_size, 0);

    let dec_size = decrypt_file(&encrypted_path, &decrypted_path, key).unwrap();
    assert_eq!(dec_size, 0);

    let recovered = fs::read_to_string(&decrypted_path).unwrap();
    assert_eq!(recovered, "");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_file_encrypt_nonexistent_input() {
    let result = encrypt_file("/nonexistent/path/file.txt", "/tmp/out.bin", b"key");
    assert!(result.is_err());
}

#[test]
fn test_encrypt_file_buffered_roundtrip() {
    let dir = tempdir("test_buffered");
    let input = format!("{}/plain.txt", dir);
    let encrypted_path = format!("{}/encrypted.bin", dir);
    let decrypted_path = format!("{}/decrypted.txt", dir);

    // Create content larger than the 4KB buffer
    let original: String = "Hello, World! ".repeat(500);
    let key = b"buffered-key";

    fs::write(&input, &original).unwrap();

    let enc_size = encrypt_file_buffered(&input, &encrypted_path, key).unwrap();
    assert_eq!(enc_size, original.len());

    // Decrypt using the same buffered approach (symmetric chunk-by-chunk)
    // Note: The buffered approach resets key cycling at each chunk boundary,
    // so we must decrypt with the same chunk size for correct roundtrip.
    let dec_size = encrypt_file_buffered(&encrypted_path, &decrypted_path, key).unwrap();
    assert_eq!(dec_size, original.len());

    let recovered = fs::read_to_string(&decrypted_path).unwrap();
    assert_eq!(recovered, original);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_file_large_binary_data() {
    let dir = tempdir("test_binary");
    let input = format!("{}/binary.dat", dir);
    let encrypted_path = format!("{}/encrypted.bin", dir);
    let decrypted_path = format!("{}/decrypted.dat", dir);

    // Create binary data with all byte values
    let original: Vec<u8> = (0..=255).cycle().take(1024).collect();
    let key = b"binary-key-test";

    fs::write(&input, &original).unwrap();

    encrypt_file(&input, &encrypted_path, key).unwrap();
    decrypt_file(&encrypted_path, &decrypted_path, key).unwrap();

    let recovered = fs::read(&decrypted_path).unwrap();
    assert_eq!(recovered, original);

    let _ = fs::remove_dir_all(&dir);
}

// ============================================================================
// HELPER
// ============================================================================

/// Create a temporary directory for test files.
fn tempdir(name: &str) -> String {
    let dir = format!("/tmp/file_encryption_test_{}", name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
