// Project 13: Collections and Iterators
//
// This program demonstrates Rust's standard library collections and powerful
// iterator system by building a word frequency counter. We'll see how iterator
// chains provide zero-cost abstractions - high-level code that compiles to
// fast machine code with no runtime overhead.
//
// Key Concepts:
// - Vec<T>: Growable arrays
// - HashMap<K, V>: Hash tables for key-value storage
// - HashSet<T>: Sets for unique values
// - Iterator trait and adaptors (map, filter, fold, etc.)
// - Zero-cost abstractions: functional style with imperative performance

use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("=== Collections and Iterators ===\n");

    // ========================================================================
    // PART 1: Vec<T> - Dynamic Arrays
    // ========================================================================
    println!("--- Part 1: Vec<T> - Dynamic Arrays ---\n");

    // Vec is the most common collection in Rust
    // It's a growable array stored on the heap

    // Creating vectors
    let mut numbers: Vec<i32> = Vec::new();  // Empty vec, type annotation needed
    let mut primes = vec![2, 3, 5, 7, 11];  // vec! macro, type inferred

    // Adding elements
    numbers.push(10);  // Add to end - O(1) amortized
    numbers.push(20);
    numbers.push(30);
    println!("Numbers: {:?}", numbers);

    // Accessing elements
    let first = numbers[0];  // Panics if index out of bounds
    let second = numbers.get(1);  // Returns Option<&T> - safer
    println!("First: {}, Second: {:?}", first, second);

    // Iterating over Vec
    println!("Iterating:");
    for num in &numbers {  // Borrow each element
        print!("{} ", num);
    }
    println!();

    // Modifying while iterating (needs mutable iterator)
    for num in &mut numbers {
        *num *= 2;  // Double each element
    }
    println!("After doubling: {:?}", numbers);

    // Vec capacity vs length
    println!("\nCapacity management:");
    println!("  Length: {} (how many elements)", numbers.len());
    println!("  Capacity: {} (allocated space)", numbers.capacity());

    // Pre-allocate capacity for better performance
    let mut optimized = Vec::with_capacity(100);
    println!("  Optimized capacity: {}", optimized.capacity());
    optimized.push(1);
    println!("  After push, capacity still: {}", optimized.capacity());

    println!();

    // ========================================================================
    // PART 2: HashMap<K, V> - Key-Value Storage
    // ========================================================================
    println!("--- Part 2: HashMap<K, V> - Key-Value Storage ---\n");

    // HashMap stores key-value pairs with O(1) average lookup time
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting values
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 75);
    scores.insert("Charlie".to_string(), 90);

    // Accessing values
    let alice_score = scores.get("Alice");  // Returns Option<&V>
    println!("Alice's score: {:?}", alice_score);

    // Safe access with default
    let david_score = scores.get("David").copied().unwrap_or(0);
    println!("David's score (with default): {}", david_score);

    // Iterating over HashMap
    println!("\nAll scores:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // THE ENTRY API - Most important HashMap pattern!
    // This is how you efficiently insert/update values
    println!("\nEntry API demonstration:");

    // Update existing or insert new
    scores.entry("Alice".to_string())
        .and_modify(|score| *score += 10)  // Add 10 if exists
        .or_insert(50);  // Insert 50 if doesn't exist

    // Common pattern: increment counter
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for word in vec!["hello", "world", "hello", "rust", "world", "hello"] {
        // This is THE way to count occurrences in Rust!
        // or_insert returns &mut V, so we can modify it directly
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    println!("Word counts: {:?}", word_counts);

    println!();

    // ========================================================================
    // PART 3: HashSet<T> - Unique Values
    // ========================================================================
    println!("--- Part 3: HashSet<T> - Unique Values ---\n");

    // HashSet is like HashMap but only stores keys (no values)
    // Perfect for testing membership and ensuring uniqueness

    let mut seen_words: HashSet<String> = HashSet::new();

    let words = vec!["apple", "banana", "apple", "cherry", "banana", "date"];

    println!("Processing words:");
    for word in words {
        // insert returns true if value was not present
        if seen_words.insert(word.to_string()) {
            println!("  New word: {}", word);
        } else {
            println!("  Duplicate: {}", word);
        }
    }

    println!("\nUnique words: {:?}", seen_words);

    // Set operations
    let set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![4, 5, 6, 7, 8].into_iter().collect();

    let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
    let union: HashSet<_> = set1.union(&set2).copied().collect();
    let difference: HashSet<_> = set1.difference(&set2).copied().collect();

    println!("\nSet operations:");
    println!("  Set1: {:?}", set1);
    println!("  Set2: {:?}", set2);
    println!("  Intersection: {:?}", intersection);
    println!("  Union: {:?}", union);
    println!("  Difference (set1 - set2): {:?}", difference);

    println!();

    // ========================================================================
    // PART 4: String Operations
    // ========================================================================
    println!("--- Part 4: String Operations ---\n");

    // String is a growable, UTF-8 encoded text type
    let mut greeting = String::from("Hello");

    // Appending
    greeting.push_str(", world");  // Append &str
    greeting.push('!');            // Append single char
    println!("Greeting: {}", greeting);

    // Concatenation
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let message = hello + &world;  // hello moved here, world borrowed
    // println!("{}", hello);  // ❌ ERROR: hello was moved
    println!("Message: {}", message);

    // Better concatenation with format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let combined = format!("{}-{}-{}", s1, s2, s3);  // Doesn't take ownership!
    println!("Combined: {}", combined);
    println!("s1 still valid: {}", s1);  // ✅ OK

    // String slicing and iteration
    let text = "Hello, 世界";  // UTF-8 string with multi-byte characters

    // Cannot index directly: text[0] ❌ ERROR
    // Because UTF-8 characters can be multiple bytes

    // Iterate by characters
    print!("Characters: ");
    for c in text.chars() {
        print!("{} ", c);
    }
    println!();

    // Iterate by bytes
    print!("Bytes: ");
    for b in text.bytes() {
        print!("{} ", b);
    }
    println!();

    // Split and collect
    let words: Vec<&str> = "one two three four".split_whitespace().collect();
    println!("Words: {:?}", words);

    println!();

    // ========================================================================
    // PART 5: Iterator Basics
    // ========================================================================
    println!("--- Part 5: Iterator Basics ---\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // Three ways to iterate:
    println!("1. Borrowing (read-only):");
    for num in &numbers {  // .iter() implicitly
        print!("{} ", num);
    }
    println!();

    println!("2. Mutable borrowing:");
    let mut numbers_mut = numbers.clone();
    for num in &mut numbers_mut {  // .iter_mut() implicitly
        *num *= 2;
    }
    println!("{:?}", numbers_mut);

    println!("3. Taking ownership (consuming):");
    for num in numbers {  // .into_iter() implicitly
        print!("{} ", num);
    }
    println!();
    // println!("{:?}", numbers);  // ❌ ERROR: numbers was moved

    println!();

    // ========================================================================
    // PART 6: Iterator Adaptors and Chains
    // ========================================================================
    println!("--- Part 6: Iterator Adaptors (Zero-Cost Abstractions!) ---\n");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map - transform each element
    let doubled: Vec<i32> = data.iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);

    // filter - keep elements matching predicate
    let evens: Vec<i32> = data.iter()
        .filter(|x| *x % 2 == 0)
        .copied()  // Convert &i32 to i32
        .collect();
    println!("Evens: {:?}", evens);

    // Chaining multiple operations
    let result: i32 = data.iter()
        .filter(|x| *x % 2 == 0)  // Keep evens
        .map(|x| x * x)            // Square them
        .sum();                    // Sum them up
    println!("Sum of squares of evens: {}", result);

    // This compiles to the same assembly as:
    let mut result_manual = 0;
    for x in &data {
        if x % 2 == 0 {
            result_manual += x * x;
        }
    }
    println!("Manual version: {}", result_manual);
    // But iterator version is often MORE readable!

    // fold - reduce to a single value with accumulator
    let product: i32 = vec![1, 2, 3, 4, 5].iter()
        .fold(1, |acc, x| acc * x);
    println!("Product using fold: {}", product);

    // enumerate - get index with value
    let items = vec!["a", "b", "c"];
    let indexed: Vec<_> = items.iter()
        .enumerate()
        .collect();
    println!("Enumerated: {:?}", indexed);  // [(0, "a"), (1, "b"), (2, "c")]

    // zip - combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let combined: Vec<_> = names.iter()
        .zip(ages.iter())
        .collect();
    println!("Zipped: {:?}", combined);

    // take and skip
    let first_three: Vec<_> = data.iter().take(3).collect();
    let skip_five: Vec<_> = data.iter().skip(5).collect();
    println!("First 3: {:?}", first_three);
    println!("Skip 5: {:?}", skip_five);

    // any and all
    let has_even = data.iter().any(|x| x % 2 == 0);
    let all_positive = data.iter().all(|x| *x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);

    println!();

    // ========================================================================
    // PART 7: Practical Example - Word Frequency Counter
    // ========================================================================
    println!("--- Part 7: Word Frequency Counter ---\n");

    // Read this source file as input
    let filename = "src/main.rs";
    println!("Reading file: {}", filename);

    let contents = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            println!("Using sample text instead.");
            "the quick brown fox jumps over the lazy dog the fox was quick".to_string()
        }
    };

    // Count word frequencies using HashMap
    let word_counts = count_words(&contents);

    // Get top N most common words
    let top_words = get_top_n_words(&word_counts, 20);

    println!("\nTop 20 most common words:");
    for (rank, (word, count)) in top_words.iter().enumerate() {
        println!("  {:2}. {:15} - {} occurrences", rank + 1, word, count);
    }

    // Statistics using iterators
    let total_words: usize = word_counts.values().sum();
    let unique_words = word_counts.len();
    let avg_frequency = total_words as f64 / unique_words as f64;

    println!("\nStatistics:");
    println!("  Total words: {}", total_words);
    println!("  Unique words: {}", unique_words);
    println!("  Average frequency: {:.2}", avg_frequency);

    // Find long words (> 10 characters) using iterator chains
    let long_words: Vec<_> = word_counts.keys()
        .filter(|word| word.len() > 10)
        .take(10)
        .collect();

    println!("\nSome long words (>10 chars): {:?}", long_words);

    println!();

    // ========================================================================
    // PART 8: Performance Demonstration
    // ========================================================================
    println!("--- Part 8: Performance - Iterators vs Loops ---\n");

    let large_data: Vec<i32> = (1..=1000).collect();

    // Iterator chain (zero-cost abstraction)
    let sum_iterator: i32 = large_data.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum();

    // Manual loop (what the iterator compiles to)
    let mut sum_manual = 0;
    for x in &large_data {
        if x % 2 == 0 {
            sum_manual += x * x;
        }
    }

    println!("Iterator result: {}", sum_iterator);
    println!("Manual loop result: {}", sum_manual);
    println!("\nThese compile to nearly identical assembly code!");
    println!("Iterators are a ZERO-COST abstraction.");

    println!();

    println!("=== Program Complete ===");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Count word frequencies in a text using HashMap and iterators
