// Project 14: Closures and Iterators (Zero-Cost Abstraction)
//
// This program demonstrates Rust's closures and how they enable powerful
// functional programming patterns with ZERO runtime cost. We'll explore the
// three closure traits (Fn, FnMut, FnOnce), lazy evaluation, and how the
// compiler turns high-level code into optimal machine code.
//
// Key Insight: Closures are just structs with captured variables!
// The compiler automatically generates a struct for each closure.

fn main() {
    println!("=== Closures and Iterators (Zero-Cost Abstraction) ===\n");

    // ========================================================================
    // PART 1: Closure Basics
    // ========================================================================
    println!("--- Part 1: Closure Basics ---\n");

    // A closure is an anonymous function that can capture variables from
    // its environment. Syntax: |parameters| body

    // Simple closure
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // Closure with explicit types (usually inferred)
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("multiply(3, 4) = {}", multiply(3, 4));

    // Multi-line closure
    let complex = |x| {
        let temp = x * 2;
        temp + 1
    };
    println!("complex(5) = {}", complex(5));

    // Capturing environment variables
    let factor = 10;
    let scale = |x| x * factor;  // Captures 'factor' from environment
    println!("scale(5) = {}", scale(5));

    println!();

    // ========================================================================
    // PART 2: The Three Closure Traits - Fn, FnMut, FnOnce
    // ========================================================================
    println!("--- Part 2: Closure Traits (Fn, FnMut, FnOnce) ---\n");

    // The compiler automatically determines which trait to implement based
    // on what the closure does with captured variables

    // Fn - Captures by immutable reference
    // Can be called multiple times, doesn't modify captures
    println!("Fn - Immutable borrow:");
    let x = 5;
    let print_x = || println!("  x = {}", x);  // Borrows x immutably
    print_x();  // Can call multiple times
    print_x();
    println!("  x is still usable: {}", x);  // x not moved

    println!();

    // FnMut - Captures by mutable reference
    // Can be called multiple times, can modify captures
    println!("FnMut - Mutable borrow:");
    let mut counter = 0;
    let mut increment = || {
        counter += 1;  // Mutably borrows counter
        println!("  Counter: {}", counter);
    };
    increment();  // Can call multiple times
    increment();
    increment();
    println!("  Final counter: {}", counter);  // counter was modified

    println!();

    // FnOnce - Takes ownership of captures
    // Can only be called once (consumes itself)
    println!("FnOnce - Takes ownership:");
    let s = String::from("hello");
    let consume_string = || {
        println!("  Consuming: {}", s);
        drop(s);  // Takes ownership and drops s
    };
    consume_string();  // Can only call once
    // consume_string();  // ❌ ERROR: already consumed!
    // println!("{}", s);  // ❌ ERROR: s was moved into closure

    println!();

    // ========================================================================
    // PART 3: The 'move' Keyword
    // ========================================================================
    println!("--- Part 3: The 'move' Keyword ---\n");

    // By default, closures capture by reference
    // Use 'move' to force ownership transfer

    println!("Without move (borrows):");
    let x = vec![1, 2, 3];
    let print_vec = || println!("  Vec: {:?}", x);  // Borrows x
    print_vec();
    println!("  x still available: {:?}", x);

    println!("\nWith move (takes ownership):");
    let x = vec![1, 2, 3];
    let print_vec = move || println!("  Vec: {:?}", x);  // Takes ownership
    print_vec();
    // println!("{:?}", x);  // ❌ ERROR: x was moved

    // 'move' is essential for returning closures or spawning threads
    println!("\nReturning a closure (requires move):");
    let make_adder = |n: i32| move |x| x + n;  // move n into closure
    let add_5 = make_adder(5);
    println!("  add_5(10) = {}", add_5(10));

    println!();

    // ========================================================================
    // PART 4: Closures with Iterators
    // ========================================================================
    println!("--- Part 4: Closures with Iterators ---\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map - Transform each element
    let doubled: Vec<_> = numbers.iter()
        .map(|x| x * 2)  // Closure transforms each element
        .collect();
    println!("Doubled: {:?}", doubled);

    // filter - Keep elements matching a predicate
    let evens: Vec<_> = numbers.iter()
        .filter(|x| *x % 2 == 0)  // Closure decides which to keep
        .copied()
        .collect();
    println!("Evens: {:?}", evens);

    // Combining map and filter
    let result: Vec<_> = numbers.iter()
        .filter(|x| **x > 5)       // Keep numbers > 5
        .copied()
        .map(|x| x * x)           // Square them
        .collect();
    println!("Squares of numbers > 5: {:?}", result);

    // Using captured variables in closures
    let threshold = 5;
    let above_threshold: Vec<_> = numbers.iter()
        .filter(|x| **x > threshold)  // Captures threshold
        .copied()
        .collect();
    println!("Above {}: {:?}", threshold, above_threshold);

    // fold - Reduce with accumulator
    let sum = numbers.iter()
        .fold(0, |acc, x| acc + x);  // Closure combines acc and each x
    println!("Sum using fold: {}", sum);

    // Custom accumulation
    let product = numbers.iter()
        .filter(|x| **x <= 5)
        .fold(1, |acc, x| acc * x);
    println!("Product of numbers <= 5: {}", product);

    println!();

    // ========================================================================
    // PART 5: Function Pointers vs Closures
    // ========================================================================
    println!("--- Part 5: Function Pointers vs Closures ---\n");

    // Regular functions can be used where closures are expected
    fn double(x: i32) -> i32 {
        x * 2
    }

    let doubled: Vec<_> = numbers.iter()
        .copied()
        .map(double)  // Function pointer (no capture)
        .collect();
    println!("Doubled with function: {:?}", doubled);

    // But functions can't capture environment
    // let factor = 3;
    // fn multiply(x: i32) -> i32 {
    //     x * factor  // ❌ ERROR: can't capture
    // }

    // Closures can capture, functions can't
    let factor = 3;
    let tripled: Vec<_> = numbers.iter()
        .map(|x| x * factor)  // Closure can capture 'factor'
        .collect();
    println!("Tripled with closure: {:?}", tripled);

    println!();

    // ========================================================================
    // PART 6: Higher-Order Functions
    // ========================================================================
    println!("--- Part 6: Higher-Order Functions ---\n");

    // Functions that take closures as parameters (higher-order functions)

    // Generic function accepting any closure
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,  // F must implement Fn trait
    {
        f(f(x))
    }

    let add_one = |x| x + 1;
    let result = apply_twice(add_one, 5);
    println!("apply_twice(add_one, 5) = {}", result);  // (5 + 1) + 1 = 7

    // Function taking FnMut (can modify state)
    fn apply_n_times<F>(mut f: F, x: i32, n: usize) -> i32
    where
        F: FnMut(i32) -> i32,
    {
        let mut result = x;
        for _ in 0..n {
            result = f(result);
        }
        result
    }

    let mut calls = 0;
    let double_and_count = |x| {
        calls += 1;
        x * 2
    };
    let result = apply_n_times(double_and_count, 1, 5);
    println!("Doubling 1 five times: {}", result);  // 1 * 2^5 = 32
    println!("Function was called {} times", calls);

    println!();

    // ========================================================================
    // PART 7: Lazy Evaluation
    // ========================================================================
    println!("--- Part 7: Lazy Evaluation ---\n");

    println!("Iterators are LAZY - they don't compute until consumed!");

    let data = vec![1, 2, 3, 4, 5];

    // This does NOTHING - no computation happens!
    let _lazy = data.iter()
        .map(|x| {
            println!("  Mapping {}", x);  // This won't print!
            x * 2
        });

    println!("No output yet - the map hasn't run!\n");

    // Now consume it - computation happens
    println!("Consuming with collect:");
    let result: Vec<_> = data.iter()
        .map(|x| {
            println!("  Mapping {}", x);  // Now it prints!
            x * 2
        })
        .collect();
    println!("Result: {:?}", result);

    println!();

    // ========================================================================
    // PART 8: Custom Iterators
    // ========================================================================
    println!("--- Part 8: Custom Iterators ---\n");

    // Implementing the Iterator trait
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;  // Type of elements yielded

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    println!("Custom Counter iterator:");
    let counter = Counter::new(5);
    for num in counter {
        print!("{} ", num);
    }
    println!();

    // Can use all iterator methods on custom iterators!
    let sum: u32 = Counter::new(10)
        .filter(|x| x % 2 == 0)  // Even numbers
        .map(|x| x * x)          // Square them
        .sum();
    println!("Sum of squares of even numbers 1-10: {}", sum);

    println!();

    // ========================================================================
    // PART 9: Iterator Adaptor Chains (Zero-Cost Abstraction)
    // ========================================================================
    println!("--- Part 9: Zero-Cost Iterator Chains ---\n");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Complex iterator chain
    let result: i32 = data.iter()
        .filter(|x| *x % 2 == 0)      // Keep evens
        .map(|x| x * x)                // Square them
        .filter(|x| *x > 10)           // Keep if > 10
        .sum();                        // Sum them

    println!("Functional style: {}", result);

    // Equivalent imperative code (what it compiles to)
    let mut result_imperative = 0;
    for x in &data {
        if x % 2 == 0 {
            let squared = x * x;
            if squared > 10 {
                result_imperative += squared;
            }
        }
    }
    println!("Imperative style: {}", result_imperative);

    println!("\nThese compile to nearly identical assembly code!");
    println!("Iterator chains are a ZERO-COST abstraction.");

    println!();

    // ========================================================================
    // PART 10: Real-World Example - Data Processing Pipeline
    // ========================================================================
    println!("--- Part 10: Data Processing Pipeline ---\n");

    #[derive(Debug, Clone)]
    struct Transaction {
        amount: f64,
        category: String,
        approved: bool,
    }

    let transactions = vec![
        Transaction { amount: 100.0, category: "Food".to_string(), approved: true },
        Transaction { amount: 50.0, category: "Transport".to_string(), approved: true },
        Transaction { amount: 200.0, category: "Food".to_string(), approved: false },
        Transaction { amount: 75.0, category: "Entertainment".to_string(), approved: true },
        Transaction { amount: 150.0, category: "Food".to_string(), approved: true },
    ];

    // Calculate total approved food expenses
    let total_food: f64 = transactions.iter()
        .filter(|t| t.approved)                    // Only approved
        .filter(|t| t.category == "Food")          // Only food
        .map(|t| t.amount)                         // Extract amount
        .sum();                                    // Sum it up

    println!("Total approved food expenses: ${:.2}", total_food);

    // Group by category and sum
    use std::collections::HashMap;
    let mut category_totals: HashMap<String, f64> = HashMap::new();

    transactions.iter()
        .filter(|t| t.approved)
        .for_each(|t| {
            *category_totals.entry(t.category.clone()).or_insert(0.0) += t.amount;
        });

    println!("\nCategory totals:");
    for (category, total) in &category_totals {
        println!("  {}: ${:.2}", category, total);
    }

    println!();

    // ========================================================================
    // PART 11: Closure Composition
    // ========================================================================
    println!("--- Part 11: Closure Composition ---\n");

    // Composing closures to build complex transformations

    let add_10 = |x: i32| x + 10;
    let multiply_2 = |x: i32| x * 2;
    let square = |x: i32| x * x;

    // Manual composition
    let result = square(multiply_2(add_10(5)));
    println!("Manual composition: {}", result);  // ((5 + 10) * 2)^2 = 900

    // Using iterator chains for composition
    let result = vec![5].into_iter()
        .map(add_10)
        .map(multiply_2)
        .map(square)
        .next()
        .unwrap();
    println!("Iterator composition: {}", result);

    println!();

    // ========================================================================
    // PART 12: What Happens Under the Hood
    // ========================================================================
    println!("--- Part 12: What Happens Under the Hood ---\n");

    println!("When you write:");
    println!("  let x = 5;");
    println!("  let f = |y| x + y;");
    println!();
    println!("The compiler generates something like:");
    println!("  struct Closure {{ x: i32 }}");
    println!("  impl Closure {{");
    println!("      fn call(&self, y: i32) -> i32 {{");
    println!("          self.x + y");
    println!("      }}");
    println!("  }}");
    println!("  let f = Closure {{ x: 5 }};");
    println!();
    println!("This is why closures are ZERO-COST:");
    println!("- No heap allocation (unless you Box them)");
    println!("- No function pointer indirection (inlined)");
    println!("- Just a struct on the stack!");

    println!();

    println!("=== Program Complete ===");
}

// ============================================================================
// ADDITIONAL DEMONSTRATIONS
// ============================================================================

/// Example: Returning a closure
/// Must use 'impl Fn' or Box<dyn Fn> since closure size is unknown
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor  // 'move' required to transfer ownership
}

