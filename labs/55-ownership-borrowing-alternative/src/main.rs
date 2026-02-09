//! # Ownership & Borrowing Demo

use ownership_borrowing_alternative::solution::{
    append_suffix, first_word, reverse_string, string_length, word_count,
};

fn main() {
    println!("=== Ownership & Borrowing Demo ===");

    let text = "hello rust world";
    println!("len={} first={} words={}", string_length(text), first_word(text), word_count(text));
    println!("reversed={}", reverse_string(text));

    let mut owned = String::from("borrow");
    append_suffix(&mut owned, " checker");
    println!("mutated={}", owned);
}
