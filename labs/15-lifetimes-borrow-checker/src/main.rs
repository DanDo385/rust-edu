//! # Lifetimes and Borrow Checker Demo

use lifetimes_borrow_checker::solution::{self, TextMetadata};

fn main() {
    println!("=== Lifetimes and Borrow Checker Demo ===\n");

    let a = String::from("short");
    let b = String::from("a much longer string");
    let longest = solution::longest(&a, &b);
    println!("longest: {longest}");

    let text = String::from("borrowed text");
    let meta = TextMetadata::new(&text, text.len());
    println!("metadata text: {}", meta.text());
    println!("metadata count: {}", meta.count());
    println!("describe: {}", solution::describe_text(&meta));

    let joined = solution::combine("hello", " world");
    println!("combined: {joined}");

    let refs = ["first", "second", "third"];
    println!("first_element: {:?}", solution::first_element(&refs));
    println!("validate_refs(len): {}", solution::validate_refs("abc", "xyz"));
}
