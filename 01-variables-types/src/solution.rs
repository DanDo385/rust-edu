//! # Variables and Types - Complete Solution with EXHAUSTIVE Explanations
//!
//! ## What We're Building
//!
//! This module contains four fundamental functions that teach you Rust's type system
//! and variable handling. We're building the foundation for everything else in Rust:
//! - Working with different data types (integers, floats, strings)
//! - Understanding owned vs borrowed data
//! - Handling cases where values might not exist (Option type)
//! - Basic iteration and logic
//!
//! These might seem simple, but they demonstrate concepts that prevent entire classes
//! of bugs that plague other languages!
//!
//! ## Why Rust Is Perfect For This
//!
//! - **Memory safety**: String operations can't overflow buffers or cause segfaults.
//!   In C, string concatenation can easily write past array bounds. Rust prevents this
//!   at compile time.
//!
//! - **Zero-cost abstractions**: Our iterator chains compile down to the same assembly
//!   as hand-written loops, but are more readable and can't have off-by-one errors.
//!
//! - **Type system**: The compiler knows the type of every value and won't let you
//!   mix incompatible types. In JavaScript, `"5" + 5 = "55"` (wat?). In Rust, this
//!   won't compile - you must be explicit.
//!
//! - **Compared to JavaScript**: No type coercion surprises, no undefined, no null
//! - **Compared to Python**: No runtime type errors, no None.attribute() crashes
//! - **Compared to Go**: More expressive type system, no nil pointer panics
//!
//! ## Key Rust Concepts You'll Learn
//!
//! - **Immutability by default**: Variables can't change unless marked `mut`
//! - **Ownership vs borrowing**: String vs &str demonstrates this beautifully
//! - **Option type**: Rust's way of saying "might not have a value" (no null!)
//! - **Expression-based**: Last expression without ; is the return value
//! - **Type inference**: Compiler figures out types for you (usually)
//!
//! ## Time Complexity: O(n) for iteration-based functions, O(1) for arithmetic
//! ## Space Complexity: O(n) for string building, O(1) for arithmetic and finding largest

/// Creates a greeting message by combining first and last name.
///
/// ## What This Function Does
///
/// Takes two names (first and last) and creates a friendly greeting message.
/// This is your introduction to string handling in Rust, which is different
/// from most languages because Rust distinguishes between owned and borrowed text.
///
/// ## Parameters
///
/// - `first: &str` - Let's break this down completely:
///   - `first` = parameter name (you choose this, it's descriptive)
///   - `:` = "has type" separator (says what type comes next)
///   - `&` = "borrow" operator - we're not taking ownership
///   - `str` = string slice type (text data that lives somewhere else)
///
///   The `&str` type means "a borrowed view of a string". The string data lives
///   somewhere else (maybe in the binary, maybe in someone's String), and we're
///   just borrowing a reference to read it. We can't modify it, and we don't own it.
///
/// - `last: &str` - Same as first, another borrowed string slice
///
/// ## Returns
///
/// - `String` (no `&`) = We're giving ownership to the caller
///   - This is a heap-allocated, growable, owned string
///   - The caller can keep it, modify it, or drop it
///   - We allocate it here and transfer ownership on return
///
/// ## Example
/// ```ignore
/// let greeting = make_greeting("Alice", "Smith");
/// println!("{}", greeting);  // "Hello, Alice Smith!"
/// // greeting owns the string, will be freed when greeting goes out of scope
/// ```ignore
///
/// ## Ownership & Borrowing Analysis
///
/// - Parameters `first` and `last` are BORROWED (&str)
///   - The `&` means we're borrowing, not taking ownership
///   - The caller still owns the original data
///   - We can only READ the data (immutable borrow)
///   - WHY: We don't need to own the strings, just look at them
///   - After this function ends, first and last just disappear (they were references)
///
/// - Return value is OWNED (String)
///   - No `&`, so ownership transfers to caller
///   - We create the String on the heap
///   - Caller receives ownership and must eventually drop it (Rust does this automatically)
///   - WHY: Caller needs to keep the result after our function ends
///
/// ## Memory Layout
/// ```ignore
/// Stack (when function is called):          Heap:
/// ┌──────────────────────┐
/// │ first: &str          │──────────▶  "Alice" (lives in caller's memory or binary)
/// │  - ptr: 0x1000       │
/// │  - len: 5            │
/// └──────────────────────┘
/// ┌──────────────────────┐
/// │ last: &str           │──────────▶  "Smith" (lives in caller's memory or binary)
/// │  - ptr: 0x2000       │
/// │  - len: 5            │
/// └──────────────────────┘
///
/// After format! creates the String:
/// ┌──────────────────────┐
/// │ result: String       │──────────▶  ┌───────────────────────┐
/// │  - ptr: 0x3000       │             │ "Hello, Alice Smith!" │
/// │  - len: 19           │             │   (19 bytes of text)  │
/// │  - capacity: 19      │             └───────────────────────┘
/// └──────────────────────┘              (allocated on heap)
///
/// Return: Ownership of the String moves to caller
/// ```ignore
pub fn make_greeting(first: &str, last: &str) -> String {
    // ========================================================================
    // UNDERSTANDING THE format! MACRO
    // ========================================================================

    // `format!` is a MACRO (notice the `!` at the end)
    //
    // What's a macro?
    // - Macros are like super-powered functions that run at compile time
    // - They can generate code based on their inputs
    // - The `!` tells you it's a macro, not a function
    // - Common macros: println!, vec!, format!, panic!
    //
    // What does format! do?
    // - Takes a format string with `{}` placeholders
    // - Replaces each `{}` with the values you provide
    // - Returns a new String (owned, on heap)
    //
    // The `{}` are placeholders that get replaced with the values:
    // - First `{}` gets replaced with `first`
    // - Second `{}` gets replaced with `last`
    //
    // This is similar to:
    // - Python: f"Hello, {first} {last}!"
    // - JavaScript: `Hello, ${first} ${last}!`
    // - Go: fmt.Sprintf("Hello, %s %s!", first, last)
    // - C: sprintf(buffer, "Hello, %s %s!", first, last)
    //
    // But safer! Rust's format! is:
    // - Type-safe (won't compile if types don't match)
    // - Buffer-safe (can't overflow)
    // - Memory-safe (automatically allocates the right amount)

    format!("Hello, {} {}!", first, last)

    // ============================================================================
    // WHY NO SEMICOLON?
    // ============================================================================
    //
    // In Rust, the last expression in a function (without a semicolon) is the
    // return value. This is called "expression-oriented programming".
    //
    // These are equivalent:
    //   format!("Hello, {} {}!", first, last)        // Implicit return
    //   return format!("Hello, {} {}!", first, last); // Explicit return
    //
    // The first form (no semicolon, no return keyword) is more idiomatic in Rust.
    //
    // What's the difference between statement and expression?
    // - Expression: Evaluates to a value (e.g., `5 + 5` evaluates to `10`)
    // - Statement: Does something but doesn't evaluate to a value
    //
    // Adding a semicolon turns an expression into a statement:
    //   let x = 5 + 5;  // Statement (the ; makes it so)
    //   5 + 5           // Expression (evaluates to 10)

    // ============================================================================
    // OWNERSHIP ANALYSIS - CRITICAL RUST CONCEPT
    // ============================================================================
    //
    // Let's trace what happens to ownership in this function:
    //
    // 1. Parameters `first` and `last` are BORROWED (&str)
    //    - The `&` means we're BORROWING, not taking ownership
    //    - The caller still owns the original string data
    //    - We can only READ it (immutable borrow)
    //    - WHY: We don't need to own it, just look at it to copy the characters
    //    - When function ends: The borrows end, but the original data still exists
    //
    // 2. `format!` CREATES a new String (owned)
    //    - Allocates memory on the heap
    //    - Copies characters from first and last
    //    - Creates a new owned String with the combined text
    //    - This String is independent of first and last
    //
    // 3. Return value transfers OWNERSHIP
    //    - Ownership moves from this function to the caller
    //    - Caller is now responsible for this memory
    //    - When caller is done, the String is automatically freed
    //    - Rust's ownership system ensures no memory leaks!

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // JavaScript equivalent:
    // ```javascript
    // function makeGreeting(first, last) {
    //     return `Hello, ${first} ${last}!`;
    // }
    // ```
    // Issues with JavaScript version:
    // - No type checking: Could pass numbers, undefined, objects, anything
    // - Might get "Hello, undefined undefined!" at runtime
    // - No compiler to catch mistakes
    //
    // Python equivalent:
    // ```python
    // def make_greeting(first, last):
    //     return f"Hello, {first} {last}!"
    // ```
    // Issues with Python version:
    // - No compile-time type checking
    // - Could pass None and get "Hello, None None!"
    // - Would only find bug by running the code
    //
    // Go equivalent:
    // ```go
    // func makeGreeting(first, last string) string {
    //     return fmt.Sprintf("Hello, %s %s!", first, last)
    // }
    // ```
    // Better! Has types. But:
    // - Uses garbage collector (slight performance cost)
    // - Can have nil pointer panics in other contexts
    //
    // C equivalent:
    // ```c
    // char* makeGreeting(const char* first, const char* last) {
    //     char* result = malloc(100);  // Hope 100 is enough!
    //     sprintf(result, "Hello, %s %s!", first, last);  // Buffer overflow risk!
    //     return result;  // Caller must remember to free()!
    // }
    // ```
    // Issues with C version:
    // - Fixed buffer size (could overflow)
    // - Manual memory management (easy to leak or double-free)
    // - No help from compiler
    //
    // Rust version advantages:
    // - Type-safe: Won't compile with wrong types
    // - Memory-safe: Allocates exactly the right amount
    // - No garbage collector: Memory freed automatically via ownership
    // - No buffer overflows: String grows as needed
    // - No memory leaks: Compiler ensures cleanup
    // - This is Rust's superpower!
}

