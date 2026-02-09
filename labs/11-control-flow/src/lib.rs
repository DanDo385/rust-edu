//! # Lab 11 - Control Flow and Pattern Matching
//!
//! This lab teaches you how Rust handles decision-making and looping.
//! Unlike other languages, Rust's control flow is more consistent and safer.
//!
//! You'll learn:
//! - **if/else expressions** that return values (not just statements)
//! - **Loops** (loop, while, for) and how to choose the right one
//! - **match expressions** with pattern matching and guards
//! - **Functions** with parameters, return types, and early returns
//!
//! By the end, you'll understand why Rust's approach is safer than languages
//! like JavaScript or Python, and how it prevents common bugs.
//!
//! ## Your Task
//!
//! Implement the functions below. Each function has:
//! - Documentation explaining what it should do
//! - A `todo!()` macro that panics when called (replace with your implementation!)
//! - Hints in comments to guide you
//!
//! ## Running Your Code
//!
//! ```bash
//! # Run tests to check your implementation
//! cargo test -p control-flow
//!
//! # Run the main binary to see it in action
//! cargo run -p control-flow
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.
//! Don't just copy - read and understand! Every line is explained.

use std::cmp::Ordering;

/// Classifies a number using match expressions and guards.
///
/// This function teaches you about:
/// - Match expressions (Rust's pattern matching)
/// - Pattern guards (adding conditions with `if`)
/// - Range patterns (1..=10, etc.)
/// - The catch-all pattern (`_`)
///
/// # Parameters
/// - `n: i32` - The number to classify
///
/// # Returns
/// A string slice (`&'static str`) describing the number's category.
/// Static lifetime means the string lives as long as the program runs.
///
/// # Example
/// ```ignore
/// use control_flow::classify_number;
/// assert_eq!(classify_number(5), "small");
/// assert_eq!(classify_number(0), "zero");
/// assert_eq!(classify_number(-10), "negative");
/// ```
///
/// # Categories
/// - "zero" if n == 0
/// - "negative" if n < 0
/// - "small" if 1 <= n <= 10
/// - "medium" if 11 <= n <= 100
/// - "large" if n > 100
pub fn classify_number(n: i32) -> &'static str {
    // TODO: Classify n with exhaustive match arms and guards.
    // Hint: 0 => "zero", n if n < 0 => "negative", 1..=10 => "small", 11..=100 => "medium".
    let _ = n;
    todo!("Classify n using match patterns and a catch-all arm")
}

/// Determines whether a guess is too small, too big, or correct.
///
/// This function teaches you about:
/// - The `cmp()` method that returns an `Ordering`
/// - Using match on enum values
/// - Early returns with match arms
///
/// # Parameters
/// - `guess: i32` - The player's guess
/// - `secret: i32` - The secret number
///
/// # Returns
/// An `Ordering` enum:
/// - `Less` if guess < secret (too small)
/// - `Greater` if guess > secret (too big)
/// - `Equal` if guess == secret (correct!)
///
/// # Example
/// ```ignore
/// use control_flow::compare_guess;
/// use std::cmp::Ordering;
/// assert_eq!(compare_guess(5, 10), Ordering::Less);
/// assert_eq!(compare_guess(10, 5), Ordering::Greater);
/// assert_eq!(compare_guess(5, 5), Ordering::Equal);
/// ```
pub fn compare_guess(guess: i32, secret: i32) -> Ordering {
    // TODO: Return Less/Equal/Greater based on guess vs secret.
    // Hint: `cmp` already returns `Ordering`.
    let _ = (guess, secret);
    todo!("Compare guess and secret with cmp")
}

/// Describes a number in different categories.
///
/// This function teaches you about:
/// - If/else expressions (not statements!)
/// - Early returns in functions
/// - Type inference in if/else branches
///
/// # Parameters
/// - `n: i32` - The number to describe
///
/// # Returns
/// A string slice describing the number:
/// - "zero" if n == 0
/// - "one" if n == 1
/// - "even" if n > 1 and even
/// - "odd" if n > 1 and odd
/// - "negative" if n < 0
///
/// # Example
/// ```ignore
/// use control_flow::describe_number;
/// assert_eq!(describe_number(0), "zero");
/// assert_eq!(describe_number(4), "even");
/// assert_eq!(describe_number(3), "odd");
/// ```
pub fn describe_number(n: i32) -> &'static str {
    // TODO: Return "negative", "zero", "one", "even", or "odd".
    // Hint: Use modulo (`% 2`) to distinguish even from odd.
    let _ = n;
    todo!("Describe number categories with if/else")
}

