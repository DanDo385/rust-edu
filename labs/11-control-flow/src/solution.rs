//! # Lab 11 - Control Flow and Pattern Matching - Complete Solution
//!
//! ## What We're Building
//!
//! This module contains six functions that teach you Rust's approach to control flow.
//! We're building the foundation for decision-making and iteration:
//! - Using match expressions (pattern matching) instead of switch statements
//! - Choosing between loop types (loop, while, for)
//! - Using if/else as expressions (not statements!)
//! - Handling Results and validation
//!
//! These concepts are fundamental to Rust programming - you'll use them in nearly
//! every program you write!
//!
//! ## Why Rust Is Perfect For This
//!
//! - **Exhaustive Pattern Matching**: The compiler FORCES you to handle all cases
//!   in a match expression. In JavaScript, you can accidentally forget a case in
//!   a switch statement (and it falls through!). In Rust, the compiler won't let
//!   you compile if you miss a case.
//!
//! - **No Truthy/Falsy Values**: JavaScript treats 0, "", null, undefined as "falsy".
//!   This causes bugs like `if (variable)` that breaks when variable is 0 or "".
//!   Rust requires explicit `if variable != 0` - no surprises!
//!
//! - **Expressions vs Statements**: In Rust, if/else ARE expressions - they return
//!   values! In most languages, you'd need a ternary operator. In Rust, you can do:
//!   `let result = if x { 5 } else { 6 };` Much cleaner!
//!
//! - **Explicit Error Handling**: The Result type forces you to think about failure.
//!   JavaScript just returns undefined or throws. Python just crashes. Rust makes
//!   you handle both success and failure cases.
//!
//! - **Compared to JavaScript**:
//!   ```javascript
//!   // JavaScript - confusing switch with fallthrough
//!   switch(x) {
//!       case 1:
//!           console.log("one");
//!           // FORGOT break! Falls through to case 2!
//!       case 2:
//!           console.log("two");
//!   }
//!   ```
//!   vs Rust (no fallthrough, exhaustive):
//!   ```rust
//!   match x {
//!       1 => println!("one"),
//!       2 => println!("two"),
//!       _ => println!("other"),  // Compiler REQUIRES this
//!   }
//!   ```
//!
//! - **Compared to Python**:
//!   ```python
//!   # Python - cryptic conditionals
//!   if items:  # Crashes if items is 0!
//!       process(items)
//!   ```
//!   vs Rust (explicit):
//!   ```rust
//!   if !items.is_empty() {  // Crystal clear what we're checking
//!       process(&items);
//!   }
//!   ```
//!
//! ## Key Rust Concepts You'll Learn
//!
//! - **Match Expressions**: Pattern matching with exhaustiveness checking
//! - **Pattern Guards**: Conditions in match arms (`if` clauses)
//! - **If/Else as Expressions**: They return values!
//! - **Result Type**: For operations that might fail
//! - **Loop Types**: loop, while, for - each with different purposes
//! - **Ordering Enum**: Used by comparison methods
//!
//! ## Time Complexity
//! - `classify_number`: O(1) - single comparison
//! - `compare_guess`: O(1) - single comparison
//! - `describe_number`: O(1) - single comparison
//! - `validate_guess`: O(n) where n = length of string (for parsing)
//! - `count_divisions`: O(log n) where n = input number
//! - `sum_range`: O(n) where n = number of integers in range
//!
//! ## Space Complexity
//! - All functions use O(1) space (just a few variables)

use std::cmp::Ordering;

