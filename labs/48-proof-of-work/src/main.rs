// Project 45: Proof of Work
//
// Implements a sophisticated Proof of Work mining system extending the blockchain from Project 10.
// Demonstrates mining algorithms, difficulty adjustment, hash rate calculation, and mining economics.

use sha2::{Digest, Sha256};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== Proof of Work Mining ===\n");

    // Example 1: Basic proof of work
    println!("1. Basic Proof of Work Mining");
    basic_mining_demo();

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 2: Different difficulty levels
    println!("2. Mining at Different Difficulty Levels");
    difficulty_comparison_demo();

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 3: Hash rate calculation
    println!("3. Hash Rate Calculation");
    hash_rate_demo();

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 4: Difficulty adjustment
    println!("4. Difficulty Adjustment (Retargeting)");
    difficulty_adjustment_demo();

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 5: Mining a blockchain
    println!("5. Mining a Complete Blockchain");
    blockchain_mining_demo();

    println!("\n=== Mining Complete ===");
}

// ============================================================================
// BLOCK STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
    difficulty: usize,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            difficulty,
        }
    }

    fn genesis(difficulty: usize) -> Block {
        let mut block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let result = hasher.finalize();

        result.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn mine(&mut self) -> MiningResult {
        let start = Instant::now();
        let target = "0".repeat(self.difficulty);
        let mut attempts = 0u64;

        println!("  Mining block {} with difficulty {}...", self.index, self.difficulty);
        println!("  Target: Hash must start with '{}'", target);

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
            attempts += 1;

            // Progress indicator every million hashes
            if attempts % 1_000_000 == 0 {
                let elapsed = start.elapsed().as_secs_f64();
                let hash_rate = attempts as f64 / elapsed;
                print!("    {} million hashes, {:.2} MH/s\r", attempts / 1_000_000, hash_rate / 1_000_000.0);
            }
        }

        let duration = start.elapsed();
        let hash_rate = if duration.as_secs_f64() > 0.0 {
            attempts as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        println!("\n  ✓ Block mined!");
        println!("    Hash: {}", self.hash);
        println!("    Nonce: {}", self.nonce);
        println!("    Attempts: {}", attempts);
        println!("    Time: {:.3}s", duration.as_secs_f64());
        println!("    Hash rate: {:.2} H/s", hash_rate);

        MiningResult {
            nonce: self.nonce,
            attempts,
            duration,
            hash_rate,
            hash: self.hash.clone(),
        }
    }

    fn is_valid(&self) -> bool {
        // Check if hash starts with required number of zeros
        let target = "0".repeat(self.difficulty);
        if !self.hash.starts_with(&target) {
            return false;
        }

        // Verify hash is correct
        self.hash == self.calculate_hash()
    }
}

// ============================================================================
// MINING RESULT
// ============================================================================

#[derive(Debug)]
struct MiningResult {
    nonce: u64,
    attempts: u64,
    duration: Duration,
    hash_rate: f64,
    hash: String,
}

// ============================================================================
// BLOCKCHAIN WITH DIFFICULTY ADJUSTMENT
// ============================================================================

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    target_block_time: u64, // Target time per block in seconds
}

impl Blockchain {
    fn new(initial_difficulty: usize, target_block_time: u64) -> Blockchain {
        let genesis = Block::genesis(initial_difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty: initial_difficulty,
            target_block_time,
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Chain is empty");
        let mut new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
        );

        new_block.mine();
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        // Skip genesis block
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check if block is internally valid
            if !current.is_valid() {
                println!("❌ Block {} has invalid hash", i);
                return false;
            }

            // Check if block references correct previous hash
            if current.previous_hash != previous.hash {
                println!("❌ Block {} has invalid previous_hash", i);
                return false;
            }
        }

