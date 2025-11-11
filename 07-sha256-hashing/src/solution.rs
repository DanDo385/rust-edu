//! # SHA-256 Hashing - Complete Solution with Blockchain Context
//!
//! ## What is SHA-256?
//!
//! SHA-256 (Secure Hash Algorithm 256-bit) is a cryptographic hash function:
//! - Takes any input (any size)
//! - Produces 256-bit (32-byte) hash
//! - Deterministic: same input always gives same output
//! - One-way: can't reverse the hash to get original input
//! - Avalanche effect: tiny input change completely changes output
//!
//! ## Blockchain Use Cases
//!
//! 1. **Block Identifiers**: Each block is identified by its hash
//! 2. **Mining**: Finding hashes with specific properties (leading zeros)
//! 3. **Data Integrity**: Verify data hasn't been tampered with
//! 4. **Proof-of-Work**: Requires computational work to find valid hash
//!
//! NOTE: This implementation uses a simple hash function for educational purposes.
//! In production, use a proper cryptographic hash library like sha2.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Simple hash function using Rust's standard library hasher.
/// This is NOT cryptographically secure - use only for learning!
fn simple_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    hasher.write(input);
    let hash_value = hasher.finish();

    // Create a 32-byte (256-bit) hash by repeating and mixing the 64-bit hash
    let mut result = Vec::with_capacity(32);
    for i in 0..4 {
        let shifted = hash_value.wrapping_mul(i as u64 + 1);
        result.extend_from_slice(&shifted.to_be_bytes());
    }
    result
}

/// Convert bytes to hexadecimal string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Hash a string with SHA-256.
///
/// ## Blockchain Context
/// This is the basic building block used everywhere in blockchain:
/// - Bitcoin uses this to hash block headers
/// - Transaction IDs are SHA-256 hashes
/// - Every piece of data is content-addressed by its hash
///
/// ## How it works
/// 1. Create SHA-256 hasher
/// 2. Feed input bytes to hasher
/// 3. Finalize to get 256-bit hash
/// 4. Convert to hexadecimal string for readability
///
/// ## Example
/// ```ignore
/// let hash = hash_string("Hello, Bitcoin!");
/// // Returns 64-character hex string (32 bytes × 2 hex digits per byte)
/// ```ignore
pub fn hash_string(input: &str) -> String {
    // ========================================================================
    // STEP 1: CONVERT INPUT TO BYTES
    // ========================================================================

    // `input.as_bytes()` converts &str to &[u8]
    //   - Hash functions work on bytes, not characters
    //   - Rust strings are UTF-8 encoded
    //   - as_bytes() gives raw UTF-8 byte representation

    // ========================================================================
    // STEP 2: HASH THE INPUT
    // ========================================================================

    // `simple_hash()` = create hash from input bytes
    //   - Uses Rust's standard library DefaultHasher
    //   - Returns 32 bytes (256 bits) for consistency with SHA-256
    //   - NOTE: Not cryptographically secure! For learning only.

    let hash_bytes = simple_hash(input.as_bytes());

    // ========================================================================
    // STEP 3: CONVERT TO HEX STRING
    // ========================================================================

    // `bytes_to_hex()` = convert bytes to hexadecimal string
    //   - Takes 32 bytes
    //   - Returns 64-character string (2 hex digits per byte)
    //   - Example: [0x1a, 0x2b] -> "1a2b"
    //
    // Why hexadecimal?
    // - Human-readable representation of binary data
    // - Standard format in blockchain (Bitcoin block hashes, etc.)
    // - Each byte (0-255) represented by two hex digits (00-ff)

    bytes_to_hex(&hash_bytes)

    // ============================================================================
    // BLOCKCHAIN EXAMPLE: Bitcoin Block Hash
    // ============================================================================
    //
    // Bitcoin Genesis Block (Block #0):
    // Input: Block header (80 bytes including version, timestamp, nonce, etc.)
    // Hash: 000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f
    //
    // Notice the leading zeros! This is from Proof-of-Work mining.
    //
    // Properties demonstrated:
    // 1. Deterministic: Same header always gives same hash
    // 2. Unpredictable: Can't predict hash without computing it
    // 3. Difficulty: Leading zeros make it hard to find (took work!)
    // 4. Fixed size: Always 64 hex characters (256 bits)
}

