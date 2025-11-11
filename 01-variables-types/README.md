# Project 01 - Variables and Types

## What You're Building (Plain English)

Welcome to your first Rust project! You'll learn how to create variables, work with different types of data, and write simple functions. Think of this as learning the alphabet before writing sentences. We'll start with numbers, text, and basic operations that you'll use in every Rust program you ever write.

## New Rust Concepts in This Project

- **let keyword**: How you create variables in Rust. Unlike some languages where variables can change accidentally, Rust makes you think about whether a variable should be changeable or not.

- **Immutability by default**: Variables can't be changed unless you explicitly say so with `mut`. This prevents bugs where you accidentally modify something you didn't mean to.

- **Types**: Rust has different kinds of data - integers (whole numbers), floating-point numbers (decimals), strings (text), and more. The compiler knows what type everything is, which catches errors early.

- **String vs &str**: This is Rust-specific! `String` is text you own (on the heap), while `&str` is borrowed text (in your binary or someone else's String). Think of `String` as owning a book, and `&str` as borrowing it from the library.

## Rust Syntax You'll See

```rust
// let = keyword to declare (create) a variable
// x = name of the variable (you choose this)
// : i32 = type annotation (optional, Rust can usually figure it out)
// = 5 = the value we're assigning
// ; = statement terminator (tells Rust the statement is done)
let x: i32 = 5;

// Without type annotation (Rust infers it's i32)
let x = 5;

// Mutable variable (can be changed)
// mut = "mutable" keyword
let mut y = 5;
y = 6;  // This is OK because we said mut

// String types
let s1 = "hello";                  // &str (string slice, borrowed)
let s2 = String::from("hello");    // String (owned, on heap)

// & = borrow operator (take a reference, don't own)
let reference = &x;

// Function definition
// fn = function keyword
// name = function name
// (params: Type) = parameters with their types
// -> Type = return type
// { body } = function body
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = this is the return value
}
```

## How to Run

```bash
# Run the main binary (executes src/main.rs)
cargo run -p variables-types

# Run the tests
cargo test -p variables-types

# Run tests with output visible
cargo test -p variables-types -- --nocapture

# Check if code compiles without running
cargo check -p variables-types

# Format your code
cargo fmt -p variables-types
```

## The Exercises

You'll implement four functions:

1. **make_greeting**: Combine two names into a greeting message
   - Learn about: String manipulation, `format!` macro
   - Example: "Alice" + "Smith" → "Hello, Alice Smith!"

2. **celsius_to_fahrenheit**: Convert temperature scales
   - Learn about: f64 (decimal numbers), arithmetic
   - Example: 0°C → 32°F

3. **find_largest**: Find the biggest number in a list
   - Learn about: Slices (&[i32]), Option type, iteration
   - Example: [3, 7, 2, 9, 1] → Some(9)
   - Example: [] → None

4. **count_vowels**: Count how many vowels in a string
   - Learn about: String iteration, character checking
   - Example: "hello" → 2 (e, o)

## Solution Explanation (No Code - Just Ideas)

**make_greeting**:
- We need to take two pieces of text and combine them
- In Rust, we use the `format!` macro (like Python's f-strings or JavaScript's template literals)
- We borrow the input strings (&str) because we just need to read them, not own them
- We return a new String (owned) that the caller can keep

**celsius_to_fahrenheit**:
- Use the formula: F = (C × 9/5) + 32
- Work with f64 (64-bit floating point numbers) for decimals
- Rust is strict about types - you can't mix integers and floats
- Everything is on the stack (no heap allocation needed)

**find_largest**:
- Handle the empty case first (return None if no numbers)
- Start with the first number as our "best guess"
- Loop through the rest, updating our guess when we find a bigger number
- Return Some(number) to indicate we found something
- This shows Rust's Option type - better than returning -1 or null!

**count_vowels**:
- Convert the string to lowercase so 'A' and 'a' both count
- Iterate through each character
- Check if it's a vowel (a, e, i, o, u)
- Count how many we find
- Return the count as usize (unsigned size type)

## Where Rust Shines

**Compared to JavaScript:**
```javascript
// JavaScript - no type checking
function findLargest(numbers) {
    return Math.max(...numbers);  // Returns -Infinity if empty (!?)
}

findLargest([1, "two", 3]);  // Returns NaN - confusing!
```

**Compared to Python:**
```python
# Python - type hints optional, checked at runtime if at all
def find_largest(numbers: list[int]) -> int | None:
    return max(numbers) if numbers else None

find_largest([1, "two", 3])  # Crashes at runtime!
```

**Rust version:**
```rust
pub fn find_largest(numbers: &[i32]) -> Option<i32> {
    // Only compiles if types are correct!
    // Compiler forces you to handle the empty case
    // Impossible to pass wrong types - won't compile
}

find_largest(&[1, "two", 3]);  // ❌ Won't compile - "two" isn't an i32
```

**Why this matters:**
- Rust catches type errors at compile time, not runtime
- Your program either compiles (and is type-safe) or doesn't compile
- No "undefined is not a function" surprises in production
- No null pointer exceptions - Option makes "no value" explicit

## Common Beginner Mistakes & How to Avoid Them

1. **Forgetting `mut` when you need to change a variable**
   ```rust
   let x = 5;
   x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
   ```
   **Fix:** `let mut x = 5;`
   **Why it happens:** Coming from languages where all variables are mutable by default

2. **Trying to use `.len()` on `&str` vs `String` confusion**
   ```rust
   let s = "hello";
   s.push_str(" world");  // ❌ ERROR: no method named `push_str`
   ```
   **Fix:** `&str` is read-only. Convert to String first: `let mut s = String::from("hello");`
   **Why it happens:** Not understanding owned vs borrowed data

3. **Forgetting to return the last expression**
   ```rust
   fn add(a: i32, b: i32) -> i32 {
       a + b;  // ❌ ERROR: semicolon makes this a statement, not a return value
   }
   ```
   **Fix:** Remove the semicolon: `a + b`
   **Why it happens:** Habit from other languages where you need explicit return

4. **Not handling the None case with Option**
   ```rust
   let numbers = vec![];
   let largest = find_largest(&numbers);
   println!("Largest: {}", largest);  // ❌ ERROR: can't print Option<i32>
   ```
   **Fix:** Handle both cases:
   ```rust
   match largest {
       Some(n) => println!("Largest: {}", n),
       None => println!("No numbers!"),
   }
   ```

## Stretch Goals

Once you've completed the basic exercises, try these:

1. **Add a `fibonacci` function**: Generate the nth Fibonacci number
   - Learn about: Recursion or loops, handling overflow

2. **Add a `is_palindrome` function**: Check if a string reads the same forwards and backwards
   - Learn about: String reversal, character comparison

3. **Add a `mode` function**: Find the most common number in a slice
   - Learn about: HashMap for counting, finding max value

4. **Add better error handling**: Instead of Option, use Result with a custom error type
   - Learn about: Result type, error messages

## What's Next?

After completing this project, you'll move to **Project 02: Ownership and Borrowing**, where you'll learn Rust's superpower - memory safety without garbage collection! Understanding variables and types is essential before diving into ownership.

Good luck! Remember: read the comments in `solution.rs` - every line is explained! 🦀