/// Classifies a number using match expressions with pattern guards.
///
/// ## What This Function Does
///
/// Takes any integer and classifies it into one of five categories using
/// a match expression. This teaches you about match expressions and guards.
///
/// ## How Match Expressions Work
///
/// Match in Rust is like a switch statement on steroids:
/// 1. It's EXHAUSTIVE - you must handle all possible values (or use `_`)
/// 2. It's SAFE - no accidental fall-through like in JavaScript/C
/// 3. It returns a value - you can use it in assignments
/// 4. It supports pattern GUARDS - conditions in each arm
///
/// ```rust,ignore
/// match value {
///     pattern1 => result1,
///     pattern2 if condition => result2,  // Guard: only matches if condition is true
///     pattern3..=pattern4 => result3,    // Range patterns
///     _ => default_result,                // Catch-all - must exist or compiler errors!
/// }
/// ```
///
/// ## Parameters
///
/// - `n: i32` - Let's break this down:
///   - `n` = parameter name (you choose this)
///   - `:` = "has type" separator
///   - `i32` = 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
///
/// ## Returns
///
/// - `&'static str` - Let's break this down:
///   - `&` = borrow (we're not giving ownership, just a reference)
///   - `'static` = lifetime (string lives as long as the program runs)
///   - `str` = string slice (read-only text)
///
/// Why return `&'static str` instead of `String`?
/// - We're returning string literals ("zero", "negative", etc.)
/// - These are baked into the program binary, not allocated on heap
/// - They live forever (static lifetime)
/// - Much more efficient than allocating Strings!
///
/// ## Example
/// ```ignore
/// use control_flow::solution::classify_number;
/// assert_eq!(classify_number(0), "zero");
/// assert_eq!(classify_number(-5), "negative");
/// assert_eq!(classify_number(5), "small");
/// assert_eq!(classify_number(50), "medium");
/// assert_eq!(classify_number(200), "large");
/// ```
///
/// ## Pattern Matching Details
///
/// The match arms we use:
/// - `0` - matches exactly 0 (literal pattern)
/// - `n if n < 0` - matches negative (guard: condition after if)
/// - `1..=10` - matches 1 through 10 inclusive (range pattern)
/// - `11..=100` - matches 11 through 100 inclusive
/// - `_` - matches anything else (catch-all pattern)
///
/// **Key point**: The compiler REQUIRES you handle all cases!
/// If you forget one range, it won't compile. This catches bugs!
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameter `n` is OWNED (not borrowed)
///   - `n: i32` has no `&`, so we take ownership
///   - `i32` is Copy, so ownership doesn't really matter - it's copied
///   - We can modify n if we wanted (we don't need `mut` unless modifying)
///   - After this function, n disappears (but it was copied, not moved)
///
/// - Return value is BORROWED (`&'static str`)
///   - We return a reference to a string literal
///   - The literal lives in the binary forever (static lifetime)
///   - We're not allocating or owning anything - just pointing to existing data
///   - WHY: Much cheaper than allocating a String on the heap
///
/// ## Memory Layout
///
/// ```ignore
/// Stack (when function is called):
/// ┌──────────────────┐
/// │ n: i32           │  8-byte integer on stack
/// │ value: -42       │  (could be any i32)
/// └──────────────────┘
///
/// Return value (reference to string):
/// ┌──────────────────┐
/// │ &str             │────────────▶ "negative" (in binary, static)
/// │ ptr: 0x1000      │
/// │ len: 8           │
/// └──────────────────┘
///
/// The actual string data lives in the binary, not on heap!
/// That's why it's &'static str - it's guaranteed to live forever.
/// ```
///
/// ## Common Mistakes & How to Avoid
///
/// 1. **Forgetting to handle all cases**
///    ```rust,ignore
///    // ❌ WRONG: Compiler error - no catch-all!
///    match n {
///        0 => "zero",
///        n if n > 0 => "positive",
///        // ERROR: What about negative numbers?
///    }
///    ```
///    **Fix:** Always include `_ =>` or handle all variants
///    ```rust,ignore
///    // ✅ CORRECT:
///    match n {
///        0 => "zero",
///        n if n > 0 => "positive",
///        _ => "negative",
///    }
///    ```
///
/// 2. **Thinking ranges include endpoints when they don't**
///    ```rust,ignore
///    // ❌ WRONG: Range doesn't include endpoint!
///    match n {
///        1..10 => "small",  // Matches 1-9, NOT 10!
///        _ => "large",
///    }
///    ```
///    **Fix:** Use `..=` for inclusive ranges
///    ```rust,ignore
///    // ✅ CORRECT:
///    match n {
///        1..=10 => "small",  // Matches 1-10 inclusive
///        _ => "large",
///    }
///    ```
///
/// 3. **Missing the `if` in pattern guards**
///    ```rust,ignore
///    // ❌ WRONG: Syntax error
///    match n {
///        1..=10 where n > 5 => "medium",  // Wrong syntax!
///    }
///    ```
///    **Fix:** Use `if` not `where`
///    ```rust,ignore
///    // ✅ CORRECT:
///    match n {
///        n if n > 5 && n <= 10 => "medium",
///    }
///    ```
pub fn classify_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        n if n < 0 => "negative",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    }
}