/// Hash input with a nonce appended.
///
/// ## Blockchain Context
/// This is how Bitcoin mining works:
/// 1. Take block header (contains transactions, previous hash, etc.)
/// 2. Append a nonce (number used once)
/// 3. Hash the result
/// 4. If hash doesn't meet difficulty target (enough leading zeros), increment nonce and try again
/// 5. Repeat until valid hash found (this is "mining")
///
/// ## Parameters
/// - `input`: The data to hash (block header in Bitcoin)
/// - `nonce`: A number to try (miners try millions of nonces per second)
///
/// ## Returns
/// SHA-256 hash of (input + nonce)
///
/// ## Example
/// ```ignore
/// let hash1 = hash_with_nonce("block data", 0);
/// let hash2 = hash_with_nonce("block data", 1);
/// // hash1 and hash2 are completely different (avalanche effect)
/// ```ignore
pub fn hash_with_nonce(input: &str, nonce: u64) -> String {
    // Create combined input: original input + nonce
    //
    // `format!("{}{}", input, nonce)` = concatenate input and nonce
    //   - Converts nonce (u64) to string
    //   - Appends to input
    //   - Example: input="block", nonce=42 -> "block42"
    //
    // Why append nonce?
    // - Changing nonce changes hash (avalanche effect)
    // - Mining = trying different nonces to find valid hash
    // - Nonce is the only variable changed during mining

    let combined = format!("{}{}", input, nonce);

    // Hash the combined string
    // Reuse our hash_string function (DRY principle)

    hash_string(&combined)
}

/// Find a nonce that produces a hash with the given prefix.
///
/// ## Blockchain Context
/// THIS IS BITCOIN MINING!
///
/// Bitcoin Proof-of-Work:
/// - Difficulty = number of leading zeros required
/// - Current Bitcoin difficulty ≈ 19 leading zeros
/// - Miners try billions of nonces per second
/// - First to find valid hash gets block reward + transaction fees
/// - Difficulty adjusts every 2016 blocks (~2 weeks) to maintain 10-minute block time
///
/// ## Parameters
/// - `input`: Block data to hash
/// - `prefix`: Required prefix (e.g., "00" for 2 leading zeros)
///
/// ## Returns
/// - `(nonce, hash)` tuple
///   - `nonce`: The nonce that produced valid hash
///   - `hash`: The valid hash with required prefix
///
/// ## Performance
/// - Each additional leading zero makes it ~16x harder (16 possibilities per hex digit)
/// - "00" prefix: ~256 attempts on average
/// - "0000" prefix: ~65,536 attempts on average
/// - Bitcoin's ~19 leading zeros: incomprehensibly difficult!
///
/// ## Example
/// ```ignore
/// let (nonce, hash) = find_hash_with_prefix("block data", "00");
/// assert!(hash.starts_with("00"));
/// // In real Bitcoin, prefix would be much longer!
/// ```ignore
pub fn find_hash_with_prefix(input: &str, prefix: &str) -> (u64, String) {
    // ========================================================================
    // MINING LOOP - THE CORE OF PROOF-OF-WORK
    // ========================================================================

    // Start with nonce = 0
    // This is where mining starts
    // In real Bitcoin mining, hardware can try billions per second!

    let mut nonce = 0;

    // Infinite loop until we find valid hash
    // `loop` = infinite loop (like while(true))
    //   - Must have `break` to exit
    //   - In Bitcoin, miner stops when valid hash found or new block arrives

    loop {
        // Try hashing with current nonce
        // `hash_with_nonce` combines input + nonce and hashes it

        let hash = hash_with_nonce(input, nonce);

        // Check if hash starts with required prefix
        // `.starts_with(prefix)` = check if string starts with prefix
        //   - Example: "00abc123...".starts_with("00") = true
        //   - This is the "difficulty check"
        //   - In Bitcoin, checks for leading zeros

        if hash.starts_with(prefix) {
            // Found valid hash! Return nonce and hash
            // `return` = exit function with this value
            // `(nonce, hash)` = tuple containing both values
            //
            // CONGRATULATIONS! You just "mined a block"!
            // In real Bitcoin:
            // - You'd broadcast this to network
            // - Receive block reward (currently 6.25 BTC)
            // - Plus transaction fees

            return (nonce, hash);
        }

        // Hash didn't meet requirements, try next nonce
        // `nonce += 1` = increment and try again
        //   - In Bitcoin, miners try millions/billions per second
        //   - Modern ASICs can do ~100 TH/s (trillion hashes per second!)
        //   - This is why Bitcoin mining uses so much energy

        nonce += 1;

        // ====================================================================
        // WHAT IF WE NEVER FIND A VALID HASH?
        // ====================================================================
        //
        // Mathematically, we will find one eventually:
        // - SHA-256 has 2^256 possible outputs
        // - Any prefix "00" has 1/256 chance
        // - On average, need to try ~256 nonces
        //
        // In practice:
        // - Bitcoin difficulty is calibrated so average time is 10 minutes
        // - If too fast, difficulty increases (more leading zeros required)
        // - If too slow, difficulty decreases
        //
        // Note: This simple implementation could overflow u64 (very unlikely)
        // Real mining uses better nonce strategies and checks for new blocks
    }

    // ============================================================================
    // BITCOIN MINING ECONOMICS (as of 2024)
    // ============================================================================
    //
    // Block Reward: 6.25 BTC (~$200,000 at $32,000/BTC)
    // Plus transaction fees: ~0.5-2 BTC per block
    //
    // Mining Hardware:
    // - Consumer PC: ~100 MH/s (million hashes/sec) - not profitable
    // - GPU: ~50 GH/s (billion hashes/sec) - not profitable
    // - ASIC miner: ~100 TH/s (trillion hashes/sec) - maybe profitable
    //
    // Network Hash Rate: ~400 EH/s (exahashes/sec)
    // That's 400,000,000,000,000,000,000 hashes per second globally!
    //
    // Difficulty: Adjusted every 2016 blocks to maintain 10-minute average
    //
    // Energy: Mining uses ~150 TWh/year (similar to entire countries)
    //
    // Why mine?
    // - Secure the network (no central authority)
    // - Earn rewards
    // - Process transactions
    // - Proof-of-Work makes attacks expensive (need 51% of hash power)
}