/// Validates and parses a guess from a string.
///
/// This function teaches you about:
/// - The `Result` type for fallible operations
/// - The `match` expression for handling Results
/// - String parsing and error handling
///
/// # Parameters
/// - `input: &str` - A string that might be a number (e.g., "42" or "hello")
///
/// # Returns
/// `Result<i32, String>`:
/// - `Ok(number)` if the string is a valid integer
/// - `Err(message)` if the string is invalid, with an error message
///
/// # Example
/// ```ignore
/// use control_flow::validate_guess;
/// assert_eq!(validate_guess("42"), Ok(42));
/// assert!(validate_guess("hello").is_err());
/// ```
///
/// # Hint
/// Use this pattern:
/// ```rust,ignore
/// match input.trim().parse::<i32>() {
///     Ok(num) => Ok(num),
///     Err(_) => Err("Please enter a valid number".to_string()),
/// }
/// ```
pub fn validate_guess(input: &str) -> Result<i32, String> {
    // TODO: Trim, parse, and validate range 1..=100.
    // Hint: Return `Err(String)` for parse failures and out-of-range values.
    let _ = input;
    todo!("Validate user input into a bounded integer guess")
}

/// Counts how many times you can divide a number by 2 before it becomes odd.
///
/// This function teaches you about:
/// - While loops with conditions
/// - Mutable variables
/// - Loop counters
///
/// # Parameters
/// - `n: i32` - The number to analyze
///
/// # Returns
/// The count of how many times n can be divided by 2.
///
/// # Example
/// ```ignore
/// use control_flow::count_divisions;
/// assert_eq!(count_divisions(8), 3);   // 8 / 2 / 2 / 2 = 1 (3 divisions)
/// assert_eq!(count_divisions(5), 0);   // 5 is odd, can't divide (0 divisions)
/// assert_eq!(count_divisions(16), 4);  // 16 / 2 / 2 / 2 / 2 = 1 (4 divisions)
/// ```
pub fn count_divisions(mut n: i32) -> u32 {
    // TODO: Count repeated divisions by 2 until n is odd.
    // Hint: Use a `while n % 2 == 0` loop and increment a counter.
    let _ = &mut n;
    todo!("Count powers of two in n")
}

/// Sums all numbers in a range using a for loop.
///
/// This function teaches you about:
/// - For loops with ranges
/// - The .. (exclusive) vs ..= (inclusive) operators
/// - Accumulating values in loops
///
/// # Parameters
/// - `start: i32` - First number (inclusive)
/// - `end: i32` - Last number (inclusive, unlike ranges!)
///
/// # Returns
/// The sum of all integers from start to end (both inclusive).
///
/// # Example
/// ```ignore
/// use control_flow::sum_range;
/// assert_eq!(sum_range(1, 5), 15);     // 1+2+3+4+5 = 15
/// assert_eq!(sum_range(0, 10), 55);    // 0+1+2+...+10 = 55
/// assert_eq!(sum_range(5, 5), 5);      // Just 5
/// ```
///
/// # Hint
/// Use `start..=end` to create an inclusive range you can iterate over.
pub fn sum_range(start: i32, end: i32) -> i32 {
    // TODO: Sum all integers from start through end (inclusive).
    // Hint: `for i in start..=end { ... }`.
    let _ = (start, end);
    todo!("Compute inclusive range sum")
}

/// Decides the next action based on user input and game state.
///
/// This function teaches you about:
/// - Complex match expressions
/// - Multiple patterns (using |)
/// - Handling impossible cases with unreachable!()
///
/// # Parameters
/// - `input: &str` - User command: "continue", "quit", or anything else
/// - `game_won: bool` - Whether the player has won
///
/// # Returns
/// A string describing what to do:
/// - "continuing game" if input is "continue" and game isn't won
/// - "exiting game" if input is "quit" or game_won is true
/// - "invalid command, please try again" for anything else
///
/// # Example
/// ```ignore
/// use control_flow::decide_action;
/// assert_eq!(decide_action("continue", false), "continuing game");
/// assert_eq!(decide_action("quit", false), "exiting game");
/// assert_eq!(decide_action("continue", true), "exiting game");
/// ```
pub fn decide_action(input: &str, game_won: bool) -> &'static str {
    // TODO: Use tuple matching on (input, game_won) to choose an action.
    // Hint: prioritize exiting when input == "quit" or game_won is true.
    let _ = (input, game_won);
    todo!("Decide action from command and game state")
}

pub mod solution;