/// Determines whether a guess is too small, too big, or correct.
///
/// ## What This Function Does
///
/// Compares two numbers and returns an Ordering enum. This teaches you about:
/// - The `cmp()` method (comparison)
/// - The `Ordering` enum (Less, Greater, Equal)
/// - Returning enum values from functions
///
/// ## The Ordering Enum
///
/// When you compare two values in Rust, you get an Ordering enum:
/// ```rust,ignore
/// pub enum Ordering {
///     Less,      // Left < Right
///     Equal,     // Left == Right
///     Greater,   // Left > Right
/// }
/// ```
///
/// This is better than returning -1, 0, or 1 (like C's strcmp) because:
/// - The type system ensures you handle all three cases
/// - It's self-documenting (Less, Equal, Greater are clear)
/// - You can't accidentally misinterpret the return value
///
/// ## Parameters
///
/// - `guess: i32` - Player's guess
/// - `secret: i32` - The secret number to guess
///
/// ## Returns
///
/// - `Ordering::Less` if guess < secret (guess is too small)
/// - `Ordering::Greater` if guess > secret (guess is too big)
/// - `Ordering::Equal` if guess == secret (they got it!)
///
/// ## Example
/// ```ignore
/// use control_flow::solution::compare_guess;
/// use std::cmp::Ordering;
///
/// assert_eq!(compare_guess(5, 10), Ordering::Less);      // 5 < 10
/// assert_eq!(compare_guess(15, 10), Ordering::Greater);  // 15 > 10
/// assert_eq!(compare_guess(10, 10), Ordering::Equal);    // 10 == 10
/// ```
///
/// ## How cmp() Works
///
/// The `cmp()` method is defined on types that implement `Ord`:
/// ```rust,ignore
/// pub fn cmp(&self, other: &Self) -> Ordering {
///     // Returns Less, Equal, or Greater
/// }
/// ```
///
/// You call it like: `guess.cmp(&secret)`
/// - `guess` = the "left" value to compare
/// - `.cmp()` = method that does the comparison
/// - `&secret` = borrowed reference to "right" value
/// - Returns `Ordering`
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameter `guess: i32` - OWNED
///   - No `&`, so we take ownership
///   - `i32` is Copy, so it's copied, not moved
///   - We can use it multiple times
///
/// - Parameter `secret: i32` - OWNED
///   - Same as guess - Copy type
///
/// - In `.cmp(&secret)`:
///   - We BORROW secret with `&`
///   - cmp() needs a reference because it compares without consuming
///   - After cmp(), secret is still available (we borrowed, didn't move)
///
/// - Return value `Ordering` - OWNED
///   - Ordering is a tiny enum, just 1 byte
///   - We return it by value (cheap copy)
///   - WHY: Data is small, copying is efficient
///
/// ## Memory Layout
///
/// ```ignore
/// Stack during function:
/// ┌──────────────────┐
/// │ guess: i32 = 42  │  4-byte integer
/// └──────────────────┘
/// ┌──────────────────┐
/// │ secret: i32 = 50 │  4-byte integer
/// └──────────────────┘
///
/// cmp() borrows secret:
/// ┌──────────────────┐
/// │ &secret          │────────▶ 50 (reference, no copy)
/// └──────────────────┘
///
/// Return value:
/// ┌──────────────────┐
/// │ Ordering::Less   │  1 byte (tiny enum)
/// └──────────────────┘
/// ```
///
/// ## Compared to Other Languages
///
/// **JavaScript:**
/// ```javascript
/// function compareGuess(guess, secret) {
///     if (guess < secret) return -1;
///     if (guess > secret) return 1;
///     return 0;
/// }
/// // Problem: Easy to return wrong value, no type safety
/// ```
///
/// **Python:**
/// ```python
/// def compare_guess(guess: int, secret: int) -> int:
///     if guess < secret: return -1
///     if guess > secret: return 1
///     return 0
/// # Problem: Return value meaning is implicit, not enforced
/// ```
///
/// **Rust:**
/// ```rust,ignore
/// fn compare_guess(guess: i32, secret: i32) -> Ordering {
///     guess.cmp(&secret)
/// }
/// // Advantage: Type system ensures you handle all three cases!
/// ```
pub fn compare_guess(guess: i32, secret: i32) -> Ordering {
    guess.cmp(&secret)
}

