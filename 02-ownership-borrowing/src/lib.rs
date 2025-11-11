//! # Ownership and Borrowing - Rust's Superpower
//!
//! This project teaches you the concepts that make Rust unique:
//! - **Ownership**: Every value has a single owner
//! - **Borrowing**: You can lend out references without transferring ownership
//! - **Move semantics**: Non-Copy types transfer ownership on assignment
//!
//! ## Your Task
//!
//! Implement the four functions below. Each demonstrates a different aspect
//! of ownership and borrowing.
//!
//! ## Running Your Code
//!
//! ```bash
//! # Run tests to check your implementation
//! cargo test -p ownership-borrowing
//!
//! # Run the main binary to see it in action
//! cargo run -p ownership-borrowing
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

/// Takes ownership of a String, adds an exclamation mark, and returns it.
///
/// This function demonstrates:
/// - Taking ownership with `s: String` (no &)
/// - Modifying owned data
/// - Returning ownership to the caller
///
/// # Parameters
/// - `s`: A String we take ownership of (caller can't use it after calling this)
///
/// # Returns
/// The String with "!" appended
///
/// # Example
/// ```ignore
/// use ownership_borrowing::add_exclamation;
/// let original = String::from("Hello");
/// let result = add_exclamation(original);
/// // original is no longer valid here (moved into function)
/// assert_eq!(result, "Hello!");
/// ```ignore
///
/// # Ownership Notes
/// - `s` is MOVED into this function
/// - Caller loses access to original variable
/// - We own `s`, so we can modify it
/// - We return it, transferring ownership back to caller
pub fn add_exclamation(s: String) -> String {
    // TODO: Add an exclamation mark to the string and return it
    // Hint: Use s.push('!') or s.push_str("!")
    // Hint: Remember to return the modified string
    todo!("Add an exclamation mark to the string")
}

/// Borrows a String immutably and returns its length.
///
/// This function demonstrates:
/// - Immutable borrowing with `&String`
/// - Reading data without taking ownership
/// - Caller can still use the String after this function
///
/// # Parameters
/// - `s`: An immutable reference to a String (we borrow it, don't own it)
///
/// # Returns
/// The length of the string as usize
///
/// # Example
/// ```ignore
/// use ownership_borrowing::get_length;
/// let text = String::from("Hello");
/// let len = get_length(&text);
/// assert_eq!(len, 5);
/// // text is still valid here! We only borrowed it.
/// assert_eq!(text, "Hello");
/// ```ignore
///
/// # Borrowing Notes
/// - `&s` is an immutable borrow
/// - We can read but not modify
/// - Multiple immutable borrows are allowed simultaneously
/// - Original data remains valid in caller
pub fn get_length(s: &String) -> usize {
    // TODO: Return the length of the string
    // Hint: Use the .len() method
    todo!("Return the length of the string")
}

/// Borrows a String mutably and converts it to uppercase in place.
///
/// This function demonstrates:
/// - Mutable borrowing with `&mut String`
/// - Modifying data without owning it
/// - In-place modification (no return value needed)
///
/// # Parameters
/// - `s`: A mutable reference to a String (we can modify it, but don't own it)
///
/// # Example
/// ```ignore
/// use ownership_borrowing::make_uppercase;
/// let mut text = String::from("hello");
/// make_uppercase(&mut text);
/// assert_eq!(text, "HELLO");
/// // text is still valid and was modified!
/// ```ignore
///
/// # Borrowing Notes
/// - `&mut s` is a mutable borrow
/// - Only ONE mutable borrow allowed at a time
/// - Can't have any immutable borrows while mutable borrow exists
/// - We can modify but don't own (can't drop or move it)
/// - Modification happens in place—caller sees the changes
pub fn make_uppercase(s: &mut String) {
    // TODO: Convert the string to uppercase in place
    // Hint: Get the uppercase version: s.to_uppercase()
    // Hint: Clear and replace: s.clear(); s.push_str(&uppercase);
    // or use: *s = s.to_uppercase();
    todo!("Convert the string to uppercase in place")
}

/// Demonstrates the difference between Copy types and Move types.
///
/// This function demonstrates:
/// - Copy types (i32) are automatically duplicated
/// - Move types (String) transfer ownership
/// - How to explicitly clone if you want a copy
///
/// # Returns
/// A String explaining what happened with Copy vs Move
///
/// # Example
/// ```ignore
/// use ownership_borrowing::demonstrate_copy_vs_move;
/// let explanation = demonstrate_copy_vs_move();
/// assert!(explanation.contains("Copy"));
/// assert!(explanation.contains("Move"));
/// ```ignore
///
/// # Ownership Notes
/// - Demonstrates i32 (implements Copy trait)
/// - Demonstrates String (doesn't implement Copy)
/// - Shows when variables become invalid
/// - Shows how to use .clone() for explicit copying
pub fn demonstrate_copy_vs_move() -> String {
    // TODO: Create a demonstration of Copy vs Move
    //
    // 1. Show Copy behavior with i32:
    //    let x = 5;
    //    let y = x;  // x is copied, both x and y are valid
    //
    // 2. Show Move behavior with String:
    //    let s1 = String::from("hello");
    //    let s2 = s1;  // s1 is moved to s2, s1 is no longer valid
    //
    // 3. Show cloning:
    //    let s3 = String::from("world");
    //    let s4 = s3.clone();  // Explicit copy, both valid
    //
    // 4. Build and return a String explaining what happened
    //    Use format! to create the explanation
    todo!("Demonstrate Copy vs Move with examples")
}

// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
