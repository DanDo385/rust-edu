// Project 12: Lifetimes and Borrow Checker
//
// This program demonstrates Rust's lifetime system and borrow checker in depth.
// Lifetimes are one of Rust's most unique features - they ensure memory safety
// at compile time with ZERO runtime cost.
//
// Key Concepts:
// - Lifetimes describe how long references are valid
// - The borrow checker enforces that references never outlive their data
// - Most of the time, lifetimes are inferred (implicit)
// - Sometimes you need to help the compiler with explicit annotations

fn main() {
    println!("=== Lifetimes and Borrow Checker ===\n");

    // ========================================================================
    // PART 1: The Borrow Checker Rules
    // ========================================================================
    println!("--- Part 1: Borrow Checker Rules ---\n");

    // RULE 1: At any given time, you can have EITHER:
    //         - One mutable reference, OR
    //         - Any number of immutable references
    //
    // RULE 2: References must always be valid (no dangling references)

    let mut s = String::from("hello");

    // Multiple immutable references - OK!
    let r1 = &s;
    let r2 = &s;
    println!("Immutable references: r1={}, r2={}", r1, r2);
    // r1 and r2 are no longer used after this point (Non-Lexical Lifetimes)

    // Now we can make a mutable reference
    let r3 = &mut s;
    r3.push_str(" world");
    println!("Mutable reference: r3={}", r3);

    // This would fail (uncomment to see error):
    // let r4 = &s;      // Immutable borrow
    // let r5 = &mut s;  // ❌ ERROR: Can't have mutable and immutable together
    // println!("{} {}", r4, r5);

    println!();

    // ========================================================================
    // PART 2: Lifetime Elision (Implicit Lifetimes)
    // ========================================================================
    println!("--- Part 2: Lifetime Elision ---\n");

    // Most of the time, you don't need to write lifetime annotations
    // The compiler infers them using "elision rules"

    // Example: Function with one reference parameter
    // The compiler knows the output must live as long as the input
    fn first_word(s: &str) -> &str {
        // No explicit lifetimes needed!
        // Compiler infers: fn first_word<'a>(s: &'a str) -> &'a str
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let sentence = String::from("Hello Rust");
    let word = first_word(&sentence);
    println!("First word: {}", word);

    // The borrow checker ensures 'sentence' lives long enough:
    // println!("{}", word);  // word is still valid here

    println!();

    // ========================================================================
    // PART 3: Explicit Lifetime Annotations
    // ========================================================================
    println!("--- Part 3: Explicit Lifetime Annotations ---\n");

    // When there are multiple input references, the compiler can't infer
    // which one the output is tied to. You must be explicit!

    // This function returns the longer of two string slices
    // Lifetime annotation 'a means: "the output lives as long as BOTH inputs"
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // The 'a annotation tells the compiler:
        // "The returned reference is valid for as long as BOTH x AND y are valid"
        // In practice, this means the SHORTER of the two lifetimes
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string");
    let string2 = String::from("short");

    let result = longest(&string1, &string2);
    println!("Longest: {}", result);

    // The borrow checker ensures this works:
    {
        let string1 = String::from("long string");
        let result;
        {
            let string2 = String::from("short");
            result = longest(&string1, &string2);
            println!("Longest (scoped): {}", result);
        }
        // result is still valid here because string1 is still alive
        // But if we returned string2, it would be dangling!
    }

    println!();

    // ========================================================================
    // PART 4: Multiple Lifetime Parameters
    // ========================================================================
    println!("--- Part 4: Multiple Lifetime Parameters ---\n");

    // Sometimes different references have different lifetimes
    // Use different lifetime parameters: 'a, 'b, 'c, etc.

    // This function returns the first parameter (x), so output lifetime = x's lifetime
    // y can have a different lifetime - it doesn't affect the output
    fn first_param<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        println!("  Comparing: '{}' and '{}'", x, y);
        x  // Only returns x, so only 'a matters for output
    }

    let string1 = String::from("first");
    let result;

    {
        let string2 = String::from("second");
        result = first_param(&string1, &string2);
        // result is tied to string1's lifetime, not string2's
    }  // string2 dropped here, but result is still valid!

    println!("Result: {}", result);

    println!();

    // ========================================================================
    // PART 5: Lifetimes in Structs
    // ========================================================================
    println!("--- Part 5: Lifetimes in Structs ---\n");

    // When a struct holds references, you must annotate lifetimes
    // This tells the compiler: "This struct cannot outlive the data it references"

    #[derive(Debug)]
    struct Excerpt<'a> {
        part: &'a str,
    }

    // The 'a annotation means:
    // "An instance of Excerpt can't outlive the reference it holds in 'part'"

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // Create an Excerpt holding a reference to part of 'novel'
    let excerpt = Excerpt {
        part: first_sentence,
    };

    println!("Excerpt: {:?}", excerpt);
    // 'excerpt' cannot outlive 'novel' - the borrow checker enforces this!

    // This would fail (uncomment to see error):
    // let excerpt_ref;
    // {
    //     let novel = String::from("Call me Ishmael.");
    //     excerpt_ref = Excerpt { part: &novel };
    // }  // ❌ ERROR: novel dropped here, but excerpt_ref would have dangling reference!
    // println!("{:?}", excerpt_ref);

    println!();

    // ========================================================================
    // PART 6: Methods with Lifetimes
    // ========================================================================
    println!("--- Part 6: Methods with Lifetimes ---\n");

    impl<'a> Excerpt<'a> {
        // Lifetime elision rule 3: If there's &self, output gets self's lifetime
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            // No explicit lifetime needed! Compiler infers:
            // fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &'a str
            println!("Attention: {}", announcement);
            self.part
        }

        // Sometimes you need multiple lifetimes in methods too
        fn compare<'b>(&self, other: &'b str) -> bool {
            self.part == other
        }
    }

    let text = String::from("In the beginning...");
    let excerpt = Excerpt {
        part: &text[..17],
    };

    let result = excerpt.announce_and_return_part("New excerpt");
    println!("Returned: {}", result);

    let matches = excerpt.compare("In the beginning");
    println!("Matches: {}", matches);

    println!();

    // ========================================================================
    // PART 7: The 'static Lifetime
    // ========================================================================
    println!("--- Part 7: The 'static Lifetime ---\n");

    // 'static means the reference can live for the ENTIRE duration of the program
    // String literals have the 'static lifetime (they're embedded in the binary)

    let static_str: &'static str = "I live for the entire program";
    println!("Static string: {}", static_str);

    // Static variables also have 'static lifetime
    static GLOBAL_GREETING: &str = "Hello from a static variable!";
    println!("{}", GLOBAL_GREETING);

    // Common mistake: Thinking you need 'static when you don't
    fn print_it(input: &str) {
        // This does NOT require 'static!
        // It just needs to be valid for the duration of this function
        println!("{}", input);
    }

    let temp = String::from("temporary");
    print_it(&temp);  // Works fine, even though temp is not 'static

    println!();

    // ========================================================================
    // PART 8: Lifetime Bounds with Generics
    // ========================================================================
    println!("--- Part 8: Lifetime Bounds with Generics ---\n");

    // You can combine lifetimes with generic type parameters

    use std::fmt::Display;

    // This function takes a generic type T that implements Display
    // and a string slice, and returns the longer of the two as a string
    fn longest_with_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = "short";
    let s2 = "longer string";
    let result = longest_with_announcement(s1, s2, "Comparing strings!");
    println!("Longest: {}", result);

    println!();

    // ========================================================================
    // PART 9: Common Patterns and Solutions
    // ========================================================================
    println!("--- Part 9: Common Patterns and Solutions ---\n");

    // Pattern 1: Returning owned data instead of references
    fn process_string(input: &str) -> String {
        // Instead of returning &str (which would need lifetime annotations),
        // return an owned String
        input.to_uppercase()
    }

    let result = process_string("hello");
    println!("Owned result: {}", result);

    // Pattern 2: Using Clone to avoid lifetime issues
    #[derive(Debug, Clone)]
    struct Config {
        value: String,
    }

    fn get_config(input: &str) -> Config {
        // Clone the input to create an owned value
        Config {
            value: input.to_string(),
        }
    }

    let cfg = get_config("my_config");
    println!("Config: {:?}", cfg);

    // Pattern 3: Slice from owned data
    fn get_first_half(data: &Vec<i32>) -> &[i32] {
        let mid = data.len() / 2;
        &data[0..mid]
    }

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let half = get_first_half(&numbers);
    println!("First half: {:?}", half);

    println!();

    // ========================================================================
    // PART 10: Advanced Example - Holding Multiple References
    // ========================================================================
    println!("--- Part 10: Struct with Multiple References ---\n");

    // A struct can hold references with different lifetimes
    struct Context<'s, 't> {
        source: &'s str,
        target: &'t str,
    }

    impl<'s, 't> Context<'s, 't> {
        fn new(source: &'s str, target: &'t str) -> Self {
            Context { source, target }
        }

        // Method returning a reference tied to 'source' lifetime
        fn get_source(&self) -> &'s str {
            self.source
        }

        // Method returning a reference tied to 'target' lifetime
        fn get_target(&self) -> &'t str {
            self.target
        }
    }

    let source_text = String::from("Source data");
    let target_text = String::from("Target data");

    let ctx = Context::new(&source_text, &target_text);
    println!("Source: {}", ctx.get_source());
    println!("Target: {}", ctx.get_target());

    println!();

    // ========================================================================
    // PART 11: What the Compiler is Doing
    // ========================================================================
    println!("--- Part 11: Understanding Borrow Checker Internals ---\n");

    println!("The borrow checker performs these checks:");
    println!("1. Each reference has a lifetime (scope where it's valid)");
    println!("2. For each borrow, check that the value lives long enough");
    println!("3. Ensure no mutable borrow while immutable borrows exist");
    println!("4. Ensure only one mutable borrow at a time");
    println!("5. All this happens at COMPILE TIME - zero runtime cost!\n");

    // Example: Visualizing lifetimes
    {
        let r;                // ----+-- 'a
                              //     |
        {                     //     |
            let x = 5;        // --+-----+-- 'b
            r = &x;           //   |  |  |
        }                     // --+  |  |
                              //      |  |
        // println!("{}", r); // ❌   |  | ERROR: x is dropped, r is dangling
    }                         // -----+

    println!("Lifetime 'a is longer than lifetime 'b");
    println!("The reference r (with lifetime 'a) tries to point to x (with lifetime 'b)");
    println!("This would be a dangling reference - the borrow checker prevents it!\n");

    // Working example:
    {
        let x = 5;            // -----+-- 'b
        let r = &x;           // --+  |
                              //   |  |
        println!("Valid reference: {}", r); // ✅ OK: x is still alive
    }                         // --+--+

    println!();

    println!("=== Program Complete ===");
}