/// Describes a number's characteristics using if/else expressions.
///
/// ## What This Function Does
///
/// Analyzes a number and returns a description. This teaches you:
/// - If/else as EXPRESSIONS (they return values!)
/// - Early returns in functions
/// - Using patterns to describe numbers
///
/// ## If/Else As Expressions (Not Just Statements!)
///
/// In most languages, if/else is a STATEMENT (no return value).
/// In Rust, if/else is an EXPRESSION (returns a value!).
///
/// ```rust,ignore
/// // In JavaScript (statement - no value returned):
/// let description;
/// if (n == 0) {
///     description = "zero";
/// } else if (n == 1) {
///     description = "one";
/// } else {
///     description = "many";
/// }
///
/// // In Rust (expression - returns value directly):
/// let description = if n == 0 {
///     "zero"
/// } else if n == 1 {
///     "one"
/// } else {
///     "many"
/// };  // Semicolon AFTER the if/else (not inside!)
/// ```
///
/// Much cleaner! This is one of Rust's design wins.
///
/// ## Parameters
///
/// - `n: i32` - The number to describe
///
/// ## Returns
///
/// - `&'static str` - A string describing the number
///   - "zero" if n is 0
///   - "one" if n is 1
///   - "even" if n is even and > 1
///   - "odd" if n is odd and > 1
///   - "negative" if n is negative
///
/// ## Example
/// ```ignore
/// use control_flow::solution::describe_number;
///
/// assert_eq!(describe_number(0), "zero");
/// assert_eq!(describe_number(1), "one");
/// assert_eq!(describe_number(2), "even");
/// assert_eq!(describe_number(3), "odd");
/// assert_eq!(describe_number(-5), "negative");
/// ```
///
/// ## Key Pattern: Early Returns
///
/// We could write this with nested if/else, but early returns are clearer:
///
/// ```rust,ignore
/// pub fn describe_number(n: i32) -> &'static str {
///     // Handle edge cases first with early returns
///     if n < 0 {
///         return "negative";
///     }
///     if n == 0 {
///         return "zero";
///     }
///     if n == 1 {
///         return "one";
///     }
///
///     // If we get here, n >= 2
///     // Check if even or odd using modulo (%)
///     if n % 2 == 0 {
///         "even"  // Last expression is implicitly returned
///     } else {
///         "odd"
///     }
/// }
/// ```
///
/// Why is this better?
/// - Easier to read (no deep nesting)
/// - Handles edge cases upfront
/// - Clear flow: check conditions, return early, default at end
/// - Less cognitive load
///
/// ## The Modulo Operator (%)
///
/// Modulo gives you the REMAINDER after division:
/// - `10 % 3 = 1` (10 ÷ 3 = 3 remainder 1)
/// - `10 % 5 = 0` (10 ÷ 5 = 2 remainder 0)
/// - `9 % 2 = 1` (9 ÷ 2 = 4 remainder 1)
///
/// To check if even or odd:
/// - `n % 2 == 0` means even (remainder 0 when divided by 2)
/// - `n % 2 != 0` (or `== 1`) means odd (remainder 1 when divided by 2)
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameter `n: i32` - OWNED
///   - Copy type, so copying is cheap
///
/// - Return value `&'static str` - BORROWED
///   - References to string literals
///   - No allocation, no ownership transfer
///   - WHY: Literals are static, live forever
///
/// ## Memory Layout
///
/// ```ignore
/// Stack:
/// ┌──────────────────┐
/// │ n: i32 = 42      │
/// └──────────────────┘
///
/// Return (reference to literal):
/// ┌──────────────────┐
/// │ &str             │───────▶ "even" (in binary)
/// │ ptr: 0x2000      │
/// │ len: 4           │
/// └──────────────────┘
/// ```
pub fn describe_number(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n == 0 {
        return "zero";
    }
    if n == 1 {
        return "one";
    }

    // For n >= 2, check even/odd using modulo
    if n % 2 == 0 {
        "even"
    } else {
        "odd"
    }
}

