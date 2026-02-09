//! # Enums and Pattern Matching Demo

use enums_pattern_matching::solution::{self, Message};

fn main() {
    println!("=== Enums and Pattern Matching Demo ===\n");

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello from enum")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in messages {
        let description = solution::process_message(msg.clone());
        let fields = solution::variant_size(&msg);
        let quit = solution::is_quit(&msg);

        println!("message={msg:?}");
        println!("  description: {description}");
        println!("  field_count: {fields}");
        println!("  is_quit: {quit}\n");
    }
}
