# Project 03 - Collections Basics

## What You're Building

Learn Rust's powerful collection types: Vec (dynamic arrays), HashMap (key-value storage), and iteration patterns. You'll build functions that manipulate data in ways that are common in real-world applications but with Rust's performance and safety guarantees.

## The Exercises

1. **sum_of_evens**: Sum all even numbers in a Vec
   - Learn about: Vec, iteration, filtering, summing
   - Example: [1, 2, 3, 4, 5] → 6 (2 + 4)

2. **word_frequency**: Count how many times each word appears
   - Learn about: HashMap, entry API, counting
   - Example: "hello world hello" → {"hello": 2, "world": 1}

3. **filter_and_sort**: Keep numbers in range and sort them
   - Learn about: Vec manipulation, filtering, sorting
   - Example: [5, 2, 8, 1, 9], min=2, max=7 → [2, 5]

4. **most_common_word**: Find the most frequently occurring word
   - Learn about: HashMap, finding max, Option
   - Example: "the cat and the dog" → Some("the")

## How to Run

```bash
cargo test -p collections-basics
cargo run -p collections-basics
```
