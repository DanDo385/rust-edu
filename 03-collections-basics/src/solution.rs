//! # Collections Basics - Complete Solution with EXHAUSTIVE Explanations
//!
//! ## What We're Building
//!
//! This module teaches Rust's collection types: Vec<T> and HashMap<K, V>.
//! Collections store multiple values and are essential for real programs.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **Memory efficiency**: Collections grow dynamically without waste
//! - **Zero-cost abstractions**: Iterators compile to efficient machine code
//! - **Type safety**: Can't mix types in collections
//! - **Ownership**: Collections own their data, preventing dangling pointers

use std::collections::HashMap;

/// Sums all even numbers in a slice.
///
/// ## Parameters
/// - `numbers: &[i32]` - Borrowed slice of integers
///
/// ## Returns
/// Sum of all even numbers as i32
///
/// ## Example
/// ```ignore
/// let nums = vec![1, 2, 3, 4, 5, 6];
/// let sum = sum_of_evens(&nums);
/// assert_eq!(sum, 12);  // 2 + 4 + 6 = 12
/// ```ignore
pub fn sum_of_evens(numbers: &[i32]) -> i32 {
    // ========================================================================
    // USING ITERATOR METHODS - FUNCTIONAL STYLE
    // ========================================================================

    // `numbers.iter()` = create an iterator over the slice
    //   - Iterator yields references to elements (&i32)
    //   - Zero-cost abstraction: compiles to efficient loop
    //   - Can chain multiple operations
    //
    // `.filter(|&&n| n % 2 == 0)` = keep only even numbers
    //   - `filter` takes a closure that returns bool
    //   - `|&&n|` = closure parameter pattern
    //     - First `&` from iter() giving us &i32
    //     - Second `&` to destructure and get i32 value
    //     - `n` is now i32 (not &i32)
    //   - `n % 2 == 0` = check if even (remainder of division by 2 is 0)
    //   - Only elements where closure returns true pass through
    //
    // `.sum()` = add up all remaining elements
    //   - Consumes the iterator
    //   - Returns the sum as i32
    //   - Type inference determines return type from function signature
    //
    // This entire chain compiles to efficient machine code!
    // - No heap allocations for intermediate results
    // - Compiles to same assembly as hand-written loop
    // - But more readable and can't have off-by-one errors

    numbers.iter().filter(|&&n| n % 2 == 0).sum()

    // ============================================================================
    // ALTERNATIVE APPROACHES
    // ============================================================================
    //
    // 1. Explicit loop (more verbose but clearer for beginners):
    // ```rust
    // let mut sum = 0;
    // for &num in numbers.iter() {
    //     if num % 2 == 0 {
    //         sum += num;
    //     }
    // }
    // sum
    // ```
    //
    // 2. Using fold (most general):
    // ```rust
    // numbers.iter().fold(0, |acc, &n| if n % 2 == 0 { acc + n } else { acc })
    // ```
    //
    // 3. Filter then sum separately:
    // ```rust
    // numbers.iter().filter(|&&n| n % 2 == 0).copied().sum()
    // ```
    //
    // The approach we used is most idiomatic!
}

