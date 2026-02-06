# Lab Conversion Example: Lab 11 - Control Flow

This document walks through a complete example of converting Lab 11 from an incomplete state (just main.rs) to a fully-featured teaching lab with lib.rs, solution.rs, comprehensive tests, and enhanced README.

## Starting State

Lab 11 currently has:
```
labs/11-control-flow/
├── Cargo.toml              ✅ (generated)
├── README.md               ⚠️ (minimal, needs enhancement)
└── src/
    └── main.rs             ✅ (9934 bytes of content)
```

## Step 1: Analyze the Existing main.rs

First, we read the existing main.rs to understand what it teaches:

```bash
head -50 labs/11-control-flow/src/main.rs
```

The main.rs contains examples of:
- Match expressions with pattern matching
- If-let syntax
- Guards in patterns
- Destructuring
- etc.

**Key functions to extract:**
1. `parse_expression()` - Parse math expressions
2. `evaluate_expression()` - Evaluate pattern-matched expressions
3. `classify_number()` - Classify numbers using match
4. etc. (based on what's actually in main.rs)

## Step 2: Run the Bootstrap Script

```bash
./scripts/convert_lab.sh labs/11-control-flow
```

This creates:
- `src/lib.rs` - Template for exercise scaffolding
- `src/solution.rs` - Template for exhaustive teaching
- `tests/integration_test.rs` - Template for tests

## Step 3: Extract Function Signatures to lib.rs

### Before (template)
```rust
pub fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    todo!("Implement [what to do]")
}
```

### After (for control-flow lab)
```rust
//! # Lab 11 - Control Flow and Pattern Matching
//!
//! This lab teaches you how Rust handles control flow with match expressions,
//! pattern matching, and guards. These are fundamental to Rust programming
//! and appear in almost every Rust program.

/// Evaluates simple mathematical expressions using pattern matching.
///
/// This function teaches you about:
/// - Match expressions (Rust's powerful pattern matching)
/// - Pattern guards (adding conditions to patterns)
/// - Destructuring (unpacking values from enums)
///
/// # Parameters
/// - `expr: &str` - A string like "5 + 3" or "10 * 2"
///
/// # Returns
/// The result of the evaluation as i32
///
/// # Example
/// ```ignore
/// use control_flow::evaluate_expression;
/// assert_eq!(evaluate_expression("5 + 3"), 8);
/// ```
pub fn evaluate_expression(expr: &str) -> i32 {
    // TODO: Parse the expression and return the result
    // Hint: You might want to split on the operator
    // Hint: Match on the operator to decide what to do
    todo!("Implement expression evaluation with pattern matching")
}

/// Classifies a number into categories using match.
///
/// This teaches you about:
/// - Match expression guards (where clauses)
/// - Matching on numeric ranges
/// - Exhaustiveness checking
///
/// # Parameters
/// - `n: i32` - The number to classify
///
/// # Returns
/// A string describing the category: "zero", "positive", "negative", "large", etc.
///
/// # Example
/// ```ignore
/// use control_flow::classify_number;
/// assert_eq!(classify_number(5), "positive");
/// assert_eq!(classify_number(0), "zero");
/// ```
pub fn classify_number(n: i32) -> &'static str {
    // TODO: Match on n to return the appropriate category
    // Hint: Use match with guards like: n if n > 100 => "large"
    todo!("Classify the number into appropriate category")
}

