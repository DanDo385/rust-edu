# Project 02 - Ownership and Borrowing

## What You're Building (Plain English)

Welcome to Rust's superpower! This project teaches you about **ownership**, **borrowing**, and **moving**—the concepts that make Rust unique. These rules prevent entire classes of bugs (use-after-free, double-free, data races) that plague C/C++ and confuse JavaScript/Python developers. By the end, you'll understand why Rust doesn't need a garbage collector but is still memory-safe.

## New Rust Concepts in This Project

- **Ownership**: Every value in Rust has a single owner. When the owner goes out of scope, the value is dropped (freed). This is Rust's core principle!

- **Move semantics**: When you assign a non-Copy type (like String) to another variable, ownership moves. The original variable becomes invalid. No copying, no garbage collection—just a transfer of responsibility.

- **Borrowing**: You can lend out references to data without transferring ownership. Like lending a book to a friend—you still own it, they just use it temporarily.

- **Immutable borrows (&T)**: Multiple readers allowed. Can't modify the data. Think: read-only access.

- **Mutable borrows (&mut T)**: Exactly one writer at a time. No other borrows allowed while it exists. Prevents data races at compile time!

- **Copy vs Move**: Types like i32, f64 implement Copy—they're duplicated automatically. Types like String, Vec don't—they move to prevent expensive accidental copies.

## Rust Syntax You'll See

```rust
// OWNERSHIP - Moves
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2, s1 is now invalid
// println!("{}", s1);  // ❌ ERROR: value borrowed here after move

// IMMUTABLE BORROWING - Read-only references
let s = String::from("hello");
let len = calculate_length(&s);  // &s borrows s
println!("{} {}", s, len);  // ✅ s is still valid!

fn calculate_length(s: &String) -> usize {
    s.len()  // Can read, can't modify
}

// MUTABLE BORROWING - Exclusive write access
let mut s = String::from("hello");
change(&mut s);  // &mut s gives mutable borrow

fn change(s: &mut String) {
    s.push_str(", world");  // Can modify!
}

// COPY TYPES - Automatic duplication
let x = 5;
let y = x;  // x is COPIED to y, both are valid
println!("{} {}", x, y);  // ✅ Both work!

// BORROWING RULES (enforced at compile time!)
let mut s = String::from("hello");
let r1 = &s;     // ✅ immutable borrow
let r2 = &s;     // ✅ another immutable borrow (multiple readers OK)
let r3 = &mut s; // ❌ ERROR: can't borrow as mutable while immutable borrows exist

// But this works (borrows don't overlap):
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);  // r1 and r2 last used here
let r3 = &mut s;  // ✅ OK! Previous borrows ended
```

## How to Run

```bash
# Run the main binary (executes src/main.rs)
cargo run -p ownership-borrowing

# Run the tests
cargo test -p ownership-borrowing

# Run tests with output visible
cargo test -p ownership-borrowing -- --nocapture

# Check if code compiles without running
cargo check -p ownership-borrowing

# Format your code
cargo fmt -p ownership-borrowing
```

## The Exercises

You'll implement four functions that demonstrate ownership and borrowing:

1. **add_exclamation**: Takes ownership of a String, modifies it, returns it
   - Learn about: Taking ownership, modifying owned data
   - Example: "Hello" → "Hello!"
   - Why: Shows how ownership transfer works

2. **get_length**: Borrows a String immutably, returns its length
   - Learn about: Immutable borrowing (&String), reading without owning
   - Example: &"Hello" → 5
   - Why: Shows you can use data without taking ownership

3. **make_uppercase**: Borrows a String mutably, modifies it in place
   - Learn about: Mutable borrowing (&mut String), in-place modification
   - Example: &mut "hello" → mutates to "HELLO"
   - Why: Shows how to modify data you don't own

4. **demonstrate_copy_vs_move**: Shows difference between Copy and Move types
   - Learn about: i32 (Copy) vs String (Move), cloning
   - Returns: A formatted report of what happens
   - Why: Crystallizes understanding of when data is copied vs moved

## Solution Explanation (No Code - Just Ideas)

**add_exclamation**:
- Takes `s: String` (ownership transfer, no &)
- Caller can't use the String anymore after calling this
- We own it, so we can modify it
- Use `.push('!')` or `.push_str("!")` to add exclamation
- Return the String (ownership transfers to caller)
- The caller gets back the modified String

**get_length**:
- Takes `s: &String` (immutable borrow, no ownership)
- We can read the string but can't modify it
- Call `.len()` method to get length
- Return the usize length
- Original String in caller is still valid after function returns

**make_uppercase**:
- Takes `s: &mut String` (mutable borrow)
- We don't own it, but we can modify it
- Convert the string to uppercase in place
- Use a method like `s.clear()` + `s.push_str(&original.to_uppercase())`
- Or replace the string's contents with uppercase version
- No return value—we modify in place!

**demonstrate_copy_vs_move**:
- Show how i32 is Copy: assigning doesn't invalidate original
- Show how String is Move: assigning invalidates original
- Show how to clone String if you want to copy
- Return a String explaining what happened
- Use format! to build the explanation