        println!("✅ Blockchain is valid!");
        true
    }

    // Adjust difficulty based on actual vs target block time
    fn adjust_difficulty(&mut self, adjustment_interval: usize) {
        if self.chain.len() < adjustment_interval {
            return;
        }

        // Get blocks from last adjustment interval
        let start_index = self.chain.len() - adjustment_interval;
        let start_block = &self.chain[start_index];
        let end_block = self.chain.last().unwrap();

        // Calculate actual time taken
        let actual_time = end_block.timestamp - start_block.timestamp;
        let expected_time = (adjustment_interval as u64) * self.target_block_time;

        println!("\n  Difficulty Adjustment:");
        println!("    Blocks: {} to {}", start_index, self.chain.len() - 1);
        println!("    Actual time: {}s", actual_time);
        println!("    Expected time: {}s", expected_time);

        // Adjust difficulty
        let old_difficulty = self.difficulty;

        if actual_time < expected_time / 2 {
            // Blocks are coming too fast, increase difficulty
            self.difficulty += 1;
            println!("    ⬆ Increasing difficulty: {} -> {}", old_difficulty, self.difficulty);
        } else if actual_time > expected_time * 2 {
            // Blocks are coming too slow, decrease difficulty
            if self.difficulty > 1 {
                self.difficulty -= 1;
                println!("    ⬇ Decreasing difficulty: {} -> {}", old_difficulty, self.difficulty);
            }
        } else {
            println!("    ➡ Difficulty unchanged: {}", self.difficulty);
        }
    }

    fn print_summary(&self) {
        println!("\n  Blockchain Summary:");
        println!("    Blocks: {}", self.chain.len());
        println!("    Current difficulty: {}", self.difficulty);

        if self.chain.len() > 1 {
            let first = &self.chain[1]; // Skip genesis
            let last = self.chain.last().unwrap();
            let time_span = last.timestamp - first.timestamp;
            let avg_block_time = if self.chain.len() > 1 {
                time_span as f64 / (self.chain.len() - 1) as f64
            } else {
                0.0
            };

            println!("    Time span: {}s", time_span);
            println!("    Avg block time: {:.2}s", avg_block_time);
        }
    }
}

// ============================================================================
// DEMO FUNCTIONS
// ============================================================================

fn basic_mining_demo() {
    let mut block = Block::new(1, "Alice sends 10 BTC to Bob".to_string(), "0".repeat(64), 4);

    println!("  Block before mining:");
    println!("    Index: {}", block.index);
    println!("    Data: {}", block.data);
    println!("    Difficulty: {}", block.difficulty);

    println!();
    block.mine();

    println!("\n  Block after mining:");
    println!("    Valid: {}", block.is_valid());
}

fn difficulty_comparison_demo() {
    let difficulties = vec![2, 3, 4, 5];

    println!("  Comparing mining times at different difficulties:\n");

    for difficulty in difficulties {
        let mut block = Block::new(
            1,
            format!("Test block at difficulty {}", difficulty),
            "0".repeat(64),
            difficulty,
        );

        let result = block.mine();

        println!();
        println!("  Difficulty {} summary:", difficulty);
        println!("    Theoretical avg attempts: ~{}", 16_u64.pow(difficulty as u32));
        println!("    Actual attempts: {}", result.attempts);
        println!("    Time: {:.3}s", result.duration.as_secs_f64());
        println!("    Hash rate: {:.2} MH/s", result.hash_rate / 1_000_000.0);
        println!();
    }
}

fn hash_rate_demo() {
    println!("  Benchmarking hash rate...\n");

    let mut block = Block::new(1, "Benchmark block".to_string(), "0".repeat(64), 1);

    let start = Instant::now();
    let mut hashes = 0u64;
    let benchmark_duration = Duration::from_secs(2);

    println!("  Hashing for 2 seconds...");

    while start.elapsed() < benchmark_duration {
        block.nonce += 1;
        let _ = block.calculate_hash();
        hashes += 1;
    }

    let elapsed = start.elapsed().as_secs_f64();
    let hash_rate = hashes as f64 / elapsed;

    println!("\n  Benchmark results:");
    println!("    Total hashes: {}", hashes);
    println!("    Time: {:.3}s", elapsed);
    println!("    Hash rate: {:.2} H/s", hash_rate);
    println!("    Hash rate: {:.2} KH/s", hash_rate / 1_000.0);
    println!("    Hash rate: {:.2} MH/s", hash_rate / 1_000_000.0);

    println!("\n  Estimated time to mine at different difficulties:");
    for difficulty in 1..=8 {
        let expected_attempts = 16_u64.pow(difficulty as u32);
        let expected_time = expected_attempts as f64 / hash_rate;

        if expected_time < 60.0 {
            println!("    Difficulty {}: {:.2} seconds", difficulty, expected_time);
        } else if expected_time < 3600.0 {
            println!("    Difficulty {}: {:.2} minutes", difficulty, expected_time / 60.0);
        } else if expected_time < 86400.0 {
            println!("    Difficulty {}: {:.2} hours", difficulty, expected_time / 3600.0);
        } else {
            println!("    Difficulty {}: {:.2} days", difficulty, expected_time / 86400.0);
        }
    }
}

