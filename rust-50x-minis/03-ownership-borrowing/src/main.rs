// Project 03: Ownership and Borrowing
//
// This is THE MOST IMPORTANT concept in Rust. Understanding ownership is the key
// to understanding why Rust is both safe AND fast. We'll explore how Rust manages
// memory without garbage collection through its ownership system.

fn main() {
    println!("=== Rust Ownership and Borrowing ===\n");

    demonstrate_ownership();
    demonstrate_borrowing();
    demonstrate_mutable_borrowing();
    demonstrate_copy_vs_move();
    demonstrate_reference_rules();
}

// ============================================================================
// OWNERSHIP: THE CORE CONCEPT
// ============================================================================
// Rust's ownership rules:
// 1. Each value has exactly ONE owner
// 2. When the owner goes out of scope, the value is dropped (freed)
// 3. Ownership can be transferred (moved)

fn demonstrate_ownership() {
    println!("--- Ownership Basics ---");

    // Rule 1: Each value has one owner
    let s1 = String::from("hello");  // s1 OWNS the string
    println!("s1: {}", s1);

    // Rule 3: Ownership can be moved
    let s2 = s1;  // Ownership MOVES from s1 to s2
    println!("s2: {}", s2);

    // println!("{}", s1);  // ❌ ERROR! s1 no longer owns the value
    // This is a COMPILE ERROR: "value borrowed here after move"

    // WHY DOES RUST DO THIS?
    // String stores data on the HEAP. If both s1 and s2 pointed to the same heap
    // data, when they go out of scope, Rust would try to free the same memory twice
    // (double-free bug). By moving ownership, Rust ensures only ONE owner will free it.

    // Let's see what happens when variables go out of scope
    {
        let s3 = String::from("scoped");
        println!("s3: {}", s3);
    }  // <- s3 goes out of scope here, String is DROPPED (memory freed)

    // println!("{}", s3);  // ❌ ERROR! s3 no longer exists

    // Rule 2: When owner goes out of scope, value is dropped
    // When s2 goes out of scope at the end of this function, its String is freed

    println!();
}

// ============================================================================
// FUNCTIONS AND OWNERSHIP
// ============================================================================
// Passing a value to a function MOVES ownership (unless it's a Copy type)

fn takes_ownership(s: String) {
    println!("Function received: {}", s);
    // s goes out of scope at the end of this function, String is dropped
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s  // Ownership is MOVED to the caller
}

fn takes_and_gives_back(s: String) -> String {
    println!("Processing: {}", s);
    s  // Return ownership to the caller
}

// ============================================================================
// BORROWING: REFERENCES WITHOUT OWNERSHIP
// ============================================================================
// Instead of moving ownership, we can BORROW a value using references (&)
// This lets functions use a value without taking ownership

fn demonstrate_borrowing() {
    println!("--- Borrowing (Immutable References) ---");

    let s1 = String::from("hello");

    // Instead of moving, we pass a REFERENCE
    let len = calculate_length(&s1);  // &s1 creates a reference

    // s1 is still valid! We only borrowed it
    println!("The length of '{}' is {}.", s1, len);

    // You can have MULTIPLE immutable references
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);

    // All references are valid simultaneously because they're all read-only
    // No one can modify the data, so it's safe

    println!();
}

fn calculate_length(s: &String) -> usize {
    // s is a REFERENCE to a String, not the String itself
    // We're borrowing the String, not taking ownership
    s.len()
    // When s goes out of scope, nothing happens (we don't own the data)
}

// ============================================================================
// MUTABLE BORROWING
// ============================================================================
// If you need to modify a borrowed value, use a MUTABLE reference (&mut)
// Rule: You can have EITHER multiple immutable refs OR one mutable ref (not both)

fn demonstrate_mutable_borrowing() {
    println!("--- Mutable Borrowing ---");

    let mut s = String::from("hello");
    println!("Before: {}", s);

    // Pass a mutable reference
    change_string(&mut s);
    println!("After: {}", s);

    // CRITICAL RULE: Only ONE mutable reference at a time
    let r1 = &mut s;
    // let r2 = &mut s;  // ❌ ERROR! Cannot have two mutable references
    // println!("{}, {}", r1, r2);

    // WHY? To prevent DATA RACES at compile time!
    // A data race occurs when:
    // 1. Two+ pointers access the same data
    // 2. At least one is writing
    // 3. No synchronization mechanism
    // Rust's borrow checker ELIMINATES data races!

    r1.push_str("!");
    println!("Modified: {}", r1);

    // After r1 is no longer used, we can create new references
    let r2 = &mut s;
    r2.push_str("!");
    println!("Modified again: {}", r2);

    println!();
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// REFERENCE RULES
// ============================================================================
// The borrow checker enforces these rules:
// 1. You can have EITHER:
//    - Any number of immutable references (&T), OR
//    - Exactly ONE mutable reference (&mut T)
// 2. References must always be valid (no dangling references)

fn demonstrate_reference_rules() {
    println!("--- Reference Rules ---");

    let mut s = String::from("hello");

    // Multiple immutable references are OK
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point

    // Now we can create a mutable reference (previous refs are out of scope)
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3: {}", r3);

    // IMPORTANT: Scope of a reference is from where it's created to its LAST USE
    // This is called "Non-Lexical Lifetimes" (NLL)
    // Older Rust required variables to go fully out of scope, but modern Rust
    // is smarter and only cares about the last use

    // The Dangling Reference Problem (Rust prevents this at compile time)
    // let reference_to_nothing = dangle();  // Would not compile!

    let valid_reference = no_dangle();
    println!("Valid: {}", valid_reference);

    println!();
}

// This would NOT compile - returns a reference to a value that gets dropped
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ❌ ERROR! s is dropped, so the reference would be invalid
// }

// Instead, return the String itself (transfer ownership)
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ✅ OK! Ownership moves to the caller
}