/// Counts how many times each word appears in text.
///
/// ## Parameters
/// - `text: &str` - Input text to analyze
///
/// ## Returns
/// HashMap where keys are words and values are counts
///
/// ## Example
/// ```ignore
/// let freq = word_frequency("hello world hello");
/// assert_eq!(freq.get("hello"), Some(&2));
/// assert_eq!(freq.get("world"), Some(&1));
/// ```ignore
pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    // ========================================================================
    // STEP 1: CREATE EMPTY HASHMAP
    // ========================================================================

    // `HashMap::new()` = create empty HashMap
    //   - `HashMap` = type name
    //   - `::` = path separator for associated functions (like static methods)
    //   - `new()` = constructor function
    //   - Returns HashMap<String, usize> (inferred from function return type)
    //   - Initially empty, no heap allocation until first insert
    //
    // What is a HashMap?
    // - Hash table data structure
    // - Stores key-value pairs
    // - O(1) average time for insert/lookup
    // - Keys must implement Hash + Eq traits
    // - Ownership: HashMap OWNS both keys and values

    let mut map = HashMap::new();

    // ========================================================================
    // STEP 2: SPLIT TEXT INTO WORDS AND COUNT THEM
    // ========================================================================

    // `text.split_whitespace()` = split on whitespace characters
    //   - Returns iterator over string slices (&str)
    //   - Splits on spaces, tabs, newlines, etc.
    //   - Consecutive whitespace treated as single separator
    //   - Does not allocate—yields borrows of original string
    //
    // `for word in ...` = iterate over each word
    //   - `word` is &str (borrowed from original text)
    //   - Each iteration processes one word

    for word in text.split_whitespace() {
        // ====================================================================
        // CONVERT WORD TO LOWERCASE
        // ====================================================================

        // `word.to_lowercase()` = create lowercase version
        //   - Returns a new String (owned)
        //   - Handles Unicode properly
        //   - Necessary because HashMap keys are String (owned)
        //
        // Why lowercase?
        // - Make counting case-insensitive
        // - "Hello" and "hello" count as same word

        let word_lower = word.to_lowercase();

        // ====================================================================
        // UPDATE COUNT IN HASHMAP - THE ENTRY API
        // ====================================================================

        // `map.entry(word_lower)` = get entry for this key
        //   - Returns an Entry enum (either Occupied or Vacant)
        //   - Consumes word_lower (moves ownership to HashMap if inserted)
        //   - Doesn't clone if key already exists
        //   - This is the most efficient way to insert or update
        //
        // `.or_insert(0)` = insert 0 if key doesn't exist
        //   - If key is missing (Vacant), inserts 0 and returns &mut to it
        //   - If key exists (Occupied), returns &mut to existing value
        //   - Returns &mut usize (mutable reference to the value)
        //
        // `*` = dereference operator
        //   - We have &mut usize, need to get to the usize to increment it
        //   - `*` follows the reference to the actual value
        //
        // `+= 1` = increment the count
        //   - Adds 1 to the value
        //   - If this was first occurrence, increments 0 to 1
        //   - If not first, increments existing count
        //
        // This pattern is VERY common with HashMaps!
        // Example execution:
        // - First "hello": entry doesn't exist, inserts 0, increments to 1
        // - Second "hello": entry exists with 1, increments to 2

        *map.entry(word_lower).or_insert(0) += 1;

        // Alternative approaches:
        //
        // 1. Manual check (more verbose):
        // ```rust
        // if let Some(count) = map.get_mut(&word_lower) {
        //     *count += 1;
        // } else {
        //     map.insert(word_lower, 1);
        // }
        // ```
        //
        // 2. Using match (even more verbose):
        // ```rust
        // match map.get_mut(&word_lower) {
        //     Some(count) => *count += 1,
        //     None => { map.insert(word_lower, 1); }
        // }
        // ```
        //
        // The entry API is most efficient and idiomatic!
    }

    // ========================================================================
    // STEP 3: RETURN THE HASHMAP
    // ========================================================================

    // Return the completed HashMap
    // - Ownership transfers to caller
    // - Caller can query, iterate, or modify it
    // - When caller drops it, all keys and values are freed

    map

    // ============================================================================
    // HASHMAP MEMORY LAYOUT
    // ============================================================================
    //
    // Stack (the HashMap itself):
    // ┌─────────────────────────┐
    // │ map: HashMap<String, usize> │
    // │  - hash_table_ptr: 0x1000│──┐
    // │  - capacity: 8           │  │
    // │  - len: 3                │  │
    // └─────────────────────────┘  │
    //                               │
    // Heap (hash table + entries): ▼
    // ┌──────────────────────────────────────┐
    // │ Hash Table (array of buckets)        │
    // ├──────────────────────────────────────┤
    // │ Bucket 0: empty                      │
    // │ Bucket 1: ("hello", 2) ───────────┐  │
    // │ Bucket 2: empty                   │  │
    // │ Bucket 3: ("world", 1) ─────────┐ │  │
    // │ Bucket 4: ("rust", 1) ────────┐ │ │  │
    // │ ...                           │ │ │  │
    // └───────────────────────────────┼─┼─┼──┘
    //                                 │ │ │
    //         Each entry owns:        │ │ │
    //         ┌──────────────┐       │ │ │
    //         │ String "rust"│◀──────┘ │ │
    //         │ usize 1      │         │ │
    //         └──────────────┘         │ │
    //         ┌──────────────┐         │ │
    //         │String "world"│◀────────┘ │
    //         │ usize 1      │           │
    //         └──────────────┘           │
    //         ┌──────────────┐           │
    //         │String "hello"│◀──────────┘
    //         │ usize 2      │
    //         └──────────────┘
    //
    // When map is dropped:
    // 1. Iterates through all buckets
    // 2. Drops each String (frees its heap buffer)
    // 3. Frees the hash table array
    // All automatic—no manual memory management needed!
}