/// Converts Celsius to Fahrenheit.
///
/// ## What This Function Does
///
/// Takes a temperature in Celsius and converts it to Fahrenheit using
/// the standard conversion formula. This teaches you about working with
/// floating-point numbers in Rust.
///
/// Formula: F = (C × 9/5) + 32
///
/// ## Rust Concepts Demonstrated
///
/// - **f64 type**: 64-bit floating-point number (decimal numbers)
/// - **Type inference**: Rust knows 9.0/5.0 are f64 because celsius is f64
/// - **Type safety**: Can't mix int and float without explicit conversion
/// - **Arithmetic**: Standard math operators work as expected
///
/// ## Parameters
///
/// - `celsius: f64` - Temperature in Celsius
///   - `f64` = 64-bit floating point (IEEE 754 double precision)
///   - Can represent decimals like 37.5 or 0.1
///   - Takes 8 bytes of memory
///   - Passed by value (copied) because numbers are cheap to copy
///   - Range: approximately ±1.7E+308 with 15-17 decimal digits of precision
///
/// ## Returns
///
/// Temperature in Fahrenheit as f64
///
/// ## Example
/// ```ignore
/// let f = celsius_to_fahrenheit(0.0);
/// assert_eq!(f, 32.0);  // Water freezes at 0°C = 32°F
///
/// let f = celsius_to_fahrenheit(100.0);
/// assert_eq!(f, 212.0);  // Water boils at 100°C = 212°F
///
/// let f = celsius_to_fahrenheit(-40.0);
/// assert_eq!(f, -40.0);  // -40° is the same in both scales!
/// ```ignore
///
/// ## Memory Layout
/// ```ignore
/// Stack:
/// ┌────────────────┐
/// │ celsius: f64   │  8 bytes: IEEE 754 double precision
/// │ (e.g., 100.0)  │  Sign(1) + Exponent(11) + Fraction(52) bits
/// └────────────────┘
///
/// Calculation happens in CPU registers (very fast!)
///
/// ┌────────────────┐
/// │ result: f64    │  8 bytes: computed result
/// │ (e.g., 212.0)  │
/// └────────────────┘
///
/// Return: Value is copied to caller (cheap - just 8 bytes)
///
/// No heap allocation needed - everything on stack!
/// ```ignore
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // ========================================================================
    // UNDERSTANDING THE CONVERSION FORMULA
    // ========================================================================

    // Mathematical formula: F = (C × 9/5) + 32
    //
    // Why this formula?
    // - Celsius scale: 0° = freezing, 100° = boiling (100 degree range)
    // - Fahrenheit scale: 32° = freezing, 212° = boiling (180 degree range)
    // - Ratio: 180/100 = 9/5 = 1.8
    // - Offset: Fahrenheit starts at 32, not 0
    //
    // Breaking it down step by step:
    // 1. Multiply celsius by 9     → scales up by 9x
    // 2. Divide by 5               → scales back down (net effect: multiply by 1.8)
    // 3. Add 32                    → offsets to Fahrenheit zero point
    //
    // Order of operations (PEMDAS applies):
    // - Multiplication and division (left to right): celsius * 9.0 / 5.0
    // - Then addition: ... + 32.0

    // `*` = multiplication operator
    // `/` = division operator
    // `+` = addition operator

    celsius * 9.0 / 5.0 + 32.0

    // ========================================================================
    // WHY USE .0 ON THE NUMBERS?
    // ========================================================================
    //
    // Type strictness in Rust:
    // - `9` is an integer literal (i32 by default)
    // - `9.0` is a floating-point literal (f64 by default)
    // - `celsius` is f64 (from function signature)
    //
    // You CANNOT mix types in Rust arithmetic:
    // - `celsius * 9` would be a compile error!
    // - Error: "cannot multiply `f64` by `{integer}`"
    //
    // Writing `9.0` tells Rust "this is a floating point number"
    // Rust then infers it should be f64 to match celsius
    //
    // This prevents bugs that are common in other languages:
    // - In C: mixing int and float can cause implicit conversion and precision loss
    // - In Python: usually works but can surprise you
    // - In JavaScript: type coercion can give unexpected results
    //
    // Rust forces you to be explicit, which catches errors at compile time!
    //
    // If you needed to use an integer, you'd convert explicitly:
    //   let int_value: i32 = 9;
    //   celsius * (int_value as f64)  // Explicit conversion with 'as'

    // ========================================================================
    // OWNERSHIP ANALYSIS
    // ========================================================================
    //
    // 1. `celsius` is an f64 (primitive type)
    //    - Primitive types implement the `Copy` trait
    //    - When we use celsius in the calculation, it's COPIED, not moved
    //    - We could use celsius again after the calculation if we wanted:
    //      ```
    //      let result = celsius * 9.0 / 5.0 + 32.0;
    //      println!("Input was: {}", celsius);  // Still valid!
    //      ```
    //    - This is different from String, which MOVES by default
    //
    // 2. The arithmetic operations create a new f64 value
    //    - Each operation produces a new temporary value
    //    - These temporaries live only for this expression
    //    - The final result is what gets returned
    //
    // 3. Return value is COPIED to the caller
    //    - f64 implements Copy, so it's copied, not moved
    //    - Copying 8 bytes is cheap (happens in CPU registers)
    //    - No heap allocation, no pointers, just raw numbers
    //
    // 4. Memory footprint is TINY
    //    - Input: 8 bytes (f64)
    //    - Output: 8 bytes (f64)
    //    - All on the stack, super fast!
    //    - No garbage collector needed
    //    - No allocations to manage

    // ========================================================================
    // COPY VS MOVE - IMPORTANT DISTINCTION
    // ========================================================================
    //
    // Types that implement Copy (all primitive types):
    // - Integers: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    // - Floats: f32, f64
    // - Booleans: bool
    // - Characters: char
    // - Tuples of Copy types: (i32, f64)
    //
    // Behavior: Assigned/passed by copying the value
    // ```
    // let x = 5;
    // let y = x;  // x is copied to y, both are valid
    // ```
    //
    // Types that DON'T implement Copy (most complex types):
    // - String
    // - Vec<T>
    // - Box<T>
    // - Most structs (unless you derive Copy)
    //
    // Behavior: Assigned/passed by moving ownership
    // ```
    // let s1 = String::from("hello");
    // let s2 = s1;  // s1 is moved to s2, s1 is no longer valid
    // ```
    //
    // Why the difference?
    // - Copy is for types that are cheap to copy (small, stack-only)
    // - Move is for types that own heap memory (expensive to copy)
    // - This prevents accidental expensive operations!

    // ========================================================================
    // RUST VS OTHER LANGUAGES
    // ========================================================================
    //
    // Python:
    // ```python
    // def celsius_to_fahrenheit(celsius):
    //     return celsius * 9 / 5 + 32
    // ```
    // Issues:
    // - Could pass a string "100" → runtime error
    // - Could pass None → runtime error
    // - No type checking at compile time
    //
    // JavaScript:
    // ```javascript
    // function celsiusToFahrenheit(celsius) {
    //     return celsius * 9 / 5 + 32;
    // }
    // ```
    // Issues:
    // - "100" * 9 / 5 + 32 = 212 (implicit conversion, confusing!)
    // - undefined * 9 / 5 + 32 = NaN (fails silently)
    // - No type checking
    //
    // Go:
    // ```go
    // func celsiusToFahrenheit(celsius float64) float64 {
    //     return celsius * 9.0 / 5.0 + 32.0
    // }
    // ```
    // Better! Has types. Similar to Rust in this case.
    // But less strict in other areas (can have nil pointer panics)
    //
    // C:
    // ```c
    // double celsius_to_fahrenheit(double celsius) {
    //     return celsius * 9.0 / 5.0 + 32.0;
    // }
    // ```
    // Similar to Rust here, but:
    // - No safety guarantees in complex code
    // - Easy to have undefined behavior
    // - No borrow checker to help you
    //
    // Rust advantages:
    // - Type-safe: Won't compile if you pass wrong type
    // - Memory-safe: No undefined behavior possible
    // - Zero-cost: Compiles to same assembly as C
    // - Clear: Type in signature tells you exactly what it expects
}

