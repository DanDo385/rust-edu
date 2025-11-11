# Rust Basics Quick Reference

This is your cheat sheet for common Rust syntax you'll see in these projects.

## Variable Declaration

```rust
// Immutable by default (can't change)
let x = 5;
// x = 6;  // ❌ ERROR - x is immutable

// Mutable (can change)
let mut y = 5;
y = 6;  // ✅ OK - y is mutable

// Type annotation (usually not needed, Rust infers types)
let z: i32 = 5;  // i32 = 32-bit integer
```

**Why immutable by default?**
- Prevents accidental changes
- Makes code easier to reason about
- Enables compiler optimizations
- You opt-in to mutability when you need it

## Ownership Basics

```rust
// OWNING
let s1 = String::from("hello");  // s1 OWNS this string
let s2 = s1;                     // Ownership MOVES to s2
// println!("{}", s1);            // ❌ ERROR - s1 no longer valid

// BORROWING (immutable)
let s1 = String::from("hello");
let s2 = &s1;                    // s2 BORROWS s1 (can read, can't modify)
println!("{}", s1);              // ✅ Still works - s1 still owns it

// BORROWING (mutable)
let mut s1 = String::from("hello");
let s2 = &mut s1;                // s2 BORROWS s1 mutably (can modify)
s2.push_str(" world");           // Modifying through s2
// println!("{}", s1);            // ❌ ERROR - can't use s1 while s2 borrows it
```

**The Rules of Ownership:**
1. Each value has one owner
2. When the owner goes out of scope, the value is dropped
3. You can have either:
   - One mutable reference (`&mut T`)
   - Any number of immutable references (`&T`)
   - But not both at the same time!

## Common Types

```rust
// Numbers
let a: i32 = 5;        // 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
let b: u32 = 5;        // 32-bit unsigned integer (0 to 4,294,967,295)
let c: i64 = 5;        // 64-bit signed integer
let d: usize = 5;      // Pointer-sized unsigned integer (32 or 64 bit depending on architecture)
let e: f64 = 3.14;     // 64-bit floating point

// Strings
let s1: &str = "hello";           // String slice (borrowed, often in binary)
let s2: String = String::from("hello");  // Owned string (on heap)

// Boolean
let t: bool = true;
let f: bool = false;

// Collections
let v: Vec<i32> = vec![1, 2, 3];  // Vector (growable array)
use std::collections::HashMap;
let mut map = HashMap::new();      // Hash map (key-value pairs)
map.insert("key", "value");
```

## Functions

```rust
// Basic function
fn add(a: i32, b: i32) -> i32 {  // -> i32 means returns i32
    a + b  // No semicolon = return value (last expression)
}

// Function with explicit return
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;  // Works, but less idiomatic
}

// Function with no return value
fn print_hello() {  // No -> means returns () (unit type, like void)
    println!("Hello!");
}

// Function with early return
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;  // Early return
    }
    Some(a / b)  // Last expression is returned
}
```

## Structs

```rust
// Define a struct (like a class in other languages)
struct Person {
    name: String,
    age: u32,
}

// Create an instance
let p = Person {
    name: String::from("Alice"),
    age: 30,
};

// Access fields
println!("{} is {}", p.name, p.age);

// Tuple struct (struct with unnamed fields)
struct Point(i32, i32);
let origin = Point(0, 0);
println!("x: {}, y: {}", origin.0, origin.1);
```

## Impl Blocks (Methods)

```rust
impl Person {
    // Associated function (like static method) - no self
    // Called with Person::new(...)
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    // Method (has &self) - borrows instance immutably
    // Called with person.greet()
    fn greet(&self) {
        println!("Hi, I'm {}", self.name);
    }

    // Method (has &mut self) - borrows mutably
    // Called with person.have_birthday()
    fn have_birthday(&mut self) {
        self.age += 1;
    }

    // Method that consumes self (takes ownership)
    // Called with person.into_json()
    fn into_json(self) -> String {
        format!("{{\"name\":\"{}\",\"age\":{}}}", self.name, self.age)
        // After this, person can't be used anymore (it was consumed)
    }
}

// Usage
let mut p = Person::new(String::from("Bob"), 25);
p.greet();           // Borrows p
p.have_birthday();   // Borrows p mutably
let json = p.into_json();  // Consumes p - can't use p after this!
```