/// Filters numbers in range [min, max] and returns them sorted.
///
/// ## Parameters
/// - `numbers: &[i32]` - Input numbers
/// - `min: i32` - Minimum value (inclusive)
/// - `max: i32` - Maximum value (inclusive)
///
/// ## Returns
/// New Vec with filtered and sorted numbers
///
/// ## Example
/// ```ignore
/// let nums = vec![5, 2, 8, 1, 9, 3];
/// let result = filter_and_sort(&nums, 2, 5);
/// assert_eq!(result, vec![2, 3, 5]);
/// ```ignore
pub fn filter_and_sort(numbers: &[i32], min: i32, max: i32) -> Vec<i32> {
    // ========================================================================
    // STEP 1: FILTER NUMBERS IN RANGE
    // ========================================================================

    // `numbers.iter()` = create iterator
    //   - Yields &i32 (references to elements)
    //
    // `.filter(|&&n| n >= min && n <= max)` = keep only numbers in range
    //   - `|&&n|` = closure with destructuring pattern
    //     - First & from iter() yielding &i32
    //     - Second & to destructure, leaving i32
    //     - n is now i32 value
    //   - `n >= min && n <= max` = check if in range [min, max]
    //   - `&&` = logical AND (both conditions must be true)
    //   - Only numbers in range pass through
    //
    // `.copied()` = convert &i32 to i32
    //   - Iterator currently yields &i32
    //   - .copied() calls .clone() on each element
    //   - For i32 (Copy type), this just copies the value
    //   - Now iterator yields i32 (owned values)
    //   - Necessary for .collect() to create Vec<i32>
    //
    // `.collect::<Vec<i32>>()` = collect into Vec
    //   - Consumes iterator
    //   - Creates new Vec on heap
    //   - Populates with all filtered values
    //   - Type annotation `::<Vec<i32>>` tells collect what to create
    //   - Could also write: `.collect()` and let type inference figure it out

    let mut result: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n >= min && n <= max)
        .copied()
        .collect();

    // ========================================================================
    // STEP 2: SORT THE RESULT
    // ========================================================================

    // `result.sort()` = sort in place
    //   - Sorts the Vec (ascending order)
    //   - Modifies the Vec in place (no new allocation)
    //   - Uses comparison based sort (requires Ord trait)
    //   - i32 implements Ord, so this works
    //   - Time complexity: O(n log n)
    //   - Uses optimized sorting algorithm (TimSort variant)
    //   - Stable sort (equal elements keep relative order)
    //
    // Why sort after collect?
    // - collect() doesn't guarantee any order
    // - Sorting on Vec is more efficient than during iteration
    // - sort() does in-place sorting (no extra allocation)

    result.sort();

    // ========================================================================
    // STEP 3: RETURN THE RESULT
    // ========================================================================

    // Return the sorted, filtered Vec
    // - Ownership transfers to caller
    // - Caller owns the heap-allocated Vec
    // - When caller drops it, memory is freed

    result

    // ============================================================================
    // VEC MEMORY LAYOUT
    // ============================================================================
    //
    // Stack (the Vec itself):
    // ┌──────────────────┐
    // │ result: Vec<i32> │
    // │  - ptr: 0x2000   │──────┐
    // │  - len: 3        │      │
    // │  - cap: 4        │      │
    // └──────────────────┘      │
    //                           │
    // Heap (the actual data):   ▼
    // ┌────┬────┬────┬────┐
    // │ 2  │ 3  │ 5  │ ?? │
    // └────┴────┴────┴────┘
    //  used→←─────→  unused
    //      len=3    cap=4
    //
    // - ptr: points to heap allocation
    // - len: number of elements currently used
    // - cap: total capacity allocated
    // - Extra capacity allows growing without reallocation
    // - All 3 fields fit in 24 bytes on stack (64-bit system)

    // ============================================================================
    // ALTERNATIVE APPROACHES
    // ============================================================================
    //
    // 1. Sort first, then filter (less efficient):
    // ```rust
    // let mut all: Vec<i32> = numbers.iter().copied().collect();
    // all.sort();
    // all.into_iter().filter(|&n| n >= min && n <= max).collect()
    // ```
    //
    // 2. Manual loop (more verbose):
    // ```rust
    // let mut result = Vec::new();
    // for &n in numbers.iter() {
    //     if n >= min && n <= max {
    //         result.push(n);
    //     }
    // }
    // result.sort();
    // result
    // ```
    //
    // 3. Using retain (if we owned the Vec):
    // ```rust
    // let mut result = numbers.to_vec();
    // result.retain(|&n| n >= min && n <= max);
    // result.sort();
    // result
    // ```
    //
    // Our approach is most idiomatic and efficient!
}