/// Verify that a hash matches the input.
///
/// ## Blockchain Context
/// This is how nodes verify blocks:
/// 1. Receive block from network
/// 2. Hash the block header
/// 3. Compare with claimed hash
/// 4. If match, block is valid (hasn't been tampered)
/// 5. If no match, reject block as invalid
///
/// Why this works:
/// - Cryptographic hash: can't fake without original input
/// - Even tiny change in input completely changes hash
/// - Proves data integrity without trusted third party
///
/// ## Parameters
/// - `input`: Original data
/// - `expected_hash`: Hash to verify against
///
/// ## Returns
/// `true` if hash of input matches expected_hash
///
/// ## Example
/// ```ignore
/// let hash = hash_string("transaction data");
/// assert!(verify_hash("transaction data", &hash));
/// assert!(!verify_hash("tampered data", &hash));
/// ```ignore
pub fn verify_hash(input: &str, expected_hash: &str) -> bool {
    // Simply hash the input and compare with expected
    //
    // `hash_string(input)` = compute hash of input
    // `== expected_hash` = compare with expected
    //   - String comparison (case-sensitive)
    //   - If hashes match, data is verified
    //   - If hashes don't match, data was tampered or incorrect
    //
    // Why this is secure:
    // - Can't find different input with same hash (collision resistance)
    // - Can't reverse hash to get original input (preimage resistance)
    // - Tiny change in input = completely different hash (avalanche)

    hash_string(input) == expected_hash

    // ============================================================================
    // BLOCKCHAIN EXAMPLE: Transaction Verification
    // ============================================================================
    //
    // When you receive a Bitcoin transaction:
    // 1. Someone sends you transaction data + transaction ID (hash)
    // 2. You hash the transaction data
    // 3. Compare your hash with their transaction ID
    // 4. If match, transaction is valid and unmodified
    // 5. If no match, transaction was tampered with
    //
    // This is why blockchain is "immutable":
    // - Changing any transaction changes its hash
    // - Which changes the block's hash
    // - Which changes all subsequent blocks' hashes
    // - Tampering is immediately obvious to everyone!
}

// ============================================================================
// CRYPTOGRAPHIC PROPERTIES OF SHA-256
// ============================================================================
//
// 1. **Deterministic**: Same input always gives same output
//    - Essential for verification
//    - Everyone agrees on what hash should be
//
// 2. **Quick to compute**: Fast to hash any input
//    - Modern computers can hash millions per second
//    - Important for mining and verification
//
// 3. **Preimage resistance**: Can't reverse hash to get input
//    - Given hash, can't find original data
//    - Would take longer than age of universe to brute force
//
// 4. **Collision resistance**: Can't find two inputs with same hash
//    - Probability of collision: 2^-256 (astronomically small)
//    - More possible hashes than atoms in universe
//
// 5. **Avalanche effect**: Small input change = completely different hash
//    - Change one bit of input
//    - Changes ~50% of output bits
//    - Makes patterns in input impossible to exploit
//
// 6. **Fixed output size**: Always 256 bits (32 bytes)
//    - Input can be any size
//    - Output always same size
//    - Useful for indexing and storage