/// Validates and parses a guess from user input.
///
/// ## What This Function Does
///
/// Takes a string that might be a number and returns a Result:
/// - `Ok(number)` if parsing succeeds
/// - `Err(message)` if parsing fails
///
/// This teaches you about:
/// - The `Result` type for fallible operations
/// - The `match` expression on Results
/// - String methods: `trim()`, `parse()`
/// - Error handling in Rust
///
/// ## The Result Type
///
/// Result is an enum with two variants:
/// ```rust,ignore
/// pub enum Result<T, E> {
///     Ok(T),      // Success with value of type T
///     Err(E),     // Failure with error of type E
/// }
/// ```
///
/// It forces you to think about failure!
/// In JavaScript, parse() returns undefined on failure (easy to miss).
/// In Rust, you MUST handle both Ok and Err cases.
///
/// ## Parameters
///
/// - `input: &str` - A string that might be a number
///   - Examples: "42", "hello", "123abc", " 50 "
///   - We don't own it, just borrow (&)
///   - We only need to read it
///
/// ## Returns
///
/// `Result<i32, String>`:
/// - `Ok(42)` if parsing succeeds
/// - `Err("Please enter a valid number")` if it fails
///
/// Why String for error? Because we're building an error message.
/// For library code, you'd use a custom error type.
///
/// ## Example
/// ```ignore
/// use control_flow::solution::validate_guess;
///
/// assert_eq!(validate_guess("42"), Ok(42));
/// assert_eq!(validate_guess("0"), Ok(0));
/// assert!(validate_guess("hello").is_err());
/// assert!(validate_guess("").is_err());
/// assert_eq!(validate_guess("  50  "), Ok(50));  // trim() removes spaces
/// ```
///
/// ## How to Parse a String to Number
///
/// ```rust,ignore
/// let result: Result<i32, ParseIntError> = input.trim().parse::<i32>();
///
/// match result {
///     Ok(num) => Ok(num),
///     Err(_) => Err("Please enter a valid number".to_string()),
/// }
/// ```
///
/// Breaking it down:
/// - `input.trim()` removes leading/trailing whitespace
/// - `.parse::<i32>()` tries to convert to i32
/// - Returns `Result<i32, ParseIntError>`
/// - We match on it to convert error type
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameter `input: &str` - BORROWED
///   - `&` means we borrow, don't take ownership
///   - The caller still owns the original string
///   - We only READ it, don't modify
///   - WHY: We don't need to own the input
///
/// - `input.trim()` - Returns new &str
///   - Trim doesn't allocate, just creates a new view
///   - Points to the same underlying data (subset)
///   - Also borrowed from original string
///
/// - `parse::<i32>()` - Might fail
///   - Parsing happens on the borrowed string
///   - Returns Result<i32, ParseIntError>
///   - If successful, we've extracted the number
///   - If fails, error is ParseIntError
///
/// - Return value - OWNED
///   - String::to_string() allocates new memory
///   - Error message is our new owned String
///   - OR we return Ok(num) with owned i32
///   - WHY: Caller needs to keep the number or error
///
/// ## Memory Layout
///
/// ```ignore
/// Input string (caller owns):
/// ┌──────────────────────────┐
/// │ "   42   " (in caller)   │
/// │ 9 characters             │
/// └──────────────────────────┘
///
/// Our parameter:
/// ┌──────────────────┐
/// │ input: &str      │────────▶ points to caller's string
/// │ ptr: 0x1000      │
/// │ len: 9           │
/// └──────────────────┘
///
/// After trim():
/// ┌──────────────────┐
/// │ trimmed: &str    │────────▶ same data, just different range
/// │ ptr: 0x1002      │          (points to "42", skips whitespace)
/// │ len: 2           │
/// └──────────────────┘
///
/// If parsing succeeds - return Ok:
/// ┌──────────────────┐
/// │ Ok(42)           │  (i32 is just a number, cheap)
/// └──────────────────┘
///
/// If parsing fails - return Err:
/// ┌──────────────────┐
/// │ String on heap   │────────▶ "Please enter a valid number"
/// │ ptr: 0x3000      │          (allocated String, owned by us)
/// │ len: 31          │
/// │ capacity: 31     │
/// └──────────────────┘
/// ```
pub fn validate_guess(input: &str) -> Result<i32, String> {
    match input.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid number".to_string()),
    }
}

