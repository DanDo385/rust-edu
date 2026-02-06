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
    // **From the borrow checker's perspective:**
    // - n: i32 is passed by value (small type, gets copied)
    // - Ownership is not relevant here (i32 is Copy)
    // - No references, no borrowing needed

    match n {
        0 => "zero",
        n if n < 0 => "negative",      // Pattern guard: condition on n
        1..=10 => "small",              // Range: includes 1 through 10
        11..=100 => "medium",           // Range: includes 11 through 100
        _ => "large",                   // Catch-all: all remaining values
    }
    // **What Rust prevents here:**
    // If we forgot the `_` arm, Rust would ERROR: "match is not exhaustive"
    // This forces us to handle ALL possible values.
    // In C/C++, a missed case would silently return garbage.
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
    // **From the borrow checker's perspective:**
    // - Both guess and secret are i32 (Copy types, passed by value)
    // - The & in cmp(&secret) is an immutable borrow of the parameter
    // - Safe because we only READ secret, never MODIFY it
    // - After this function, the caller still owns both values (unchanged)
    guess.cmp(&secret)
    // **Why use cmp() instead of if/else?**
    // - cmp() returns Ordering directly - our return type
    // - Shorter and clearer intent: "compare these values"
    // - Less chance for bugs (can't accidentally forget a case)
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
    // **From the borrow checker's perspective:**
    // - n: i32 is passed by value (copied, it's small)
    // - No references, no borrowing
    // - Each branch returns a &'static str (constant data in binary)

    if n == 0 {
        "zero"
    } else if n == 1 {
        "one"
    } else if n < 0 {
        "negative"
    } else if n % 2 == 0 {
        // n % 2: Modulo operator gets the remainder
        // If remainder is 0, the number is even
        "even"
    } else {
        // If we reach here: n > 1 and n is not divisible by 2
        "odd"
    }
    // **Why if/else vs match?**
    // - if/else works well for simple binary conditions
    // - match is better for complex patterns or exhaustive cases
    // - Both compile to the same efficient code
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
    // **From the borrow checker's perspective:**
    // - input: &str is borrowed (we don't own it)
    // - trim() returns another &str (still borrowed from input)
    // - parse() is a method on &str that returns Result<i32, ParseIntError>
    // - We transform the error into our own String message

    let trimmed = input.trim();  // Borrowed reference to trimmed string

    match trimmed.parse::<i32>() {
        Ok(num) => {
            // Success! We have a number.
            // Check bounds: valid range is 1-100
            if num < 1 || num > 100 {
                Err(format!("Guess must be between 1 and 100, got {}", num))
            } else {
                Ok(num)
            }
        }
        Err(_) => {
            // Parse failed. We ignore the ParseIntError and create our own message.
            // The _ means "we don't care about the error details"
            Err(format!("'{}' is not a valid number", trimmed))
        }
    }
    // **What Rust prevents here:**
    // - If you ignore the Result without handling both Ok/Err, the compiler errors!
    // - In languages like JavaScript, parse errors often silently return NaN
    // - Rust forces explicit error handling - safer code!
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
    // **From the borrow checker's perspective:**
    // - n: i32 is passed by value (copied, caller still has their copy)
    // - mut n means we can MODIFY our local copy
    // - count is a mutable variable (mutable binding)
    // - When function ends, our copy of n is dropped (no cleanup needed for i32)

    let mut count = 0u32;  // Mutable variable to track divisions

    // While n is divisible by 2 (is even)
    while n % 2 == 0 {
        n /= 2;        // Divide n by 2
        count += 1;    // Increment counter
    }

    count  // Return the count of divisions

    // **Why mut here?**
    // - n needs to change: n /= 2 requires mut
    // - count needs to change: count += 1 requires mut
    // - Rust makes this explicit (unlike languages where all variables are mutable by default)
    //
    // **Why pass by value and then mutate?**
    // - We only need to modify our local copy
    // - Caller doesn't care about the modified value (we return count, not n)
    // - This is safer than using &mut (which would require the caller to give up control)
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
    // **From the borrow checker's perspective:**
    // - start, end: i32 are passed by value (copied, caller keeps theirs)
    // - sum: i32 is a mutable local variable
    // - The for loop binds i to each value (immutable by default)
    // - When function ends, all local variables are dropped

    let mut sum = 0;

    // start..=end is an INCLUSIVE range (includes both start and end)
    // Examples:
    //   1..=5   = [1, 2, 3, 4, 5]
    //   5..=5   = [5]
    //   5..=4   = [] (empty range, loop doesn't run)
    for i in start..=end {
        sum += i;  // Add each value to the accumulator
    }

    sum  // Return the accumulated sum

    // **Why for loop on ranges?**
    // - Idiomatic Rust for iterating over known sequences
    // - Can't accidentally skip or repeat elements
    // - No manual index management (no off-by-one errors)
    //
    // **What if start > end?**
    // - The range is empty, the loop never runs
    // - sum remains 0 (the neutral element for addition)
    // - No error, no undefined behavior - just the expected result!
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
    // **From the borrow checker's perspective:**
    // - input: &str is borrowed (read-only reference)
    // - game_won: bool is passed by value (1 byte, trivial to copy)
    // - We're only READING these, never MODIFYING them
    // - After this function, caller still owns and can use both values

    match (input, game_won) {
        // Tuple pattern: match on both values at once
        ("quit", _) => "exiting game",        // Quit always means exit (ignore game_won with _)
        (_, true) => "exiting game",          // If game_won, exit regardless of input
        ("continue", false) => "continuing game",  // Continue only if haven't won
        _ => "invalid command, please try again",  // Everything else is invalid
    }

    // **Why match (input, game_won)?**
    // - We're matching on TWO conditions simultaneously
    // - Tuple patterns let us handle all combinations clearly
    // - More readable than nested if/else statements
    //
    // **What about the underscore (_)?**
    // - _ means "I don't care about this value"
    // - In ("quit", _): We accept any game_won value when input is "quit"
    // - In (_, true): We accept any input when game_won is true
    // - Safety: Compiler ensures we handle ALL possible (input, game_won) pairs
}

pub mod solution;