/// Finds the most frequently occurring word in text.
///
/// ## Parameters
/// - `text: &str` - Input text to analyze
///
/// ## Returns
/// - Some(word) - The most common word
/// - None - If text is empty
///
/// ## Example
/// ```ignore
/// let text = "the cat and the dog and the bird";
/// let result = most_common_word(text);
/// assert_eq!(result, Some("the".to_string()));
/// ```ignore
pub fn most_common_word(text: &str) -> Option<String> {
    // ========================================================================
    // STEP 1: GET WORD FREQUENCIES
    // ========================================================================

    // Reuse our word_frequency function!
    // - DRY principle (Don't Repeat Yourself)
    // - word_frequency already handles the counting logic
    // - Returns HashMap<String, usize>

    let frequencies = word_frequency(text);

    // ========================================================================
    // STEP 2: FIND THE WORD WITH MAXIMUM COUNT
    // ========================================================================

    // `frequencies.into_iter()` = consume HashMap, iterate over entries
    //   - `into_iter()` takes ownership of HashMap
    //   - Yields (String, usize) tuples (owned key-value pairs)
    //   - Can't use HashMap after this (it was consumed)
    //   - Why into_iter? We need to return owned String, not reference
    //
    // `.max_by_key(|(_word, count)| *count)` = find entry with max count
    //   - `max_by_key` finds element where key function returns largest value
    //   - `|(_word, count)|` = closure parameter
    //     - Takes tuple (String, usize)
    //     - `_word` = we don't need the word here, just the count
    //     - Leading `_` tells Rust we intentionally don't use it
    //     - `count` = the frequency count (reference to usize)
    //   - `*count` = dereference count to get usize value
    //   - Compares all counts, returns entry with largest count
    //   - Returns Option<(String, usize)>
    //     - Some((word, count)) if HashMap not empty
    //     - None if HashMap was empty
    //
    // `.map(|(word, _count)| word)` = extract just the word
    //   - `map` transforms Option<(String, usize)> to Option<String>
    //   - `|(word, _count)|` = destructure tuple in closure
    //   - `word` = the String we want to return
    //   - `_count` = ignore the count
    //   - Returns just the word, discarding count
    //   - If max_by_key returned None, map does nothing (None stays None)
    //   - If max_by_key returned Some, map extracts the word
    //
    // Final result: Option<String>
    // - Some(word) if text had at least one word
    // - None if text was empty or only whitespace

    frequencies
        .into_iter()
        .max_by_key(|(_word, count)| *count)
        .map(|(word, _count)| word)

    // ============================================================================
    // UNDERSTANDING THE PIPELINE
    // ============================================================================
    //
    // Let's trace execution with example: "the cat and the dog"
    //
    // 1. word_frequency("the cat and the dog") returns:
    //    {"the": 2, "cat": 1, "and": 1, "dog": 1}
    //
    // 2. into_iter() converts to iterator yielding:
    //    ("the", 2), ("cat", 1), ("and", 1), ("dog", 1)
    //    (order undefined—HashMap doesn't guarantee order)
    //
    // 3. max_by_key finds entry with max count:
    //    - Compares counts: 2, 1, 1, 1
    //    - Maximum is 2
    //    - Returns Some(("the", 2))
    //
    // 4. map extracts just the word:
    //    - Takes ("the", 2)
    //    - Returns "the"
    //    - Result: Some("the")
    //
    // If text was empty:
    // 1. word_frequency("") returns empty HashMap: {}
    // 2. into_iter() yields nothing
    // 3. max_by_key returns None (no elements to compare)
    // 4. map doesn't run (None stays None)
    // 5. Result: None

    // ============================================================================
    // ALTERNATIVE APPROACHES
    // ============================================================================
    //
    // 1. Using fold (more manual):
    // ```rust
    // frequencies.into_iter().fold(None, |max, (word, count)| {
    //     match max {
    //         None => Some((word, count)),
    //         Some((_, max_count)) if count > max_count => Some((word, count)),
    //         _ => max,
    //     }
    // }).map(|(word, _)| word)
    // ```
    //
    // 2. Using iter() and clone (less efficient):
    // ```rust
    // frequencies.iter()
    //     .max_by_key(|(_, count)| *count)
    //     .map(|(word, _)| word.clone())
    // ```
    // This would return Option<String> but requires cloning
    //
    // 3. Manual loop (most verbose):
    // ```rust
    // let mut max_word = None;
    // let mut max_count = 0;
    // for (word, count) in frequencies {
    //     if count > max_count {
    //         max_count = count;
    //         max_word = Some(word);
    //     }
    // }
    // max_word
    // ```
    //
    // Our approach is most concise and idiomatic!

    // ============================================================================
    // TIME AND SPACE COMPLEXITY
    // ============================================================================
    //
    // Time Complexity:
    // - word_frequency: O(n) where n = number of characters
    //   - Split and iterate through words: O(n)
    //   - HashMap operations: O(1) average per word
    // - into_iter().max_by_key(): O(m) where m = number of unique words
    //   - Must check all entries to find maximum
    // - Total: O(n + m), typically O(n) since m ≤ number of words
    //
    // Space Complexity:
    // - HashMap: O(m) where m = number of unique words
    // - Each entry stores String + usize
    // - Returned String: O(k) where k = length of most common word
    // - Total: O(m)
    //
    // This is optimal for this problem!
}