/// Counts how many times a number can be divided by 2.
///
/// ## What This Function Does
///
/// Counts the power of 2 that divides a number evenly.
/// For example: 8 = 2³, so 8 can be divided by 2 three times.
///
/// This teaches you about:
/// - While loops with conditions
/// - Mutable variables that change
/// - Loop counters
/// - Modulo operator (%)
///
/// ## Parameters
///
/// - `mut n: i32` - The number to analyze
///   - `mut` means we can modify it in the loop
///   - We divide it by 2 repeatedly
///
/// ## Returns
///
/// - `u32` - Count of divisions (unsigned, can't be negative)
///
/// ## Example
/// ```ignore
/// use control_flow::solution::count_divisions;
///
/// assert_eq!(count_divisions(8), 3);    // 8 / 2 / 2 / 2 = 1
/// assert_eq!(count_divisions(16), 4);   // 16 / 2 / 2 / 2 / 2 = 1
/// assert_eq!(count_divisions(5), 0);    // 5 is odd, can't divide
/// assert_eq!(count_divisions(1), 0);    // 1 is already odd
/// assert_eq!(count_divisions(0), 0);    // 0 / 2 / 2... = 0 forever
/// ```
///
/// ## While Loops
///
/// Use while when you have a condition to check repeatedly:
/// ```rust,ignore
/// let mut counter = 0;
/// while n % 2 == 0 {  // While n is even
///     n /= 2;          // Divide by 2
///     counter += 1;    // Count the division
/// }
/// counter
/// ```
///
/// **Key difference from for loops:**
/// - For loops: When you know HOW MANY times to iterate
/// - While loops: When you know a CONDITION to check
///
/// ## Mutable Variables
///
/// Variables are immutable by default in Rust. To modify a variable:
/// ```rust,ignore
/// let mut x = 5;  // mut means we can change it
/// x = 6;          // This is OK
/// x += 1;         // This is OK
/// x /= 2;         // This is OK
///
/// let y = 5;      // No mut
/// y = 6;          // ERROR: can't assign to immutable binding
/// ```
///
/// **Why default immutable?**
/// - Prevents bugs where you accidentally modify something
/// - Makes code easier to reason about
/// - Forces you to think about what changes
/// - Thread-safe by default
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameter `mut n: i32` - OWNED and MUTABLE
///   - `mut` means we CAN modify it
///   - `i32` is Copy, so modification is fine
///   - We can divide it: `n /= 2`
///   - After function ends, n disappears
///   - WHY: We don't need to return the modified value
///
/// - Local variable `counter: u32` - OWNED
///   - We create it, modify it, return it
///   - Rust moves ownership to caller
///   - WHY: Caller needs the count result
///
/// ## Memory Layout
///
/// ```ignore
/// Stack at start:
/// ┌──────────────────┐
/// │ n: i32 = 8       │  (we'll modify this)
/// │ counter: u32 = 0 │  (starts at 0)
/// └──────────────────┘
///
/// After first iteration:
/// ┌──────────────────┐
/// │ n: i32 = 4       │  (8 / 2)
/// │ counter: u32 = 1 │  (incremented)
/// └──────────────────┘
///
/// After second iteration:
/// ┌──────────────────┐
/// │ n: i32 = 2       │  (4 / 2)
/// │ counter: u32 = 2 │  (incremented)
/// └──────────────────┘
///
/// After third iteration:
/// ┌──────────────────┐
/// │ n: i32 = 1       │  (2 / 2)
/// │ counter: u32 = 3 │  (incremented)
/// └──────────────────┘
///
/// Exit loop (1 is odd, 1 % 2 != 0):
/// Return 3
/// ```
///
/// ## Performance
///
/// This function is very efficient:
/// - Time: O(log n) - halves the number each iteration
/// - Space: O(1) - just two variables
///
/// For n=1,000,000, it only iterates ~20 times!
pub fn count_divisions(mut n: i32) -> u32 {
    let mut count = 0;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    count
}

