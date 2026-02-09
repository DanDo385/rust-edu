//! # Lab 12: Enums & Pattern Matching
//!
//! This lab teaches you about Rust's most powerful type system feature: **enums**.
//! Unlike enums in languages like C, Rust enums can hold different types of data
//! in each variant. Combined with pattern matching, this prevents entire classes of bugs.
//!
//! ## Learning Objectives
//!
//! By the end, you'll understand:
//! - **Enums with data**: Each variant can hold different types
//! - **Pattern matching**: Exhaustive matching (compiler enforces completeness)
//! - **Option<T>**: Rust's null-safe alternative
//! - **Ownership in enums**: How data flows through enum variants

/// A message type that can hold different kinds of data.
///
/// **From the borrow checker's perspective:**
/// - Each variant holds different data (Quit=none, Move=i32s, Write=String, ChangeColor=u8s)
/// - String variants involve OWNERSHIP (who owns the String?)
/// - Tuple variants hold owned values (the enum owns them)
/// - When you extract data with match, you can take OWNERSHIP or borrow
#[derive(Debug, Clone)]
pub enum Message {
    /// No associated data
    Quit,
    /// Two i32 fields (named, like a struct)
    Move { x: i32, y: i32 },
    /// One String (owned data)
    Write(String),
    /// Three u8 values (RGB)
    ChangeColor(u8, u8, u8),
}

/// Processes a message and prints its contents.
///
/// **Teaching focus:**
/// - Pattern matching on enums
/// - Destructuring data from variants
/// - Taking ownership vs borrowing
pub fn process_message(msg: Message) -> String {
    // **From the borrow checker's perspective:**
    // - msg is owned by this function (we consume it with match)
    // - When we match, we extract the data by MOVING it
    // - After match, msg is no longer available (was moved)
    match msg {
        Message::Quit => "Quit command received".to_string(),
        Message::Move { x, y } => {
            // **Ownership note:**
            // - x and y are i32 (Copy types), so they're copied from msg
            // - Caller still has their copy of msg.x and msg.y
            format!("Move to ({}, {})", x, y)
        }
        Message::Write(text) => {
            // **Ownership note:**
            // - text is a String (owned type)
            // - We TAKE ownership of the String from msg
            // - msg.Write's String is now "moved" to text
            // - After match, we own text and can return it
            format!("Write: {}", text)
        }
        Message::ChangeColor(r, g, b) => {
            // **Ownership note:**
            // - r, g, b are u8 (Copy types)
            // - Copied from msg, no ownership concerns
            format!("Change color to RGB({}, {}, {})", r, g, b)
        }
    }
}

/// Gets the number of fields in a message variant.
///
/// **Teaching focus:**
/// - Pattern matching with guards (`if`)
/// - Extracting structural information
pub fn variant_size(msg: &Message) -> usize {
    // **From the borrow checker's perspective:**
    // - msg: &Message is borrowed (immutable reference)
    // - We only READ the enum, never MODIFY it
    // - Safe to borrow because we're not taking ownership
    match msg {
        Message::Quit => 0,
        Message::Move { .. } => 2,      // { x: i32, y: i32 }
        Message::Write(_) => 1,         // (String)
        Message::ChangeColor(_, _, _) => 3,  // (u8, u8, u8)
    }
}

/// Checks if a message is a Quit command.
///
/// **Teaching focus:**
/// - Simple pattern matching
/// - Using match for boolean logic
pub fn is_quit(msg: &Message) -> bool {
    matches!(msg, Message::Quit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_quit() {
        let msg = Message::Quit;
        let result = process_message(msg);
        assert!(result.contains("Quit"));
    }

    #[test]
    fn test_variant_size_move() {
        let msg = Message::Move { x: 10, y: 20 };
        assert_eq!(variant_size(&msg), 2);
    }
}