// ============================================================================
// COPY vs MOVE TYPES
// ============================================================================
// Some types are COPY (assignment copies the value)
// Some types are MOVE (assignment moves ownership)

fn demonstrate_copy_vs_move() {
    println!("--- Copy vs Move ---");

    // Simple types like integers implement the Copy trait
    let x = 5;
    let y = x;  // x is COPIED to y
    println!("x: {}, y: {}", x, y);  // Both are still valid!

    // Types that implement Copy:
    // - All integer types (i32, u64, etc.)
    // - bool
    // - char
    // - Floating point types (f32, f64)
    // - Tuples containing only Copy types

    let tuple = (1, 2, 3);
    let tuple2 = tuple;  // Copied
    println!("tuple: {:?}, tuple2: {:?}", tuple, tuple2);

    // Types that are Move (do NOT implement Copy):
    // - String (owns heap data)
    // - Vec<T> (owns heap data)
    // - Any type that owns heap-allocated data

    let s1 = String::from("hello");
    let s2 = s1;  // MOVED, not copied
    // println!("{}", s1);  // ❌ ERROR! s1 was moved

    // WHY THE DIFFERENCE?
    // Copy types are small and live entirely on the stack.
    // Copying them is cheap (just copy a few bytes).
    //
    // Move types own heap data. Copying would require:
    // 1. Allocating new heap memory
    // 2. Copying all the data
    // 3. Managing two separate heap allocations
    // This is expensive! Rust makes it explicit with .clone()

    let s3 = String::from("hello");
    let s4 = s3.clone();  // Explicit deep copy
    println!("s3: {}, s4: {}", s3, s4);  // Both valid

    println!();
}

// ============================================================================
// SLICES: REFERENCES TO PARTS OF DATA
// ============================================================================
// Slices let you reference a contiguous sequence of elements

fn demonstrate_slices() {
    println!("--- Slices ---");

    let s = String::from("hello world");

    // String slices (&str) are references to parts of a String
    let hello = &s[0..5];   // or &s[..5]
    let world = &s[6..11];  // or &s[6..]
    let whole = &s[..];     // entire string

    println!("Full: {}", s);
    println!("Slice 1: {}", hello);
    println!("Slice 2: {}", world);
    println!("Whole: {}", whole);

    // Array slices work the same way
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // [2, 3]
    println!("Array slice: {:?}", slice);

    // String literals are slices!
    let literal: &str = "hello";  // Type is &str, not String
    println!("Literal: {}", literal);

    println!();
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. COMPILE-TIME CHECKING
//    All ownership and borrowing rules are checked at compile time.
//    There is ZERO runtime overhead for these safety guarantees!
//
// 2. STACK vs HEAP
//    - Copy types (i32, bool, etc.) live on the STACK
//    - Move types (String, Vec) have a stack component (pointer, length, capacity)
//      and heap-allocated data
//
// 3. AUTOMATIC DEALLOCATION
//    When a value goes out of scope, Rust automatically calls `drop`
//    to clean up any heap allocations. No garbage collector needed!
//
// 4. REFERENCE COUNTING (not shown here, but in Rc<T>)
//    For shared ownership, Rust has Rc<T> (reference counted smart pointer)
//    This adds minimal runtime overhead only when you need shared ownership
//
// 5. INTERIOR MUTABILITY (not shown here, but in RefCell<T>)
//    For cases where you need runtime borrow checking, RefCell<T> exists
//    This moves borrow checking from compile-time to runtime

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Each value has exactly ONE owner
// 2. Ownership can be MOVED (default for heap types) or COPIED (small stack types)
// 3. References (&T) let you borrow without taking ownership
// 4. Mutable references (&mut T) allow modification
// 5. Either multiple immutable refs OR one mutable ref (never both)
// 6. References must always be valid (no dangling pointers)
// 7. All checking happens at COMPILE TIME - zero runtime cost
// 8. This system prevents memory leaks, use-after-free, and data races
// 9. String owns heap data, &str is a borrowed slice
// 10. Understanding ownership is the KEY to mastering Rust

// ============================================================================
// THE BORROW CHECKER: YOUR FRIEND, NOT YOUR ENEMY
// ============================================================================
// New Rust developers often fight the borrow checker. This is normal!
// The borrow checker is teaching you to write safe, correct code.
//
// When you get a borrow checker error:
// 1. Read the error message carefully (Rust has GREAT error messages)
// 2. Understand WHY Rust is complaining (it's protecting you from a bug)
// 3. Redesign your code to work with ownership, not against it
//
// Common solutions:
// - Use references instead of moving ownership
// - Clone data when you need independent copies
// - Use smart pointers (Rc, Arc) for shared ownership
// - Restructure your code to have clearer ownership

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ let s2 = s1; println!("{}", s1);  (moved value)
// ❌ let r1 = &mut s; let r2 = &mut s;  (two mutable refs)
// ❌ let r1 = &s; let r2 = &mut s;  (immutable + mutable refs)
// ❌ return &s;  (returning reference to local variable)
// ❌ Forgetting to use & when you want to borrow
// ❌ Using & when you want to transfer ownership