/// Sums numbers in a range using a for loop.
///
/// ## What This Function Does
///
/// Adds up all integers from start to end (both inclusive).
/// This teaches you about:
/// - For loops with ranges
/// - The `..=` (inclusive) operator
/// - Accumulator pattern
///
/// ## Parameters
///
/// - `start: i32` - First number (inclusive)
/// - `end: i32` - Last number (inclusive)
///
/// ## Returns
///
/// - `i32` - The sum of all integers from start to end
///
/// ## Example
/// ```ignore
/// use control_flow::solution::sum_range;
///
/// assert_eq!(sum_range(1, 5), 15);     // 1+2+3+4+5 = 15
/// assert_eq!(sum_range(0, 10), 55);    // 0+1+2+...+10 = 55
/// assert_eq!(sum_range(5, 5), 5);      // Just 5
/// assert_eq!(sum_range(-2, 2), 0);     // -2+(-1)+0+1+2 = 0
/// ```
///
/// ## For Loops vs While Loops
///
/// **Use for when:** You're iterating over a collection or range
/// **Use while when:** You have a condition to check
///
/// ```rust,ignore
/// // For loop (know the range):
/// for i in 1..=5 {
///     sum += i;  // Easy to read, can't mess up the loop logic
/// }
///
/// // While loop (condition-based):
/// let mut i = 1;
/// while i <= 5 {
///     sum += i;
///     i += 1;  // Easy to accidentally forget to increment!
/// }
/// ```
///
/// For loops are safer - you can't forget to increment!
///
/// ## Range Operators
///
/// Rust has three range operators:
/// - `1..5` = exclusive (1, 2, 3, 4) - doesn't include 5
/// - `1..=5` = inclusive (1, 2, 3, 4, 5) - includes 5
/// - `..5` = from start, exclusive end (not used here, but good to know)
///
/// **Easy to remember:** `=` looks like it goes "up to and including"
///
/// ## Accumulator Pattern
///
/// A common pattern for combining values:
/// ```rust,ignore
/// let mut sum = 0;      // Start with identity (0 for addition)
/// for num in 1..=5 {
///     sum += num;       // Add each number
/// }
/// sum                   // Return the accumulated result
/// ```
///
/// This works for:
/// - Addition (start with 0)
/// - Multiplication (start with 1)
/// - Logical AND (start with true)
/// - Logical OR (start with false)
/// - String concatenation (start with empty String)
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameters `start`, `end` - OWNED
///   - Copy types, cheap to copy
///
/// - Local variable `sum` - OWNED and MUTABLE
///   - `let mut sum = 0;` creates mutable i32
///   - `sum += num;` modifies it
///   - Returned by value (cheap copy)
///
/// - Loop variable `num` - OWNED
///   - For each iteration, `num` gets a new value
///   - Created fresh each iteration
///   - Dropped at end of iteration
///   - WHY: Rust manages this automatically for you
///
/// ## Memory Layout
///
/// ```ignore
/// Stack at start:
/// ┌──────────────────┐
/// │ start: i32 = 1   │
/// │ end: i32 = 5     │
/// │ sum: i32 = 0     │
/// └──────────────────┘
///
/// Iteration 1 (num = 1):
/// ┌──────────────────┐
/// │ num: i32 = 1     │
/// │ sum: i32 = 1     │  (0 + 1)
/// └──────────────────┘
///
/// Iteration 2 (num = 2):
/// ┌──────────────────┐
/// │ num: i32 = 2     │
/// │ sum: i32 = 3     │  (1 + 2)
/// └──────────────────┘
///
/// ...continues through num = 5...
///
/// Final:
/// ┌──────────────────┐
/// │ sum: i32 = 15    │
/// └──────────────────┘
/// Return 15
/// ```
///
/// ## Time Complexity
///
/// O(n) where n = number of integers in range
/// - Each iteration is O(1) work
/// - We iterate (end - start + 1) times
///
/// ## Space Complexity
///
/// O(1) - just two numbers stored
pub fn sum_range(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    for num in start..=end {
        sum += num;
    }
    sum
}

/// Decides the next action based on game state.
///
/// ## What This Function Does
///
/// Uses a match expression to decide what to do based on input and game state.
/// This demonstrates combining pattern matching with additional logic.
///
/// ## Parameters
///
/// - `input: &str` - User command
/// - `game_won: bool` - Has the player won?
///
/// ## Returns
///
/// A string describing what to do
///
/// ## Example
/// ```ignore
/// use control_flow::solution::decide_action;
///
/// assert_eq!(decide_action("continue", false), "continuing game");
/// assert_eq!(decide_action("quit", false), "exiting game");
/// assert_eq!(decide_action("c", false), "continuing game");
/// assert_eq!(decide_action("q", false), "exiting game");
/// assert_eq!(decide_action("continue", true), "exiting game");
/// ```
pub fn decide_action(input: &str, game_won: bool) -> &'static str {
    // If player won, always exit
    if game_won {
        return "exiting game";
    }

    // Match on user input
    match input.trim().to_lowercase().as_str() {
        "continue" | "c" => "continuing game",
        "quit" | "q" => "exiting game",
        _ => "invalid command, please try again",
    }
}