/// Finds the largest number in a slice of integers.
///
/// ## What This Function Does
///
/// Looks through a list of numbers and finds the biggest one.
/// If the list is empty, returns None (Rust's way of saying "no value").
/// This demonstrates Rust's Option type, which is used instead of null.
///
/// ## Rust Concepts Demonstrated
///
/// - **Slices**: &[i32] is a borrowed view into an array or Vec
/// - **Option**: Rust's way of handling "might not exist" (instead of null)
/// - **Pattern matching**: Using different code paths for different cases
/// - **Iterators**: Efficient way to go through collections
/// - **Borrowing**: We borrow the slice, don't take ownership
///
/// ## Parameters
///
/// - `numbers: &[i32]` - Let's break this down completely:
///   - `numbers` = parameter name
///   - `:` = "has type"
///   - `&` = borrowed (we don't own this data)
///   - `[i32]` = slice of i32s (a view into a contiguous sequence of integers)
///
///   What's a slice?
///   - A slice is a view into a sequence of elements
///   - It's a pointer to the first element + a length
///   - It doesn't own the data, just references it
///   - Think of it like a window showing part (or all) of an array
///
///   The type `&[i32]` means:
///   - A borrowed reference to a slice
///   - The slice contains i32 values (32-bit signed integers)
///   - We can read the data but not modify it (immutable borrow)
///   - The caller keeps ownership of the actual array/Vec
///
/// ## Returns
///
/// - `Option<i32>` = Maybe has an i32, maybe doesn't
///   - `Some(number)` if we found at least one number
///   - `None` if the slice was empty
///
///   Why Option instead of null?
///   - Null pointer errors are billion-dollar mistakes (Tony Hoare's words)
///   - In Java/C/C++/JavaScript, null can cause crashes
///   - Rust has NO null! Instead, Option makes "no value" explicit
///   - Compiler forces you to handle both Some and None cases
///   - Can't forget to check - won't compile if you don't handle it
///
/// ## Example
/// ```ignore
/// let nums = vec![3, 7, 2, 9, 1];
/// let largest = find_largest(&nums);
/// assert_eq!(largest, Some(9));
///
/// let empty: Vec<i32> = vec![];
/// let largest = find_largest(&empty);
/// assert_eq!(largest, None);
///
/// let negatives = vec![-5, -1, -10];
/// let largest = find_largest(&negatives);
/// assert_eq!(largest, Some(-1));  // -1 is largest of the three
/// ```ignore
///
/// ## Memory Layout
/// ```ignore
/// Caller's stack:                    Caller's heap:
/// ┌─────────────────────┐
/// │ vec: Vec<i32>       │────────▶  ┌───┬───┬───┬───┬───┐
/// │  - ptr: 0x1000      │           │ 3 │ 7 │ 2 │ 9 │ 1 │
/// │  - len: 5           │           └───┴───┴───┴───┴───┘
/// │  - capacity: 5      │            (each i32 is 4 bytes)
/// └─────────────────────┘
///         │
///         │ (when we call find_largest(&vec), we pass a slice)
///         ▼
/// Our function's stack:
/// ┌─────────────────────┐
/// │ numbers: &[i32]     │────────▶  (points to same heap data above)
/// │  - ptr: 0x1000      │            We don't own it, just borrow it!
/// │  - len: 5           │
/// └─────────────────────┘
/// ┌─────────────────────┐
/// │ largest: i32        │           (holds a copy of the largest value)
/// │  value: 9           │           (4 bytes on stack)
/// └─────────────────────┘
///
/// Return: Option<i32>
/// ┌─────────────────────┐
/// │ Some(9)             │           (enum with the value inside)
/// │  discriminant: 1    │           (1 byte: 0 for None, 1 for Some)
/// │  value: 9           │           (4 bytes: the i32 value)
/// └─────────────────────┘
///
/// We didn't copy the entire array! Just borrowed a view of it.
/// Very efficient - no matter how big the array is, we only use:
/// - 16 bytes for the slice reference (ptr + len on 64-bit system)
/// - 4 bytes for tracking the largest number
/// ```ignore
pub fn find_largest(numbers: &[i32]) -> Option<i32> {
    // ========================================================================
    // STEP 1: HANDLE THE EMPTY CASE
    // ========================================================================

    // `if` = conditional statement keyword
    //   - Evaluates a boolean expression
    //   - Executes the block if true
    //   - Can have optional `else` branch
    //
    // `numbers.is_empty()` = method call that checks if slice has no elements
    //   - `numbers` = our slice parameter
    //   - `.` = access a method on numbers (dot notation)
    //   - `is_empty()` = method that returns bool
    //     - Returns true if length is 0
    //     - Returns false otherwise
    //   - `()` = no parameters to the method
    //
    // Why check this first?
    // - If there are no numbers, we can't find a largest one
    // - Trying to access numbers[0] on empty slice would panic!
    // - Better to handle this case explicitly and safely

    if numbers.is_empty() {
        // Early return with None
        // `return` = keyword to exit function immediately with a value
        // `None` = the "no value" variant of Option<i32>
        //   - Option is an enum with two variants: Some(T) and None
        //   - None means "there is no value"
        //   - This is MUCH better than returning -1 or null!
        //
        // Why is None better than null?
        // - Type-safe: Option<i32> explicitly says "might not have a value"
        // - Compiler enforces handling: Must check Some vs None
        // - Can't accidentally use None as if it were a number
        // - No null pointer exceptions possible!
        //
        // `;` = statement terminator (required after return)

        return None;
    }

    // If we reach here, we know numbers has at least one element
    // The compiler doesn't know this, but we do from the logic

    // ========================================================================
    // STEP 2: START WITH THE FIRST NUMBER
    // ========================================================================

    // `let` = keyword to declare a variable
    // `mut` = mutable keyword - this variable can be changed
    //   - Without `mut`, variables are immutable by default in Rust
    //   - We need `mut` because we'll update `largest` in the loop
    //   - This is Rust's safety-by-default philosophy
    //
    // `largest` = our variable name (descriptive names are good!)
    //
    // `=` = assignment operator
    //
    // `numbers[0]` = array/slice indexing syntax
    //   - `numbers` = our slice
    //   - `[0]` = access element at index 0 (first element)
    //   - Indexes are 0-based like most languages
    //   - This COPIES the i32 value (i32 implements Copy trait)
    //
    // Why is indexing safe here?
    // - We already checked that numbers is not empty
    // - So we know numbers[0] exists
    // - If we hadn't checked, this could panic at runtime
    //
    // What's happening in memory?
    // - numbers[0] reads the first i32 from the slice
    // - i32 is a Copy type, so the value is copied
    // - `largest` now holds a copy of that number on the stack
    // - The original number in the slice is unchanged

    let mut largest = numbers[0];

    // WHY START WITH numbers[0]?
    // - We need something to compare against
    // - The first element is a good starting point
    // - As we loop, we'll update this if we find a bigger number
    // - Alternative approach: could use numbers.iter().max()
    //   But we're doing it manually to show the algorithm!

    // ========================================================================
    // STEP 3: ITERATE THROUGH ALL NUMBERS
    // ========================================================================

    // `for` = loop keyword for iteration
    //   - Executes the block once for each element
    //   - Automatically handles the iteration logic
    //   - Can't have off-by-one errors like C-style for loops
    //
    // `&number` = pattern that destructures each element
    //   - numbers.iter() yields &i32 (references to i32)
    //   - The `&` in `&number` destructures the reference
    //   - So `number` is an i32 (value), not &i32 (reference)
    //   - This is called "pattern matching in binding"
    //
    //   Alternative ways to write this:
    //   ```
    //   for number_ref in numbers.iter() {
    //       let number = *number_ref;  // Explicit dereference
    //       if number > largest { ... }
    //   }
    //   ```
    //   But `&number` does the dereference automatically - cleaner!
    //
    // `in` = keyword meaning "in this collection"
    //
    // `numbers.iter()` = creates an iterator over the slice
    //   - `.iter()` = method that creates an iterator
    //   - Iterator is a trait (interface) for going through collections
    //   - Yields references to elements: &i32
    //   - This is a zero-cost abstraction!
    //   - Compiles down to efficient machine code
    //   - As fast as hand-written loop but safer
    //
    // Why use .iter()?
    // - It borrows each element (doesn't move them)
    // - More explicit than `for number in numbers` which would try to move
    // - For slices, `for number in numbers` wouldn't work (can't move from borrowed)
    // - .iter() is the idiomatic way to iterate over borrowed data

    for &number in numbers.iter() {
        // Now `number` is an i32 value (not a reference)
        // We can use it directly in comparisons

        // ====================================================================
        // UNDERSTANDING THE PATTERN: &number
        // ====================================================================
        //
        // `numbers.iter()` yields &i32 (references to i32s)
        // Each iteration, we get a reference to one element
        //
        // The pattern `&number` does this:
        // 1. Takes the &i32 that iter() gives us
        // 2. Dereferences it (follows the pointer)
        // 3. Copies the i32 value into `number`
        //
        // Why does this work?
        // - i32 implements the Copy trait
        // - Small, cheap to copy (just 4 bytes)
        // - Pattern matching can dereference and copy in one step
        //
        // If we were iterating over Strings, we couldn't do this!
        // String doesn't implement Copy, so we'd need:
        //   for string_ref in strings.iter() {
        //       // string_ref is &String, we can't copy it
        //   }

        // ====================================================================
        // COMPARISON AND UPDATE
        // ====================================================================

        // `if` = conditional check
        //
        // `number > largest` = comparison expression
        //   - `number` = current element we're looking at
        //   - `>` = greater than operator
        //   - `largest` = our current best guess
        //   - Returns bool (true or false)
        //
        // How does `>` work for i32?
        // - i32 implements the PartialOrd trait
        // - This trait provides comparison operators: <, >, <=, >=
        // - For integers, it's simple numeric comparison

        if number > largest {
            // We found a bigger number! Update our tracking variable.

            // `largest = number;` = assignment statement
            //   - `=` = assignment operator (not comparison!)
            //   - Takes the value from the right (number)
            //   - Stores it in the variable on the left (largest)
            //   - `;` = statement terminator
            //
            // What happens in memory?
            // - `number` is copied to `largest`
            // - Both are i32 (Copy type), so this is a simple copy
            // - Takes 4 bytes on stack
            // - Previous value of `largest` is forgotten (overwritten)

            largest = number;
        }

        // ====================================================================
        // ALGORITHM EXPLANATION
        // ====================================================================
        //
        // This is a linear search for the maximum value:
        //
        // Initial state: largest = numbers[0]
        //
        // For each number in the list:
        //   If number > largest:
        //     largest = number
        //
        // Final state: largest = the biggest number we saw
        //
        // Why this works:
        // 1. We start with the first number as our "best guess"
        // 2. We look at each remaining number
        // 3. Whenever we find a bigger number, that becomes our new "best guess"
        // 4. After checking all numbers, our "best guess" must be the largest
        //
        // Proof of correctness:
        // - We compare against every number (completeness)
        // - We only update when we find something bigger (correctness)
        // - At the end, largest ≥ every number we saw (maximality)
        //
        // Time Complexity: O(n)
        // - We look at each element exactly once
        // - n comparisons for n elements
        // - Linear time - can't do better (must look at all elements)
        //
        // Space Complexity: O(1)
        // - Only store one extra i32 (largest)
        // - Don't allocate any collections
        // - Constant space - very efficient!
        //
        // Comparison to other algorithms:
        // - Sorting would be O(n log n) time + O(n) space - overkill!
        // - Binary search wouldn't work (array not sorted)
        // - This is optimal for the unsorted case
    }

    // ========================================================================
    // STEP 4: RETURN THE RESULT
    // ========================================================================

    // After the loop, `largest` contains the biggest number we found

    // `Some(largest)` = construct the Some variant of Option<i32>
    //   - `Some` = variant of Option that holds a value
    //   - `(largest)` = the value to wrap (our largest number)
    //   - Creates Option::Some(largest)
    //
    // No semicolon! This is an expression, not a statement
    // The value of this expression is the return value of the function

    Some(largest)

    // What happens to the values when we return?
    // - `largest` is i32 (Copy type)
    // - It gets copied into the Some wrapper
    // - The Some(largest) gets returned to caller
    // - When function ends, our stack frame is cleaned up
    // - But the returned Option<i32> is copied to caller's stack

    // ============================================================================
    // OWNERSHIP ANALYSIS - DEEP DIVE
    // ============================================================================
    //
    // 1. `numbers: &[i32]` - We BORROW the slice
    //    - We don't own the data, just have a reference to it
    //    - Can't modify it (immutable borrow)
    //    - Can't outlive the data we're borrowing (enforced by borrow checker)
    //    - Caller keeps ownership of the original Vec or array
    //    - When function ends: The borrow ends, but data still exists in caller
    //
    // 2. `largest: i32` - This is an OWNED COPY
    //    - We copied an i32 from numbers[0]
    //    - i32 is a Copy type (primitive)
    //    - Each assignment (largest = number) copies the value
    //    - We own this i32 copy
    //    - When function ends: This stack space is reclaimed
    //
    // 3. `number` in the loop - Each iteration creates a COPY
    //    - The `&number` pattern copies the i32 from the reference
    //    - Each loop iteration creates and destroys this copy
    //    - Very cheap: i32 is only 4 bytes
    //    - All on the stack, no heap allocation
    //
    // 4. Return value `Option<i32>` - OWNED and returned
    //    - We create Some(largest) which wraps a copy of largest
    //    - This Option is an enum (stack-allocated)
    //    - Size: typically 8 bytes (4 for discriminant/padding, 4 for i32)
    //    - Ownership transfers to caller
    //    - Caller can use it, then it gets cleaned up automatically
    //
    // Memory efficiency:
    // - Input: 16 bytes (fat pointer to slice: pointer + length)
    // - Working memory: 4 bytes (one i32 for largest)
    // - Output: 8 bytes (Option<i32>)
    // - Total stack usage: ~28 bytes regardless of input size!
    // - No heap allocations at all
    // - This is what "zero-cost abstraction" means

    // ============================================================================
    // ZERO-COST ABSTRACTION EXAMPLE
    // ============================================================================
    //
    // This high-level Rust code:
    // ```rust
    // for &number in numbers.iter() {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // ```
    //
    // Compiles down to essentially the same assembly as this C code:
    // ```c
    // for (size_t i = 0; i < numbers_len; i++) {
    //     if (numbers[i] > largest) {
    //         largest = numbers[i];
    //     }
    // }
    // ```
    //
    // The Rust compiler optimizes away the iterator abstraction!
    // You get:
    // - Same performance as hand-written C
    // - But with safety guarantees:
    //   - Can't index out of bounds
    //   - Can't have data races
    //   - Can't forget to handle the empty case
    // - And better readability:
    //   - Intent is clear: "for each number"
    //   - No index variable to manage
    //   - No off-by-one errors possible
    //
    // This is "zero-cost abstraction": high-level code, low-level performance!

    // ============================================================================
    // THREE INPUT ITERATION TABLES
    // ============================================================================
    //
    // Example 1: Happy Path - Normal case with multiple numbers
    // Input: numbers = &[3, 7, 2, 9, 1]
    //
    // | Step | Variable       | Value     | Action                                |
    // |------|----------------|-----------|---------------------------------------|
    // | 1    | numbers        | &[3,7,2,9,1] | Function called with slice         |
    // | 2    | is_empty()     | false     | Check passes, continue                |
    // | 3    | largest        | 3         | Initialize with first element         |
    // | 4    | number (iter 1)| 3         | 3 > 3? No, skip                       |
    // | 5    | number (iter 2)| 7         | 7 > 3? Yes, largest = 7               |
    // | 6    | largest        | 7         | Updated                               |
    // | 7    | number (iter 3)| 2         | 2 > 7? No, skip                       |
    // | 8    | number (iter 4)| 9         | 9 > 7? Yes, largest = 9               |
    // | 9    | largest        | 9         | Updated                               |
    // | 10   | number (iter 5)| 1         | 1 > 9? No, skip                       |
    // | 11   | return         | Some(9)   | Wrap in Some and return               |
    //
    // Example 2: Edge Case - Empty slice
    // Input: numbers = &[]
    //
    // | Step | Variable       | Value     | Action                                |
    // |------|----------------|-----------|---------------------------------------|
    // | 1    | numbers        | &[]       | Function called with empty slice      |
    // | 2    | is_empty()     | true      | Check succeeds                        |
    // | 3    | return         | None      | Early return, skip rest of function   |
    //
    // Example 3: Edge Case - All negative numbers
    // Input: numbers = &[-5, -1, -10]
    //
    // | Step | Variable       | Value     | Action                                |
    // |------|----------------|-----------|---------------------------------------|
    // | 1    | numbers        | &[-5,-1,-10] | Function called                    |
    // | 2    | is_empty()     | false     | Check passes                          |
    // | 3    | largest        | -5        | Initialize with first                 |
    // | 4    | number (iter 1)| -5        | -5 > -5? No, skip                     |
    // | 5    | number (iter 2)| -1        | -1 > -5? Yes, largest = -1            |
    // | 6    | largest        | -1        | Updated                               |
    // | 7    | number (iter 3)| -10       | -10 > -1? No, skip                    |
    // | 8    | return         | Some(-1)  | -1 is the largest (least negative)    |
    //
    // Example 4: Type System Prevents This Bug
    // ```rust
    // let numbers = vec![1, 2, 3];
    // let largest = find_largest(&numbers);
    // drop(numbers);  // Free the vector
    // match largest {
    //     Some(n) => println!("{}", n),  // ✅ OK! n is a Copy, independent of numbers
    //     None => println!("None"),
    // }
    // ```
    //
    // Compare to this bug in C:
    // ```c
    // int* largest = find_largest(numbers);
    // free(numbers);  // Free the array
    // printf("%d", *largest);  // ❌ DANGLING POINTER! Use-after-free!
    // ```
    //
    // Rust prevents this because:
    // - We return Option<i32>, not &i32
    // - The i32 is copied, not referenced
    // - No way to have a dangling reference
    // - Even if you tried to return &i32, compiler would stop you:
    //   "cannot return reference to local data"

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // Python:
    // ```python
    // def find_largest(numbers: list[int]) -> int | None:
    //     if not numbers:
    //         return None
    //     return max(numbers)
    // ```
    // Issues:
    // - Type hints are optional, not enforced
    // - Could pass [1, "two", 3] → runtime error
    // - Could forget to check None → AttributeError at runtime
    // - Much slower (interpreted, dynamic typing)
    //
    // JavaScript:
    // ```javascript
    // function findLargest(numbers) {
    //     if (numbers.length === 0) return null;
    //     return Math.max(...numbers);
    // }
    // ```
    // Issues:
    // - null is a pain (need null checks everywhere)
    // - Could pass [1, "two", 3] → returns NaN (not a number)
    // - No type safety at all
    // - Math.max with spread can hit call stack size limit on large arrays
    //
    // Go:
    // ```go
    // func findLargest(numbers []int) (int, bool) {
    //     if len(numbers) == 0 {
    //         return 0, false
    //     }
    //     largest := numbers[0]
    //     for _, n := range numbers {
    //         if n > largest {
    //             largest = n
    //         }
    //     }
    //     return largest, true
    // }
    // ```
    // Better! Has types and similar logic. But:
    // - Still can have nil slice panics in other contexts
    // - Tuple return (int, bool) is less clear than Option<int>
    // - Uses garbage collector (slight overhead)
    //
    // C++:
    // ```cpp
    // std::optional<int> find_largest(const std::vector<int>& numbers) {
    //     if (numbers.empty()) return std::nullopt;
    //     return *std::max_element(numbers.begin(), numbers.end());
    // }
    // ```
    // Pretty good! C++17 added std::optional. But:
    // - Still possible to have undefined behavior in complex code
    // - No borrow checker to prevent data races
    // - Manual memory management elsewhere
    //
    // Rust advantages:
    // - Type-safe: Won't compile with wrong types
    // - Memory-safe: Can't have buffer overflows
    // - No null: Uses Option instead
    // - Explicit error handling: Compiler forces you to handle None
    // - Zero-cost: As fast as hand-written C
    // - Thread-safe: Borrow checker prevents data races
    // - No garbage collector: Deterministic performance
}

