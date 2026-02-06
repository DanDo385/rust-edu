//! # Collections Basics - Interactive Demo

use collections_basics::solution::*;
use std::collections::HashMap;

fn main() {
    println!("=== Collections Basics Demo ===\n");

    // Demo 1: sum_of_evens
    println!("1. Sum of Even Numbers:");
    println!("   -------------------");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("   Numbers: {:?}", numbers);
    let sum = sum_of_evens(&numbers);
    println!("   Sum of evens: {}\n", sum);

    // Demo 2: word_frequency
    println!("2. Word Frequency Counter:");
    println!("   -----------------------");
    let text = "the quick brown fox jumps over the lazy dog the fox";
    println!("   Text: \"{}\"", text);
    let freq = word_frequency(text);
    println!("   Frequencies:");
    let mut sorted: Vec<_> = freq.iter().collect();
    sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    for (word, count) in sorted {
        println!("     {:10} : {}", word, count);
    }
    println!();

    // Demo 3: filter_and_sort
    println!("3. Filter and Sort:");
    println!("   ----------------");
    let numbers = vec![15, 3, 27, 8, 42, 19, 5, 33];
    println!("   Numbers: {:?}", numbers);
    println!("   Filter range: [10, 30]");
    let filtered = filter_and_sort(&numbers, 10, 30);
    println!("   Result: {:?}\n", filtered);

    // Demo 4: most_common_word
    println!("4. Most Common Word:");
    println!("   ------------------");
    let text = "Rust is great and Rust is fast and Rust is safe";
    println!("   Text: \"{}\"", text);
    match most_common_word(text) {
        Some(word) => println!("   Most common: \"{}\"\n", word),
        None => println!("   No words found\n"),
    }

    println!("=== Demo Complete! ===");
}