/// Example: Closure that implements FnMut
fn _demonstrate_fn_mut() {
    let mut numbers = vec![1, 2, 3];

    // This closure mutates the vector
    let mut append = |x| numbers.push(x);

    append(4);
    append(5);

    println!("Numbers after append: {:?}", numbers);
}

/// Example: Closure as parameter with different traits
fn _call_fn<F>(f: F) where F: Fn() {
    f();
    f();  // Can call multiple times
}

fn _call_fn_mut<F>(mut f: F) where F: FnMut() {
    f();
    f();  // Can call multiple times, may mutate
}

fn _call_fn_once<F>(f: F) where F: FnOnce() {
    f();
    // f();  // ❌ ERROR: Can only call once
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Closures are anonymous functions that capture their environment
// 2. Three traits: Fn (immutable), FnMut (mutable), FnOnce (consuming)
// 3. Compiler auto-chooses the most flexible trait based on usage
// 4. Use 'move' to force ownership transfer into closure
// 5. Closures are ZERO-COST - compile to structs with methods
// 6. Iterators are lazy - no work until consumed
// 7. Iterator chains compile to efficient loops (zero-cost abstraction)
// 8. Can implement custom iterators with the Iterator trait
// 9. Higher-order functions enable powerful composition
// 10. Functional style in Rust is as fast as imperative loops!

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Trying to call FnOnce closure multiple times
// ❌ Forgetting 'move' when returning closures or spawning threads
// ❌ Not understanding capture semantics (borrow vs move)
// ❌ Forgetting to consume lazy iterators (no .collect() or .sum())
// ❌ Fighting borrow checker with closures (restructure instead)
// ❌ Overusing Box<dyn Fn> when impl Fn would work
// ❌ Not realizing closures are zero-cost (same as manual code)
// ❌ Using function pointers when closures with captures are needed

// ============================================================================
// ZERO-COST ABSTRACTION EXPLAINED IN DEPTH
// ============================================================================
//
// "Zero-cost" means the abstraction has no runtime overhead compared to
// writing the code manually. For closures and iterators:
//
// 1. Closures are monomorphized (specialized) at compile time
//    - Each closure gets its own type
//    - Type is a struct with captured variables
//    - Methods are inlined
//
// 2. Iterator chains are fused by the compiler
//    - Multiple map/filter/etc. become a single loop
//    - No intermediate allocations
//    - LLVM can optimize the whole chain together
//
// 3. Example transformation:
//
//    Source code:
//      data.iter().filter(|x| x % 2 == 0).map(|x| x * 2).sum()
//
//    Compiles to (roughly):
//      let mut sum = 0;
//      for x in &data {
//          if x % 2 == 0 {
//              sum += x * 2;
//          }
//      }
//      sum
//
// This is why Rust can have beautiful, functional code that's as fast as C!

// ============================================================================
// COMPARISON: RUST vs OTHER LANGUAGES
// ============================================================================
//
// JavaScript:
//   - Closures capture by reference (can cause bugs)
//   - Always heap-allocated
//   - Runtime overhead
//   - No ownership semantics
//
// Python:
//   - Closures capture by reference
//   - 'nonlocal' keyword for mutations
//   - Interpreted, slow
//   - Generator expressions similar to lazy iterators
//
// C++:
//   - Lambda syntax: [captures](params) { body }
//   - Can capture by value or reference (explicit)
//   - No automatic trait system like Fn/FnMut/FnOnce
//   - Can be zero-cost if inlined
//
// Rust:
//   - Automatic trait selection (Fn/FnMut/FnOnce)
//   - Compile-time ownership checking
//   - Zero-cost (stack allocation + inlining)
//   - Type-safe, memory-safe
//   - Best of all worlds!