///
/// This demonstrates:
/// - String processing with split and iteration
/// - HashMap entry API for efficient counting
/// - Iterator chains for data transformation
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    // Process each word
    text.split_whitespace()  // Split by whitespace (iterator of &str)
        .flat_map(|word| word.split(&['(', ')', '{', '}', '[', ']', ',', '.', ';', ':', '!', '?']))
        .map(|word| word.trim())  // Remove leading/trailing whitespace
        .filter(|word| !word.is_empty())  // Skip empty strings
        .map(|word| word.to_lowercase())  // Normalize to lowercase
        .for_each(|word| {
            // Entry API: efficient way to insert/update
            // or_insert returns &mut V, so we can increment directly
            *counts.entry(word).or_insert(0) += 1;
        });

    counts
}

/// Get top N words by frequency
///
/// This demonstrates:
/// - Converting HashMap to Vec for sorting
/// - Sorting with custom comparator
/// - Taking a subset of results
fn get_top_n_words(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut word_vec: Vec<(String, usize)> = counts.iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    // Sort by count (descending), then by word (ascending) for ties
    word_vec.sort_by(|a, b| {
        b.1.cmp(&a.1)  // Count descending
            .then_with(|| a.0.cmp(&b.0))  // Word ascending
    });

    // Take top N
    word_vec.into_iter()
        .take(n)
        .collect()
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Vec<T> is a growable array on the heap - use it for dynamic lists
// 2. HashMap<K, V> provides O(1) average lookup - use Entry API for efficiency
// 3. HashSet<T> ensures uniqueness - use for membership testing
// 4. String is UTF-8 encoded - can't index by position, iterate by chars or bytes
// 5. Iterators are LAZY - nothing happens until consumed (collect, sum, etc.)
// 6. Iterator chains are ZERO-COST - compile to same code as manual loops
// 7. Use .iter() for borrowing, .iter_mut() for mutable, .into_iter() for ownership
// 8. Common adaptors: map, filter, fold, collect, sum, any, all, find
// 9. Entry API pattern: *map.entry(key).or_insert(default) += 1
// 10. Collections own their data - moved values are transferred to collection

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Trying to modify collection while iterating over it
// ❌ Forgetting that iterators are lazy (need to collect/consume)
// ❌ Using multiple HashMap lookups instead of Entry API
// ❌ Trying to index String directly: s[0] (use .chars().nth(0))
// ❌ Not understanding the difference between iter(), iter_mut(), into_iter()
// ❌ Collecting without type annotation when compiler can't infer
// ❌ Using clone() excessively instead of borrowing
// ❌ Forgetting that HashMap iteration order is not guaranteed

// ============================================================================
// ZERO-COST ABSTRACTIONS EXPLAINED
// ============================================================================
//
// "Zero-cost" means:
// 1. The high-level abstraction (iterator chains) compiles to the SAME assembly
//    as low-level code (manual loops)
// 2. No runtime overhead - no function pointers, no dynamic dispatch
// 3. Compiler can often optimize iterator chains BETTER than manual loops!
//
// How it works:
// - Iterators use static dispatch (monomorphization)
// - All iterator adaptors inline into a single function
// - LLVM optimizer sees the whole chain and can optimize aggressively
// - Result: Fast code that's also readable and composable
//
// Example:
//   data.iter().filter(p).map(f).sum()
//
// Gets monomorphized and inlined to essentially:
//   let mut sum = 0;
//   for x in &data {
//       if p(x) {
//           sum += f(x);
//       }
//   }
//
// This is the SAME efficiency as hand-written loops, but more composable!

// ============================================================================
// COMPARISON: RUST vs PYTHON vs GO
// ============================================================================
//
// Python:
//   - List comprehensions: [x*2 for x in data if x % 2 == 0]
//   - Very readable, but slower (interpreted)
//   - Generators are lazy like Rust iterators
//   - No compile-time optimization
//
// Go:
//   - No iterator abstraction in standard library
//   - Must use manual loops
//   - More verbose, but explicit
//   - Some third-party iterator libraries exist
//
// Rust:
//   - Iterator chains: data.iter().filter(...).map(...).collect()
//   - Readable AND fast (zero-cost)
//   - Compile-time optimization
//   - Best of both worlds: high-level + high-performance
