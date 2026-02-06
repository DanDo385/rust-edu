//! # Variables and Types - Your First Rust Code
//!
//! This project teaches you the absolute basics of Rust:
//! - How to declare variables with `let`
//! - When to use `mut` (mutable)
//! - Different number types (i32, u32, f64)
//! - The difference between String and &str
//!
//! ## Your Task
//!
//! Implement the four functions below. Each function has:
//! - Documentation explaining what it should do
//! - A `todo!()` macro that panics when called
//!
//! Replace the `todo!()` with your implementation!
//!
//! ## Running Your Code
//!
//! ```bash
//! # Run tests to check your implementation
//! cargo test -p variables-types
//!
//! # Run the main binary to see it in action
//! cargo run -p variables-types
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

/// Creates a greeting message by combining first and last name.
///
/// This function teaches you about:
/// - String concatenation in Rust
/// - The `format!` macro (like printf or string formatting)
/// - How to take borrowed string slices (&str) as parameters
///
/// # Parameters
/// - `first`: A borrowed string slice - we don't need to own it, just read it
/// - `last`: A borrowed string slice - we don't need to own it, just read it
///
/// # Returns
/// An owned String that the caller can keep
///
/// # Example
/// ```ignore
/// use variables_types::make_greeting;
/// let greeting = make_greeting("Alice", "Smith");
/// assert_eq!(greeting, "Hello, Alice Smith!");
/// ```ignore
pub fn make_greeting(first: &str, last: &str) -> String {
    // TODO: Combine first and last into a greeting like "Hello, [first] [last]!"
    // Hint: Use the format! macro
    // Hint: format!("Hello, {} {}!", first, last)
    todo!("Create a greeting like 'Hello, [first] [last]!'")
}

/// Converts Celsius to Fahrenheit.
///
/// This teaches you about:
/// - f64 type (64-bit floating point number)
/// - Basic arithmetic in Rust
/// - Function return values
///
/// Formula: F = (C × 9/5) + 32
///
/// # Parameters
/// - `celsius`: Temperature in Celsius (can be negative or decimal)
///
/// # Returns
/// Temperature in Fahrenheit as f64
///
/// # Example
/// ```ignore
/// use variables_types::celsius_to_fahrenheit;
/// assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```ignore
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // TODO: Convert celsius to fahrenheit using the formula: F = (C * 9/5) + 32
    // Hint: Remember to use .0 for floating point literals (9.0 not 9)
    todo!("Convert celsius to fahrenheit")
}

/// Finds the largest number in a slice of integers.
///
/// This teaches you about:
/// - Slices (&[i32]) - borrowed views into arrays/vectors
/// - Option type - maybe has a value, maybe doesn't
/// - Pattern matching with match
/// - Iteration
///
/// # Parameters
/// - `numbers`: A borrowed slice of 32-bit integers
///
/// # Returns
/// - Some(largest) if the slice has at least one number
/// - None if the slice is empty
///
/// # Example
/// ```ignore
/// use variables_types::find_largest;
/// assert_eq!(find_largest(&[3, 7, 2, 9, 1]), Some(9));
/// assert_eq!(find_largest(&[]), None);
/// assert_eq!(find_largest(&[-5, -1, -10]), Some(-1));
/// ```ignore
pub fn find_largest(numbers: &[i32]) -> Option<i32> {
    // TODO: Find the largest number, or return None if empty
    // Hint: Check if empty first: if numbers.is_empty() { return None; }
    // Hint: Start with the first number: let mut largest = numbers[0];
    // Hint: Loop through the rest: for &num in numbers.iter() { ... }
    // Hint: Return: Some(largest)
    todo!("Find the largest number, or None if empty")
}

/// Counts the number of vowels in a string.
///
/// This teaches you about:
/// - Iterating over characters in a string
/// - Boolean logic
/// - String methods
/// - Case-insensitive comparison
///
/// Vowels are: a, e, i, o, u (case-insensitive)
///
/// # Parameters
/// - `text`: A borrowed string slice to analyze
///
/// # Returns
/// The count of vowels as usize (unsigned size type)
///
/// # Example
/// ```ignore
/// use variables_types::count_vowels;
/// assert_eq!(count_vowels("hello"), 2);  // e, o
/// assert_eq!(count_vowels("AEIOU"), 5);  // All vowels
/// assert_eq!(count_vowels("xyz"), 0);    // No vowels
/// assert_eq!(count_vowels(""), 0);       // Empty string
/// ```ignore
pub fn count_vowels(text: &str) -> usize {
    // TODO: Count vowels (a, e, i, o, u) - case insensitive
    // Hint: Convert to lowercase: text.to_lowercase()
    // Hint: Iterate chars: for ch in text.chars() { ... }
    // Hint: Check if vowel: matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')
    // Hint: Count with a mutable counter variable
    todo!("Count vowels (a, e, i, o, u) - case insensitive")
}

// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