## Option and Result

```rust
// Option - represents "maybe has a value"
let some_number: Option<i32> = Some(5);  // Has a value
let no_number: Option<i32> = None;       // No value

// Matching on Option
match some_number {
    Some(n) => println!("Got: {}", n),
    None => println!("Got nothing"),
}

// Using if let (when you only care about one case)
if let Some(n) = some_number {
    println!("Got: {}", n);
}

// Unwrapping (dangerous! panics if None)
let value = some_number.unwrap();  // Gets 5, but panics if None

// Safe alternatives
let value = some_number.unwrap_or(0);        // Returns 0 if None
let value = some_number.expect("No value!"); // Panics with custom message if None

// Result - represents success or error
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Can't divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Using Result
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}

// Using Result with ?
fn calculate() -> Result<i32, String> {
    let x = divide(10, 2)?;  // ? returns early if error
    let y = divide(x, 0)?;   // This will return the error
    Ok(y)
}
```

## Loops and Iteration

```rust
// loop - infinite loop
loop {
    println!("Forever!");
    break;  // Exit the loop
}

// while - conditional loop
let mut count = 0;
while count < 5 {
    println!("{}", count);
    count += 1;
}

// for - iterate over a range
for i in 0..5 {  // 0, 1, 2, 3, 4 (excludes 5)
    println!("{}", i);
}

for i in 0..=5 {  // 0, 1, 2, 3, 4, 5 (includes 5)
    println!("{}", i);
}

// for - iterate over a collection
let numbers = vec![1, 2, 3, 4, 5];
for num in &numbers {  // Borrow each element
    println!("{}", num);
}

for num in &mut numbers {  // Borrow mutably
    *num *= 2;  // Dereference to modify
}

for num in numbers {  // Take ownership (can't use numbers after)
    println!("{}", num);
}
```

## Pattern Matching

```rust
let number = 3;

// match expression (must be exhaustive)
match number {
    1 => println!("One"),
    2 | 3 => println!("Two or three"),  // Multiple patterns
    4..=10 => println!("Four through ten"),  // Range
    _ => println!("Something else"),  // Catch-all
}

// Matching with values
let result = match number {
    1 => "one",
    2 => "two",
    _ => "many",
};

// Matching structs
struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };

match p {
    Point { x: 0, y: 0 } => println!("Origin"),
    Point { x: 0, y } => println!("On Y axis at {}", y),
    Point { x, y: 0 } => println!("On X axis at {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

## Closures (Anonymous Functions)

```rust
// Closure syntax: |parameters| body
let add_one = |x| x + 1;
println!("{}", add_one(5));  // 6

// With type annotations
let add_one: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };

// Closures can capture environment
let y = 10;
let add_y = |x| x + y;  // Captures y
println!("{}", add_y(5));  // 15

// Using closures with iterators
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
```

## Common Symbols

- `::` - Path separator (like `.` in other languages for static things)
  - `String::from("hello")` - calling associated function
  - `std::collections::HashMap` - accessing module
- `&` - Borrow (take a reference)
  - `&x` - immutable reference to x
  - `&mut x` - mutable reference to x
- `*` - Dereference (get the value from a reference)
  - `*reference` - get the value behind the reference
- `?` - Propagate error (return early if Result is Err or Option is None)
  - Only works in functions that return Result or Option
- `!` - Two meanings:
  - Macro call: `println!("hello")` - the ! means it's a macro
  - Never type: `fn panic() -> !` - this function never returns
- `|x|` - Closure parameter (like lambda in Python/JS)
  - `|x| x * 2` - anonymous function that doubles x
- `<T>` - Generic type parameter
  - `Vec<i32>` - Vec containing i32s
  - `Option<String>` - Option containing a String
- `..` - Range (exclusive)
  - `0..5` - numbers 0, 1, 2, 3, 4
- `..=` - Range (inclusive)
  - `0..=5` - numbers 0, 1, 2, 3, 4, 5
- `_` - Wildcard/ignore
  - In match: catch-all pattern
  - In let: ignore value `let _ = some_function();`

## Macros

```rust
// Common macros (note the ! at the end)
println!("Hello, {}!", "world");  // Print with newline
print!("No newline");              // Print without newline
eprintln!("Error!");               // Print to stderr

