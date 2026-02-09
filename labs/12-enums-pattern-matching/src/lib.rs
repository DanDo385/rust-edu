//! # Lab 12: Enums & Pattern Matching
//!
//! This student-facing module exposes the API you will implement.
//! Replace each `todo!()` with your own code, then compare with `solution.rs`.

/// A message type demonstrating enums with data-bearing variants.
#[derive(Debug, Clone)]
pub enum Message {
    /// No associated data.
    Quit,
    /// Struct-like enum variant with named fields.
    Move { x: i32, y: i32 },
    /// Tuple-like enum variant with owned heap data.
    Write(String),
    /// Tuple-like enum variant with three components.
    ChangeColor(u8, u8, u8),
}

/// Process a message and return a human-readable description.
pub fn process_message(msg: Message) -> String {
    // TODO: Use `match` to handle every variant exhaustively.
    // Hint: `Write(String)` moves the owned String out of the enum.
    let _ = msg;
    todo!("Process each Message variant")
}

/// Return how many fields are carried by a given variant.
pub fn variant_size(msg: &Message) -> usize {
    // TODO: Match on `&Message` and return: Quit=0, Move=2, Write=1, ChangeColor=3.
    let _ = msg;
    todo!("Return structural field count for each variant")
}

/// Return true only when the message is `Message::Quit`.
pub fn is_quit(msg: &Message) -> bool {
    // TODO: Implement with either `match` or `matches!`.
    let _ = msg;
    todo!("Check whether message is Quit")
}

#[doc(hidden)]
pub mod solution;