fn difficulty_adjustment_demo() {
    let mut blockchain = Blockchain::new(3, 5); // Difficulty 3, target 5 seconds per block

    println!("  Initial difficulty: {}", blockchain.difficulty);
    println!("  Target block time: {}s", blockchain.target_block_time);
    println!();

    // Mine some blocks
    for i in 1..=6 {
        blockchain.add_block(format!("Transaction block {}", i));

        // Adjust difficulty every 3 blocks
        if i % 3 == 0 {
            blockchain.adjust_difficulty(3);
        }
    }

    blockchain.print_summary();
}

fn blockchain_mining_demo() {
    let mut blockchain = Blockchain::new(3, 10);

    println!("  Creating blockchain with genesis block...");
    println!("  Genesis hash: {}", blockchain.chain[0].hash);
    println!();

    // Add blocks with transactions
    let transactions = vec![
        "Alice sends 10 BTC to Bob",
        "Bob sends 5 BTC to Charlie",
        "Charlie sends 2 BTC to Diana",
    ];

    for (i, tx) in transactions.iter().enumerate() {
        println!("  [{}/{}] Mining block with transaction: {}", i + 1, transactions.len(), tx);
        blockchain.add_block(tx.to_string());
        println!();
    }

    println!("  === Final Blockchain ===");
    blockchain.print_summary();

    println!("\n  Validating blockchain...");
    blockchain.is_valid();

    println!("\n  Block details:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("    Block {}:", i);
        println!("      Data: {}", block.data);
        println!("      Hash: {}...", &block.hash[..16]);
        println!("      Nonce: {}", block.nonce);
        println!("      Difficulty: {}", block.difficulty);
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. SHA-256 HASHING
//    - SHA-256 is a cryptographic hash function
//    - Takes any input, produces 256-bit (64 hex chars) output
//    - Deterministic: same input always gives same hash
//    - Avalanche effect: tiny input change changes ~50% of output bits
//    - One-way: can't reverse hash to get original input
//
// 2. BRUTE FORCE SEARCH
//    - Mining tries different nonces sequentially (0, 1, 2, 3, ...)
//    - Each nonce produces a different hash
//    - Keep trying until hash meets difficulty requirement
//    - This is a "proof" that work was done (can't fake it)
//
// 3. DIFFICULTY SCALING
//    - Difficulty 1: 1/16 chance (16 attempts avg)
//    - Difficulty 2: 1/256 chance (256 attempts avg)
//    - Difficulty 3: 1/4,096 chance (4,096 attempts avg)
//    - Each +1 difficulty multiplies attempts by 16!
//
// 4. MEMORY MANAGEMENT
//    - String allocations for hashes (heap allocated)
//    - Block struct is relatively large (~200 bytes)
//    - Blockchain Vec grows dynamically
//    - All memory freed automatically when out of scope
//
// 5. TIME MEASUREMENT
//    - Instant::now() uses monotonic clock (doesn't jump)
//    - SystemTime for timestamps (can jump forward/backward)
//    - Duration for time differences
//    - All at nanosecond precision

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Proof of Work = finding a hash that meets difficulty requirement
// 2. Mining is brute-force search (try nonces until success)
// 3. Difficulty scales exponentially (each +1 is 16x harder)
// 4. Hash rate = hashes per second (benchmark your hardware)
// 5. Difficulty adjustment keeps block time consistent
// 6. Mining secures the blockchain (expensive to rewrite)
// 7. Nonce is the only variable that changes during mining
// 8. Valid hash can be verified instantly (but finding it is hard)
// 9. Network difficulty adjusts based on total hash power
// 10. Real Bitcoin difficulty is ~20 leading zeros (trillions of attempts)

// ============================================================================
// PROOF OF WORK PROPERTIES
// ============================================================================
// 1. HARD TO SOLVE
//    - Finding a valid hash requires brute force
//    - No shortcuts (can't predict which nonce works)
//    - Difficulty can be adjusted to control speed
//
// 2. EASY TO VERIFY
//    - Just hash once and check if it meets difficulty
//    - Anyone can verify without redoing the work
//    - O(1) verification vs O(2^n) mining
//
// 3. ADJUSTABLE DIFFICULTY
//    - Can increase/decrease to target specific block time
//    - Bitcoin adjusts every 2016 blocks (~2 weeks)
//    - Maintains ~10 minute block time despite hash rate changes
//
// 4. UNPREDICTABLE
//    - Can't predict which nonce will work
//    - Follows Poisson distribution (memoryless)
//    - Finding one block doesn't help find the next

// ============================================================================
// MINING ECONOMICS
// ============================================================================
// 1. COSTS
//    - Hardware (ASICs, GPUs, etc.)
//    - Electricity (biggest ongoing cost)
//    - Cooling (data center infrastructure)
//    - Maintenance and replacements
//
// 2. REVENUE
//    - Block reward (currently 6.25 BTC for Bitcoin)
//    - Transaction fees
//    - Halves every 210,000 blocks (~4 years)
//
// 3. PROFITABILITY
//    - Depends on: electricity cost, hardware efficiency, Bitcoin price
//    - Most profitable with cheap electricity (<$0.05/kWh)
//    - Requires economies of scale (large mining farms)
//    - Individual mining rarely profitable anymore

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Starting with too high difficulty (won't finish in reasonable time)
// ❌ Not incrementing nonce (infinite loop with same hash)
// ❌ Checking wrong condition (bits vs leading zeros)
// ❌ Integer overflow on nonce (use u64, not u32)
// ❌ Not measuring hash rate (can't optimize)
// ❌ Forgetting difficulty affects security (too low = insecure)
// ❌ Not handling mining cancellation (can't interrupt)
// ❌ Ignoring timestamp in hash (allows manipulation)

// ============================================================================
// OPTIMIZATION TECHNIQUES
// ============================================================================
// 1. PARALLEL MINING
//    - Use multiple CPU cores (rayon, std::thread)
//    - Each thread tries different nonce ranges
//    - 8 cores = ~8x faster mining
//
// 2. SIMD INSTRUCTIONS
//    - SHA-256 can use SIMD (AVX2, SSE)
//    - Process multiple hashes simultaneously
//    - 2-4x speedup on modern CPUs
//
// 3. GPU MINING
//    - GPUs have thousands of cores
//    - Excellent for parallel brute force
//    - 100-1000x faster than CPU
//
// 4. ASIC MINING
//    - Application-Specific Integrated Circuit
//    - Custom chips designed only for SHA-256
//    - 10,000-100,000x faster than CPU
//    - Used by all serious Bitcoin miners
//
// 5. ALGORITHM OPTIMIZATION
//    - Cache partial hash calculations
//    - Minimize allocations in hot path
//    - Inline critical functions

// ============================================================================
// SECURITY CONSIDERATIONS
// ============================================================================
// 1. 51% ATTACK
//    - Attacker with >50% hash power can rewrite blockchain
//    - Can double-spend, prevent confirmations
//    - Bitcoin network hash rate makes this extremely expensive
//
// 2. SELFISH MINING
//    - Miners hide found blocks, release strategically
//    - Can gain unfair advantage with ~25% hash power
//    - Real-world impact is debated
//
// 3. DIFFICULTY MANIPULATION
//    - Attacker could manipulate timestamps to lower difficulty
//    - Bitcoin has timestamp validation rules to prevent this
//
// 4. MINING CENTRALIZATION
//    - Large mining pools control majority of hash power
//    - Reduces decentralization (counter to Bitcoin's goals)
//    - Geographic concentration (cheap electricity regions)

// ============================================================================
// REAL-WORLD COMPARISONS
// ============================================================================
// BITCOIN (SHA-256):
//   - Difficulty: ~50 trillion
//   - Hash rate: ~400 EH/s (400 × 10^18)
//   - Block time: ~10 minutes
//   - Block reward: 6.25 BTC
//
// ETHEREUM (before PoS):
//   - Algorithm: Ethash (memory-hard)
//   - Block time: ~13 seconds
//   - More ASIC-resistant
//
// MONERO (RandomX):
//   - CPU-optimized algorithm
//   - ASIC-resistant
//   - Favors decentralized mining
//
// LITECOIN (Scrypt):
//   - Memory-hard (initially)
//   - Block time: 2.5 minutes
//   - Eventually had ASICs too

// ============================================================================
// FUTURE OF PROOF OF WORK
// ============================================================================
// - Bitcoin will likely continue using PoW (security proven)
// - Ethereum switched to Proof of Stake (energy efficiency)
// - Debate: Security vs Environmental Impact
// - Innovation: Green mining (renewable energy)
// - Alternative: Proof of Space, Proof of Stake, BFT consensus