## Where Rust Shines

**Compared to C/C++:**
```c
// C++ - Manual memory management, easy to mess up
std::string* create_string() {
    std::string* s = new std::string("hello");
    return s;  // Caller must remember to delete!
}

void use_string(std::string* s) {
    delete s;  // Free memory
}

int main() {
    std::string* s = create_string();
    use_string(s);
    std::cout << *s << std::endl;  // ❌ USE AFTER FREE! Undefined behavior!
    return 0;
}
```

**Rust version:**
```rust
// Rust - Ownership prevents use-after-free at compile time!
fn create_string() -> String {
    String::from("hello")  // Ownership transfers to caller
}

fn use_string(s: String) {
    // s is dropped here when function ends
}

fn main() {
    let s = create_string();
    use_string(s);  // s is moved into function
    // println!("{}", s);  // ❌ WON'T COMPILE: value used after move
}
```

**Compared to JavaScript/Python:**
```javascript
// JavaScript - Everything is references, surprising behavior
let obj1 = { value: 10 };
let obj2 = obj1;  // Both refer to same object
obj2.value = 20;
console.log(obj1.value);  // 20 (surprise! obj1 changed too)
```

```rust
// Rust - Clear about moves
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED, no longer valid
// let s3 = s1;  // ❌ Won't compile!

// If you want a copy, be explicit:
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit copy
let s3 = s1;  // Can still use s1 (moved after the clone)
```

**Why this matters:**
- **Memory safety without garbage collection**: Ownership rules prevent leaks and use-after-free
- **No hidden costs**: Cloning is explicit. You see when data is duplicated.
- **Fearless concurrency**: These same rules prevent data races in multithreaded code
- **Zero runtime cost**: All checks happen at compile time. No garbage collector pauses!

## Common Beginner Mistakes & How to Avoid Them

1. **Trying to use a value after it's moved**
   ```rust
   let s1 = String::from("hello");
   let s2 = s1;
   println!("{}", s1);  // ❌ ERROR: value borrowed here after move
   ```
   **Fix:** Use `s1.clone()` if you need both, or use `&s1` to borrow instead
   **Why it happens:** Coming from languages where assignment copies

2. **Returning a reference to local data**
   ```rust
   fn dangle() -> &String {
       let s = String::from("hello");
       &s  // ❌ ERROR: returns a reference to data owned by this function
   }  // s is dropped here, so &s would be a dangling pointer!
   ```
   **Fix:** Return the String itself (ownership transfer): `fn dangle() -> String`
   **Why it happens:** Thinking like C/C++ where you can return pointers to anything

3. **Multiple mutable borrows**
   ```rust
   let mut s = String::from("hello");
   let r1 = &mut s;
   let r2 = &mut s;  // ❌ ERROR: cannot borrow as mutable more than once
   ```
   **Fix:** Only have one mutable borrow at a time. Use one, then the other.
   **Why it happens:** Not understanding the exclusivity rule for &mut

4. **Mixing mutable and immutable borrows**
   ```rust
   let mut s = String::from("hello");
   let r1 = &s;      // immutable borrow
   let r2 = &mut s;  // ❌ ERROR: cannot borrow as mutable because it's borrowed as immutable
   println!("{}", r1);
   ```
   **Fix:** Don't have any immutable borrows when you need a mutable one
   **Why it happens:** The rule is: many readers OR one writer, not both

5. **Confusion between &String and &str**
   ```rust
   fn takes_string_ref(s: &String) {  // Too specific!
       println!("{}", s);
   }

   takes_string_ref("hello");  // ❌ ERROR: expected &String, found &str
   ```
   **Fix:** Use `&str` in function parameters (more flexible):
   ```rust
   fn takes_string_ref(s: &str) {  // ✅ Works with both String and &str
       println!("{}", s);
   }
   takes_string_ref("hello");       // ✅ Works!
   takes_string_ref(&String::from("hello"));  // ✅ Also works!
   ```

## Stretch Goals

Once you've completed the basic exercises, try these:

1. **Add a `swap_strings` function**: Takes two mutable String references and swaps their contents
   - Learn about: Multiple mutable borrows (not at the same time!), std::mem::swap

2. **Add a `longest_string` function**: Takes two &str, returns the longer one
   - Learn about: Lifetime annotations, borrowing with lifetimes

3. **Add a `concat_strings` function**: Takes ownership of two Strings, returns concatenated String
   - Learn about: Taking ownership of multiple values, consuming values

4. **Add a `count_references` example**: Create a String and multiple references, document what's allowed
   - Learn about: Rules of multiple immutable borrows, when borrows end

## What's Next?

After completing this project, you'll move to **Project 03: Collections Basics**, where you'll learn about Vec and HashMap. Ownership and borrowing are crucial for working with collections, as they hold multiple values that all follow these rules!

Remember: Ownership might feel restrictive at first, but it's eliminating entire categories of bugs that plague other languages. The compiler is your friend!

Good luck, and remember to read the solution.rs comments—every line is explained in extreme detail!