// ============================================================================
// ADDITIONAL DEMONSTRATIONS (as functions)
// ============================================================================

/// Example: Function that doesn't need lifetime annotations
/// (Elision rule: one input → lifetime assigned to output)
fn get_first_char(s: &str) -> &str {
    &s[0..1]
}

/// Example: Function with multiple inputs returning a reference
/// Must explicitly state which input's lifetime the output is tied to
fn choose_first<'a>(first: &'a str, _second: &str) -> &'a str {
    // Output lifetime tied only to 'first', not '_second'
    first
}

/// Example: Generic function with lifetime bounds
fn print_ref<'a, T: std::fmt::Display>(value: &'a T) {
    println!("Value: {}", value);
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Lifetimes are COMPILE-TIME annotations - zero runtime cost
// 2. They prevent dangling references and use-after-free bugs
// 3. Most of the time, lifetimes are inferred (lifetime elision)
// 4. Use explicit lifetimes when compiler can't infer (multiple inputs → one output)
// 5. Structs holding references need lifetime annotations
// 6. 'static means "lives for entire program" (not "lives forever at runtime")
// 7. Borrow checker rules: one mutable XOR many immutable references
// 8. References must always be valid (no dangling)
// 9. Non-Lexical Lifetimes (NLL) make borrows end when last used, not at scope end
// 10. Work WITH the borrow checker, not against it - it's protecting you!

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Trying to return a reference to a local variable (dangling reference)
// ❌ Holding both mutable and immutable references simultaneously
// ❌ Forgetting lifetime annotations on structs with references
// ❌ Thinking 'static means "lasts forever at runtime" (it's compile-time!)
// ❌ Fighting the borrow checker instead of restructuring code
// ❌ Not using Non-Lexical Lifetimes (update Rust edition to 2018+)
// ❌ Over-using .clone() to "fix" borrow checker errors (inefficient)
// ❌ Not understanding that lifetimes describe relationships, not durations

// ============================================================================
// WORKING WITH THE BORROW CHECKER (NOT AGAINST IT)
// ============================================================================
//
// If the borrow checker complains, ask yourself:
// 1. Do I really need this reference? Could I use an owned value?
// 2. Can I restructure to minimize borrow scope?
// 3. Am I trying to mutate something while it's borrowed?
// 4. Do I need interior mutability? (RefCell, Mutex, etc.)
// 5. Would cloning be acceptable here? (Sometimes it's fine!)
//
// The borrow checker is NOT your enemy - it's preventing bugs that would
// cause crashes, security vulnerabilities, and data races in other languages!

// ============================================================================
// COMPARISON: RUST vs C++ vs GO
// ============================================================================
//
// C++ References:
//   - Can dangle (undefined behavior)
//   - No compile-time safety
//   - Requires manual discipline
//   - Valgrind/ASan needed to find bugs
//
// Go Pointers:
//   - Safe (garbage collected)
//   - Runtime overhead
//   - GC pauses
//   - Can't have pointers to stack
//
// Rust References:
//   - Safe (borrow checker)
//   - ZERO runtime overhead
//   - All checks at compile time
//   - Can reference stack safely
//   - Steeper learning curve, but worth it!