// Add more functions extracted from main.rs following the same pattern
```

## Step 4: Populate solution.rs with Exhaustive Documentation

### Extract from main.rs

Copy the working implementations from main.rs and add exhaustive documentation:

```rust
/// Evaluates simple mathematical expressions using pattern matching.
///
/// ## What This Function Does
///
/// Takes a string like "5 + 3" and evaluates it using match expressions.
/// This demonstrates pattern matching, one of Rust's most powerful features.
///
/// ## How It Works
/// 1. Split the input string on the operator
/// 2. Parse the two numbers
/// 3. Match on the operator to decide which operation to perform
/// 4. Return the result
///
/// ## Parameters
///
/// - `expr: &str` - Let's break this down:
///   - `expr` = parameter name
///   - `:` = "has type" separator
///   - `&` = borrow operator (we don't take ownership, just look at the string)
///   - `str` = string slice type (text that lives somewhere else)
///
///   We use `&str` because we only need to read the expression, not own it.
///   After this function ends, the caller still owns the original string.
///
/// ## Returns
/// - `i32` - A 32-bit signed integer with the result
///   - We return an owned value (no `&`), so it transfers to the caller
///   - They can use it however they want
///
/// ## Example
/// ```ignore
/// let result = evaluate_expression("5 + 3");
/// assert_eq!(result, 8);
/// ```
///
/// ## Pattern Matching Explanation
///
/// Match expressions in Rust are exhaustive - they MUST handle every possible case.
/// This is different from switch statements in other languages:
///
/// ```rust
/// match value {
///     pattern1 => { /* code */ },
///     pattern2 => { /* code */ },
///     _ => { /* catch all other cases */ }
/// }
/// ```
///
/// **Key difference from JavaScript switch:**
/// - JavaScript: switch statements fall through unless you use `break`
/// - Rust: match arms don't fall through, each arm stands alone
/// - Rust: compiler requires you handle ALL cases (or use _ wildcard)
///
/// **Pattern Guards:**
/// You can add conditions to patterns with `if`:
/// ```rust
/// match x {
///     n if n > 0 => println!("positive"),
///     n if n < 0 => println!("negative"),
///     _ => println!("zero"),
/// }
/// ```
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameter `expr` is BORROWED (&str)
///   - The `&` means we're borrowing, not taking ownership
///   - The caller still owns the original string
///   - We can only READ the data (immutable borrow)
///   - WHY: We don't need to own the expression, just parse it
///   - After this function ends, expr just disappears (it was a reference)
///
/// - Return value is OWNED (i32)
///   - No `&`, so we own the result
///   - i32 is on the stack (not heap), so very cheap to copy
///   - Caller receives ownership of the result
///   - WHY: Caller needs the result after our function ends
///
/// ## Memory Layout
///
/// ```ignore
/// Stack (when function is called):
/// ┌──────────────────────┐
/// │ expr: &str           │─────────▶ "5 + 3" (in caller's memory)
/// │  - ptr: 0x1000       │            (string data lives elsewhere)
/// │  - len: 5            │
/// └──────────────────────┘
///
/// ┌──────────────────────┐
/// │ result: i32          │              (8)
/// │  - value: 8          │  (lives on stack, very efficient)
/// └──────────────────────┘
/// ```
///
/// ## Time Complexity: O(n) where n = length of expression string
/// - We scan the string once to split and parse
///
/// ## Space Complexity: O(n) for parsing the numbers
/// - We allocate strings for the number parts during parsing
///
/// ## Common Mistakes & How to Avoid
///
/// 1. **Not handling all cases in match**
///    ```rust
///    // ❌ WRONG:
///    match operator {
///        '+' => result = a + b,
///        '-' => result = a - b,
///        // ERROR: What about '*', '/', etc?
///    }
///    ```
///    **Fix:** Use the `_` wildcard or explicitly handle all cases
///    ```rust
///    // ✅ CORRECT:
///    match operator {
///        '+' => result = a + b,
///        '-' => result = a - b,
///        '*' => result = a * b,
///        '/' => result = a / b,
///        _ => panic!("Unknown operator"),
///    }
///    ```
///    **Why it happens:** Coming from languages where switch cases fall through
///
/// 2. **Forgetting patterns must be exhaustive**
///    Rust compiler REQUIRES you handle all cases - this prevents bugs!
///
pub fn evaluate_expression(expr: &str) -> i32 {
    // Split on space to get "5", "+", "3"
    let parts: Vec<&str> = expr.split_whitespace().collect();

    if parts.len() != 3 {
        panic!("Expected format: 'a op b'");
    }

    let a: i32 = parts[0].parse().expect("First part must be a number");
    let op = parts[1];
    let b: i32 = parts[2].parse().expect("Third part must be a number");

    // Match on the operator - this is pattern matching!
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => panic!("Unknown operator: {}", op),
    }
}

// Continue with more functions...
```

## Step 5: Write Comprehensive Tests

### integration_test.rs structure

```rust
use control_flow::solution::*;

// ============================================================================
// TESTS FOR: evaluate_expression
// ============================================================================

#[test]
fn test_evaluate_expression_addition() {
    // Test basic addition
    assert_eq!(evaluate_expression("5 + 3"), 8);
}

#[test]
fn test_evaluate_expression_subtraction() {
    // Test subtraction (order matters!)
    assert_eq!(evaluate_expression("10 - 3"), 7);
    assert_eq!(evaluate_expression("3 - 10"), -7);
}

#[test]
fn test_evaluate_expression_multiplication() {
    // Test multiplication
    assert_eq!(evaluate_expression("5 * 3"), 15);
    assert_eq!(evaluate_expression("0 * 100"), 0);
}

#[test]
fn test_evaluate_expression_division() {
    // Test division
    assert_eq!(evaluate_expression("10 / 2"), 5);
}

#[test]
fn test_evaluate_expression_negative_numbers() {
    // Test with negative numbers
    assert_eq!(evaluate_expression("-5 + 3"), -2);
    assert_eq!(evaluate_expression("5 + -3"), 2);
}