/// Counts the number of vowels in a string.
///
/// ## What This Function Does
///
/// Goes through each character in a string and counts how many are vowels
/// (a, e, i, o, u). The check is case-insensitive, so 'A' and 'a' both count.
///
/// ## Rust Concepts Demonstrated
///
/// - **String iteration**: Different ways to go through characters
/// - **Character comparison**: How to check what character we have
/// - **Case conversion**: to_lowercase() for case-insensitive matching
/// - **Pattern matching**: Using matches! macro for elegant checks
/// - **Mutability**: Counting with a mutable counter
///
/// ## Parameters
///
/// - `text: &str` - A borrowed string slice
///   - We don't need to own it, just read it
///   - Works with string literals and borrowed Strings
///   - Can't modify the text (immutable borrow)
///
/// ## Returns
///
/// The count of vowels as `usize`
/// - usize is an unsigned integer that matches pointer size
/// - On 64-bit systems: u64 (8 bytes)
/// - On 32-bit systems: u32 (4 bytes)
/// - Perfect for sizes, counts, indexes
/// - Can't be negative (unsigned)
///
/// ## Example
/// ```ignore
/// assert_eq!(count_vowels("hello"), 2);      // e, o
/// assert_eq!(count_vowels("AEIOU"), 5);      // All vowels, uppercase
/// assert_eq!(count_vowels("xyz"), 0);        // No vowels
/// assert_eq!(count_vowels(""), 0);           // Empty string
/// assert_eq!(count_vowels("rhythm"), 0);     // No vowels (y doesn't count)
/// ```ignore
pub fn count_vowels(text: &str) -> usize {
    // ========================================================================
    // STEP 1: INITIALIZE COUNTER
    // ========================================================================

    // `let mut count = 0;` = declare a mutable counter variable
    //   - `let` = variable declaration keyword
    //   - `mut` = mutable (we'll increment this)
    //   - `count` = our variable name
    //   - `= 0` = initialize to zero
    //   - Type is inferred as usize (from function return type)
    //   - `;` = statement terminator

    let mut count = 0;

    // Why start at 0?
    // - We haven't found any vowels yet
    // - We'll increment for each vowel we find
    // - If text is empty, loop won't run, we return 0 (correct!)

    // ========================================================================
    // STEP 2: ITERATE THROUGH CHARACTERS
    // ========================================================================

    // `for ch in text.chars()` = iterate over each character
    //   - `for` = loop keyword
    //   - `ch` = variable name for each character (our choice)
    //   - `in` = keyword meaning "in this sequence"
    //   - `text` = our &str parameter
    //   - `.chars()` = method that returns an iterator over characters
    //     - Yields Unicode scalar values (proper characters)
    //     - Handles multi-byte UTF-8 sequences correctly
    //     - Each `ch` is of type `char` (4 bytes, Unicode scalar)
    //
    // Why .chars() instead of .bytes()?
    // - .bytes() gives you raw bytes (u8)
    // - For ASCII text, one char = one byte
    // - But for Unicode, one char can be multiple bytes!
    // - Example: "🦀" (Rust crab) is 4 bytes in UTF-8
    // - .chars() handles this correctly, giving you the whole character
    // - .bytes() would give you 4 separate bytes (wrong!)

    for ch in text.chars() {
        // `ch` is now a single character (char type)
        // char in Rust is a Unicode scalar value (4 bytes)

        // ====================================================================
        // STEP 3: CONVERT TO LOWERCASE FOR CASE-INSENSITIVE MATCHING
        // ====================================================================

        // `.to_lowercase()` = method on char
        //   - Converts the character to lowercase
        //   - Returns an iterator over the lowercase character(s)
        //   - Usually just one char, but some languages have special cases
        //   - Example: German 'ß' remains 'ß', but 'SS' → 'ss'
        //
        // `.next()` = gets the first (and usually only) character
        //   - Returns Option<char>
        //   - Some(lowercase_char) if there is one
        //   - None if... well, there's always one for normal chars
        //
        // `.unwrap()` = extracts the char from Some(char)
        //   - Panics if None (but that won't happen here)
        //   - Safe in this context because chars always have lowercase forms
        //
        // Why not just `ch.to_lowercase()`?
        // - to_lowercase() returns an iterator, not a char
        // - This is because some chars lowercase to multiple chars
        // - For English vowels, it's always one char
        // - We need .next().unwrap() to get the actual char

        let ch_lower = ch.to_lowercase().next().unwrap();

        // Alternative approach (more verbose but clearer):
        // ```rust
        // let lowercase_iter = ch.to_lowercase();
        // let ch_lower = lowercase_iter.next().unwrap();
        // ```

        // ====================================================================
        // STEP 4: CHECK IF CHARACTER IS A VOWEL
        // ====================================================================

        // `matches!` = macro for pattern matching that returns bool
        //   - Returns true if the pattern matches
        //   - Returns false otherwise
        //   - More concise than match expression for boolean checks
        //
        // `ch_lower` = the character we're checking
        //
        // `'a' | 'e' | 'i' | 'o' | 'u'` = pattern with multiple alternatives
        //   - `'a'` = character literal (single char in single quotes)
        //   - `|` = "or" in pattern matching (not boolean or!)
        //   - Matches if ch_lower is any of these five vowels
        //
        // Why lowercase only?
        // - We already converted to lowercase above
        // - So we only need to check 'a', 'e', 'i', 'o', 'u'
        // - Don't need to also check 'A', 'E', 'I', 'O', 'U'

        if matches!(ch_lower, 'a' | 'e' | 'i' | 'o' | 'u') {
            // We found a vowel! Increment our counter.

            // `count += 1;` = shorthand for `count = count + 1;`
            //   - `+=` = add and assign operator
            //   - Adds 1 to count
            //   - Stores result back in count
            //   - Requires count to be mutable (mut)

            count += 1;

            // What happens in memory?
            // - count is read from stack
            // - 1 is added to it
            // - Result is written back to count's location on stack
            // - Very fast operation (single CPU instruction)
        }

        // Alternative ways to check for vowel:
        //
        // 1. Using match expression:
        // ```rust
        // match ch_lower {
        //     'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
        //     _ => {}  // Do nothing for non-vowels
        // }
        // ```
        //
        // 2. Using contains:
        // ```rust
        // if "aeiou".contains(ch_lower) {
        //     count += 1;
        // }
        // ```
        //
        // 3. Using comparison chain:
        // ```rust
        // if ch_lower == 'a' || ch_lower == 'e' || ch_lower == 'i' ||
        //    ch_lower == 'o' || ch_lower == 'u' {
        //     count += 1;
        // }
        // ```
        //
        // The matches! macro is most concise and idiomatic
    }

    // ========================================================================
    // STEP 5: RETURN THE COUNT
    // ========================================================================

    // After the loop, `count` contains the total number of vowels found

    // `count` = just the variable name, no semicolon
    //   - This is an expression
    //   - Its value is the return value of the function
    //   - count is usize, matching our return type

    count

    // Alternative (explicit return):
    // return count;
    // But implicit return (no semicolon) is more idiomatic in Rust

    // ============================================================================
    // OWNERSHIP ANALYSIS
    // ============================================================================
    //
    // 1. `text: &str` - BORROWED string slice
    //    - We don't own the string data
    //    - Just have a read-only reference to it
    //    - Can't modify it
    //    - Can't outlive it (borrow checker ensures this)
    //    - Caller keeps ownership
    //
    // 2. `count: usize` - OWNED integer
    //    - We own this value
    //    - It's on our stack frame
    //    - Gets copied to caller on return (usize is Copy)
    //    - Our stack frame is cleaned up after return
    //
    // 3. `ch: char` - COPIED in each iteration
    //    - char implements Copy trait
    //    - Each iteration copies one char from the iterator
    //    - char is 4 bytes (Unicode scalar value)
    //    - Very cheap to copy
    //
    // 4. `ch_lower: char` - OWNED copy
    //    - Created from to_lowercase()
    //    - Owned by this scope (the loop body)
    //    - Dropped at end of each loop iteration
    //
    // Memory efficiency:
    // - Input: 16 bytes (fat pointer: ptr + len)
    // - count: 8 bytes (usize on 64-bit)
    // - ch: 4 bytes (char)
    // - ch_lower: 4 bytes (char)
    // - Total: ~32 bytes on stack regardless of input length!
    // - No heap allocations
    // - Very efficient

    // ============================================================================
    // TIME AND SPACE COMPLEXITY
    // ============================================================================
    //
    // Time Complexity: O(n) where n is the length of the string
    // - We visit each character exactly once
    // - For each character:
    //   - to_lowercase(): O(1) for most characters
    //   - matches! check: O(1) (constant time comparison)
    //   - count += 1: O(1)
    // - Total: O(n)
    //
    // Space Complexity: O(1)
    // - Only allocate a fixed amount of stack space (count, ch, ch_lower)
    // - Don't create any collections or dynamic allocations
    // - Doesn't matter how long the input string is
    // - Constant space usage
    //
    // This is optimal! Can't count vowels without looking at each character.

    // ============================================================================
    // THREE INPUT ITERATION TABLES
    // ============================================================================
    //
    // Example 1: Happy Path - "hello"
    //
    // | Iteration | ch  | ch_lower | Is Vowel? | count | Notes              |
    // |-----------|-----|----------|-----------|-------|--------------------|
    // | Start     | -   | -        | -         | 0     | Initialize         |
    // | 1         | 'h' | 'h'      | No        | 0     | Consonant, skip    |
    // | 2         | 'e' | 'e'      | Yes       | 1     | Vowel! Increment   |
    // | 3         | 'l' | 'l'      | No        | 1     | Consonant, skip    |
    // | 4         | 'l' | 'l'      | No        | 1     | Consonant, skip    |
    // | 5         | 'o' | 'o'      | Yes       | 2     | Vowel! Increment   |
    // | End       | -   | -        | -         | 2     | Return 2           |
    //
    // Example 2: Edge Case - "AEIOU" (all uppercase vowels)
    //
    // | Iteration | ch  | ch_lower | Is Vowel? | count | Notes              |
    // |-----------|-----|----------|-----------|-------|--------------------|
    // | Start     | -   | -        | -         | 0     | Initialize         |
    // | 1         | 'A' | 'a'      | Yes       | 1     | Uppercase converted|
    // | 2         | 'E' | 'e'      | Yes       | 2     | Uppercase converted|
    // | 3         | 'I' | 'i'      | Yes       | 3     | Uppercase converted|
    // | 4         | 'O' | 'o'      | Yes       | 4     | Uppercase converted|
    // | 5         | 'U' | 'u'      | Yes       | 5     | Uppercase converted|
    // | End       | -   | -        | -         | 5     | Return 5           |
    //
    // Example 3: Edge Case - "" (empty string)
    //
    // | Iteration | ch  | ch_lower | Is Vowel? | count | Notes              |
    // |-----------|-----|----------|-----------|-------|--------------------|
    // | Start     | -   | -        | -         | 0     | Initialize         |
    // | End       | -   | -        | -         | 0     | Loop never runs, return 0 |
    //
    // Example 4: Edge Case - "xyz" (no vowels)
    //
    // | Iteration | ch  | ch_lower | Is Vowel? | count | Notes              |
    // |-----------|-----|----------|-----------|-------|--------------------|
    // | Start     | -   | -        | -         | 0     | Initialize         |
    // | 1         | 'x' | 'x'      | No        | 0     | Consonant          |
    // | 2         | 'y' | 'y'      | No        | 0     | y is not a vowel   |
    // | 3         | 'z' | 'z'      | No        | 0     | Consonant          |
    // | End       | -   | -        | -         | 0     | Return 0           |

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // Python:
    // ```python
    // def count_vowels(text: str) -> int:
    //     return sum(1 for ch in text.lower() if ch in 'aeiou')
    // ```
    // Issues:
    // - Type hints not enforced
    // - Much slower (interpreted)
    // - text.lower() creates a new string (O(n) space)
    //
    // JavaScript:
    // ```javascript
    // function countVowels(text) {
    //     return (text.toLowerCase().match(/[aeiou]/g) || []).length;
    // }
    // ```
    // Issues:
    // - No type checking
    // - toLowerCase() creates new string
    // - RegEx has parsing overhead
    // - Need || [] workaround for null
    //
    // Go:
    // ```go
    // func countVowels(text string) int {
    //     count := 0
    //     for _, ch := range strings.ToLower(text) {
    //         if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
    //             count++
    //         }
    //     }
    //     return count
    // }
    // ```
    // Issues:
    // - ToLower() creates a new string (allocates memory)
    // - Less concise than Rust's matches! macro
    //
    // C:
    // ```c
    // size_t count_vowels(const char* text) {
    //     size_t count = 0;
    //     for (; *text; text++) {
    //         char ch = tolower(*text);
    //         if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u') {
    //             count++;
    //         }
    //     }
    //     return count;
    // }
    // ```
    // Issues:
    // - No UTF-8 support (breaks on multi-byte characters)
    // - tolower() only works for ASCII
    // - Could have buffer overflow if text not null-terminated
    // - No safety guarantees
    //
    // Rust advantages:
    // - Proper Unicode handling with .chars()
    // - No string allocation needed (we convert chars individually)
    // - Type-safe
    // - Memory-safe
    // - Zero-cost abstraction (compiles to efficient code)
    // - Can't have buffer overflows
    // - matches! macro is concise and clear
}