vec![1, 2, 3]                      // Create a vector
format!("x = {}", x)               // Create a formatted string (like sprintf)

panic!("Something went wrong!");   // Crash the program with a message
assert_eq!(a, b);                  // Assert two values are equal (panics if not)
assert!(condition);                // Assert condition is true (panics if not)

todo!()                            // Mark unimplemented code (panics if called)
unimplemented!()                   // Same as todo!
```

## Attributes

```rust
// Attributes modify the next item
#[derive(Debug, Clone)]   // Auto-implement Debug and Clone traits
struct Point { x: i32, y: i32 }

#[cfg(test)]              // Only compile in test mode
mod tests {
    #[test]               // This function is a test
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[allow(dead_code)]       // Suppress "unused" warnings
fn helper() {}
```

## Common Traits

```rust
// Debug - print with {:?}
#[derive(Debug)]
struct Point { x: i32, y: i32 }
println!("{:?}", point);

// Clone - create a deep copy
let s1 = String::from("hello");
let s2 = s1.clone();  // Both s1 and s2 are valid

// Copy - types that can be copied implicitly
// Integers, floats, booleans, chars implement Copy
let x = 5;
let y = x;  // x is copied, not moved (both valid)

// PartialEq - compare with ==
if x == y { }

// Default - create a default value
#[derive(Default)]
struct Config { verbose: bool }
let config = Config::default();
```

## Module System

```rust
// In lib.rs or main.rs
mod my_module;           // Looks for my_module.rs or my_module/mod.rs

// Inline module
mod my_module {
    pub fn public_function() {}
    fn private_function() {}
}

// Using items
use std::collections::HashMap;
use std::io::{self, Write};  // Import multiple items

// Re-exporting
pub use my_module::public_function;
```

## Memory Locations Quick Reference

**Stack** (fast, fixed size, automatically managed):
- Integers, floats, booleans, chars
- Pointers and references
- Fixed-size arrays
- Struct/enum values (but they might contain pointers to heap data)

**Heap** (slower, dynamic size, ownership-managed):
- String (owns heap data)
- Vec<T> (owns heap array)
- Box<T> (owned heap pointer)
- Rc<T>, Arc<T> (reference counted heap data)

```rust
let x = 5;                    // Stack: integer value
let s = String::from("hi");   // Stack: pointer+length+capacity
                              // Heap: actual "hi" characters
let v = vec![1, 2, 3];       // Stack: pointer+length+capacity
                              // Heap: actual [1, 2, 3] array
```

---

## Quick Tips

1. **Start with immutable** - only use `mut` when needed
2. **Borrow instead of own** - use `&T` instead of `T` when you don't need ownership
3. **Read compiler errors** - Rust's compiler errors are very helpful!
4. **Use `cargo clippy`** - get helpful suggestions
5. **Use `cargo fmt`** - auto-format your code
6. **When stuck on ownership** - draw a diagram of who owns what

## Common Beginner Questions

**Q: Why can't I use a value after moving it?**
A: Rust prevents double-free bugs. Once moved, the old variable is invalid.

**Q: When should I use String vs &str?**
A: Use String when you need to own/modify text. Use &str when just reading.

**Q: Why so many different integer types?**
A: Performance and correctness. Use the right size for your data. `i32` is a good default.

**Q: What's the difference between panic and Result?**
A: panic crashes the program. Result lets the caller handle errors. Use Result for recoverable errors.

**Q: How do I print a struct?**
A: Add `#[derive(Debug)]` above it, then use `println!("{:?}", my_struct);`

---

Keep this file open while working through the projects! 🦀