#[test]
fn test_evaluate_expression_zero() {
    // Test with zero
    assert_eq!(evaluate_expression("0 + 5"), 5);
    assert_eq!(evaluate_expression("5 - 5"), 0);
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

#[test]
fn test_property_addition_commutative() {
    // Property: addition is commutative (a + b = b + a)
    assert_eq!(
        evaluate_expression("5 + 3"),
        evaluate_expression("3 + 5")
    );
}

#[test]
fn test_property_subtraction_not_commutative() {
    // Property: subtraction is NOT commutative (a - b ≠ b - a)
    let result1 = evaluate_expression("10 - 3");
    let result2 = evaluate_expression("3 - 10");
    assert_ne!(result1, result2);
    assert_eq!(result1, -result2);  // They're opposites
}

// Add more tests...
```

## Step 6: Enhance README.md

Start from `templates/README.md.template` and fill in specifics:

```markdown
# Lab 11 - Control Flow and Pattern Matching

## What You're Building (Plain English)

In this lab, you'll learn how Rust handles decision-making with match expressions.
Instead of if-else chains, Rust gives you pattern matching - a powerful way to
work with complex data. You'll write expressions that evaluate math problems and
classify numbers using patterns.

Think of pattern matching like sorting mail - you look at each piece and decide
where it goes based on its characteristics. Rust makes sure you handle every
possible case!

## New Rust Concepts in This Lab

- **Match Expressions**: Like `switch` statements but more powerful. Every pattern
  MUST be handled (exhaustive), which prevents bugs.

- **Pattern Guards**: Add conditions to patterns with `if`, letting you match on
  ranges and computed values.

- **Destructuring**: Break apart complex data types (enums, structs, tuples) into
  their components so you can work with them.

## Where Rust Shines

**Compared to JavaScript:**
```javascript
// JavaScript switch - can fall through accidentally!
switch (operator) {
    case '+':
        result = a + b;
        break;  // Easy to forget!
    case '-':
        result = a - b;
        // OOPS - no break! Falls through to next case!
}
```

**Rust match - exhaustive and no fallthrough:**
```rust
match operator {
    '+' => a + b,
    '-' => a - b,
    // Compiler ERROR if you forget any case!
    // And you can't accidentally fall through to the next arm
}
```

**Key differences:**
- Rust requires you handle EVERY case
- No accidental fall-through bugs
- More concise - no need for `break` statements
- Compiler catches incomplete pattern matching at compile time

## Common Beginner Mistakes

1. **Forgetting to handle all cases**
   ```rust
   // ❌ WRONG: Compiler error - cases not exhaustive
   match x {
       1 => println!("one"),
       2 => println!("two"),
   }
   ```
   **Fix:** Use `_` to catch all other cases:
   ```rust
   // ✅ CORRECT:
   match x {
       1 => println!("one"),
       2 => println!("two"),
       _ => println!("other"),
   }
   ```

2. **Thinking match arms fall through (like switch)**
   ```rust
   // In JavaScript, this falls through
   // In Rust, each arm is independent - no fallthrough!
   match x {
       1 => println!("one"),   // Code after this doesn't run for other arms
       2 => println!("two"),
       _ => {}
   }
   ```

## Memory Model

### At a High Level

Rust uses match expressions to safely branch on data. Unlike other languages,
the compiler FORCES you to consider all possibilities before the code compiles.

### Symbol Deep Dive: The `@` Pattern Binding

- **What it means**: Capture a matched value with a name
- **Example**: `n @ 1..=5` captures numbers 1-5 as `n`
- **Why it matters**: Lets you work with the matched value

## What's Next?

After this lab, you'll move to **Lab 12: Enums and Pattern Matching**, where
you'll create your own custom types and match on them. Understanding pattern
matching fundamentals here is essential!

---

Good luck! Pattern matching is one of Rust's superpowers. 🦀
```

## Step 7: Test Everything

```bash
cd labs/11-control-flow

# Check it compiles
cargo check

# Run all tests
cargo test

# Run main binary
cargo run

# Format code
cargo fmt

# Verify there are no warnings
cargo clippy
```

## Step 8: Verify with Checklist

- [ ] `src/lib.rs` has function signatures with todos
- [ ] `src/solution.rs` has exhaustive documentation
- [ ] `tests/integration_test.rs` has 20+ tests
- [ ] `README.md` has full pedagogical structure
- [ ] All tests pass
- [ ] All code compiles without warnings
- [ ] Main binary runs without panicking

## Time Estimate for This Lab

- Analyze main.rs: 20 min
- Extract to lib.rs: 45 min
- Write solution.rs docs: 90 min
- Write comprehensive tests: 90 min
- Enhance README: 60 min
- Testing & refinement: 30 min
- **Total: ~5-6 hours**

## Key Learnings for This Example

1. **Understand before you document** - Read main.rs thoroughly first
2. **Start with functions** - Extract lib.rs signatures before solution.rs docs
3. **Tests drive understanding** - Writing tests reveals what the code should do
4. **Documentation is key** - Exhaustive docs are what makes labs teachable
5. **Use templates** - They save time and ensure consistency

## Next Labs to Convert (Priority Order)

1. Lab 15 - Lifetimes (critical concept)
2. Lab 20 - Multithreading (important systems concept)
3. Lab 21 - Async/await (increasingly important)
4. Lab 25 - Transaction Pool (domain-specific)
5. Lab 11 - Control Flow (this example)

Good luck converting! Remember: quality of documentation > quantity of labs.