// ============================================================================
// GENERAL RUST PRINCIPLES DEMONSTRATED
// ============================================================================
//
// 1. **Immutability by Default**
//    - Variables are immutable unless marked `mut`
//    - Prevents accidental modifications
//    - Makes code easier to reason about
//    - Opt-in to mutability when needed
//
// 2. **Ownership System**
//    - Every value has a single owner
//    - When owner goes out of scope, value is dropped
//    - Can borrow (immutable) or borrow mutably
//    - Prevents dangling pointers, double frees, data races
//
// 3. **No Null**
//    - Option<T> instead of null
//    - Compiler forces you to handle None case
//    - No null pointer exceptions possible
//    - Explicit about "might not exist"
//
// 4. **Expression-Oriented**
//    - Last expression in block is its value
//    - No semicolon on return values
//    - More concise than explicit returns
//    - Everything is an expression (except let/;)
//
// 5. **Zero-Cost Abstractions**
//    - High-level code compiles to low-level efficiency
//    - Iterators optimize away
//    - No runtime cost for safety
//    - As fast as C, safer than anything
//
// 6. **Type Inference**
//    - Compiler figures out types
//    - Still statically typed (checked at compile time)
//    - Less verbose than Java/C++
//    - More safety than Python/JavaScript
//
// 7. **Memory Safety**
//    - No buffer overflows
//    - No dangling pointers
//    - No data races
//    - No segfaults
//    - All checked at compile time!
//
// This is why Rust is the future of systems programming! 🦀
