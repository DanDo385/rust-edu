//! # Lab 15: Lifetimes & The Borrow Checker
//!
//! Lifetimes are Rust's answer to: "How long is this reference valid?"
//! They're not about how long data lives, but how long borrowing is safe.
//!
//! **Core insight:** Rust's borrow checker uses lifetimes to prevent use-after-free bugs
//! at compile time - bugs that would cause segfaults in C/C++.

/// Returns the longest of two string slices.
///
/// **Lifetime annotation: `'a`**
/// - This lifetime parameter says: "all references must have the same lifetime 'a"
/// - Input references (`s1` and `s2`) must live at least as long as the output
/// - Rust prevents returning a reference to data that gets dropped
///
/// **From the borrow checker's perspective:**
/// ```ignore
/// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
///
/// s1: &'a str      -- reference valid for lifetime 'a
/// s2: &'a str      -- reference valid for lifetime 'a
/// return: &'a str  -- output reference also valid for lifetime 'a
/// ```
///
/// **What this prevents:**
/// ```ignore
/// // ❌ This would NOT compile:
/// fn bad<'a>(s1: &'a str) -> &'a str {
///     let temp = String::from("nope");
///     &temp  // ERROR: temp lives only as long as this function
///            // Can't return reference to data that will be dropped!
/// }
/// ```
/// In C: This would compile and return a dangling pointer (use-after-free bug!)
/// In Rust: Compiler rejects it at compile time. Memory safe!
pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/// A string reference paired with a number that owns its data.
///
/// **Mixed ownership and borrowing:**
/// - `text: &'a str` - borrowed reference (we don't own it)
/// - `count: i32` - owned value (we own it)
/// - Struct owns the count but only borrows the text
///
/// **Lifetime parameter on the struct itself:**
/// The struct can't live longer than the borrowed data it holds.
#[derive(Debug)]
pub struct TextMetadata<'a> {
    /// Reference to text (borrowed, not owned)
    pub text: &'a str,
    /// Number of characters (owned)
    pub count: usize,
}

impl<'a> TextMetadata<'a> {
    /// Creates a new TextMetadata.
    ///
    /// **Ownership transfer:**
    /// - text reference is BORROWED (not moved)
    /// - count is COPIED (it's usize, a tiny type)
    /// - TextMetadata now holds the borrow
    pub fn new(text: &'a str, count: usize) -> TextMetadata<'a> {
        TextMetadata { text, count }
    }

    /// Gets the borrowed text.
    pub fn text(&self) -> &'a str {
        self.text
    }

    /// Gets the character count.
    pub fn count(&self) -> usize {
        self.count
    }
}

/// Combines two text values into a new owned String.
///
/// **No lifetime parameters needed!**
/// - Both inputs are borrowed (`&str`)
/// - Output is owned (String)
/// - We CREATE new data (allocate on heap)
/// - Output lifetime is independent of input lifetimes
///
/// **Why no lifetime?**
/// - We don't return references to input data
/// - We return an owned String we created
/// - Ownership transfers to caller
pub fn combine(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

/// Returns a reference to the first element, if list is not empty.
///
/// **Lifetime on returned reference:**
/// - 'a ties the output lifetime to the input list lifetime
/// - If list is dropped, returned reference becomes invalid
/// - Rust prevents using the reference after list is dropped
pub fn first_element<'a>(list: &'a [&str]) -> Option<&'a str> {
    list.first().copied()
}

/// Validates that two references have compatible lifetimes.
///
/// **Multiple lifetime parameters:**
/// - 'a: lifetime of first reference
/// - 'b: lifetime of second reference
/// - Can be different! Function works even if lifetimes differ
pub fn validate_refs<'a, 'b>(first: &'a str, second: &'b str) -> bool {
    first.len() == second.len()
}

/// Demonstrates the borrow checker with a struct containing references.
///
/// **Lifetime inference:**
/// Sometimes Rust can infer lifetimes. This returns an owned String,
/// so no lifetime parameters needed on return type.
pub fn describe_text(meta: &TextMetadata) -> String {
    format!("Text: '{}', Count: {}", meta.text, meta.count)
}

pub mod solution;
