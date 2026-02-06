// Project 02: Control Flow and Functions
//
// This program demonstrates Rust's control flow structures through a number
// guessing game. We'll explore if/else, loops, match expressions, and functions.

use std::io;
use std::cmp::Ordering;

fn main() {
    println!("=== Rust Control Flow & Functions ===\n");

    // First, let's explore control flow concepts
    demonstrate_if_else();
    demonstrate_loops();
    demonstrate_match();

    println!("\n=== Now let's play a guessing game! ===\n");

    // Play the guessing game
    guessing_game();
}

// ============================================================================
// IF/ELSE EXPRESSIONS
// ============================================================================
// In Rust, if is an EXPRESSION (returns a value), not just a statement

fn demonstrate_if_else() {
    println!("--- If/Else Expressions ---");

    let number = 7;

    // Basic if/else
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is exactly 5");
    } else {
        println!("Number is greater than 5");
    }

    // if as an expression (returns a value!)
    // This is similar to ternary operators in other languages
    let comparison = if number < 10 { "small" } else { "large" };
    println!("The number is: {}", comparison);

    // Both branches must return the same type
    let condition = true;
    let result = if condition {
        5  // Returns i32
    } else {
        6  // Also returns i32 - must match!
    };
    println!("Result: {}", result);

    // IMPORTANT: No truthy/falsy values in Rust!
    // Conditions must be EXACTLY bool type
    // if number {  // ❌ This is an ERROR - number is not bool
    if number != 0 {  // ✅ Must explicitly compare
        println!("Number is not zero");
    }

    println!();
}

// ============================================================================
// LOOPS
// ============================================================================
// Rust has three kinds of loops: loop, while, and for

fn demonstrate_loops() {
    println!("--- Loops ---");

    // 1. LOOP - infinite loop until you break
    // This is the most primitive loop type
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2;  // break can return a value!
        }
    };
    println!("Loop result: {}", result);

    // 2. WHILE - loop while condition is true
    let mut countdown = 3;
    print!("Countdown: ");
    while countdown > 0 {
        print!("{}... ", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // 3. FOR - iterate over a collection or range
    // This is the safest and most common loop
    print!("For loop (range): ");
    for i in 1..4 {  // 1..4 is exclusive of 4 (1, 2, 3)
        print!("{} ", i);
    }
    println!();

    print!("For loop (inclusive range): ");
    for i in 1..=3 {  // 1..=3 includes 3 (1, 2, 3)
        print!("{} ", i);
    }
    println!();

    // Iterating over a collection
    let numbers = [10, 20, 30, 40];
    print!("For loop (array): ");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();

    // Loop with index (enumeration)
    print!("For loop (with index): ");
    for (index, value) in numbers.iter().enumerate() {
        print!("{}:{} ", index, value);
    }
    println!("\n");

    // WHY SO MANY LOOP TYPES?
    // - `loop`: When you want infinite loops with explicit breaks
    // - `while`: When you have a clear condition to check
    // - `for`: When iterating over collections (safest, prevents off-by-one errors)
}

// ============================================================================
// MATCH EXPRESSIONS
// ============================================================================
// Match is like switch, but more powerful and EXHAUSTIVE (must cover all cases)

fn demonstrate_match() {
    println!("--- Match Expressions ---");

    let number = 13;

    // Match with multiple patterns
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Something else"),  // _ is the catch-all pattern
    }

    // Match with expression return values
    let description = match number {
        n if n < 0 => "negative",
        0 => "zero",
        1..=100 => "between 1 and 100",
        _ => "something else",
    };
    println!("Number is: {}", description);

    // Match is EXHAUSTIVE - you must handle all possible cases
    // The compiler will error if you forget a case!

    println!();
}

// ============================================================================
// FUNCTIONS
// ============================================================================
// Functions in Rust use snake_case naming convention

// Function with no parameters, no return value
fn say_hello() {
    println!("Hello from a function!");
}

// Function with parameters
fn add_numbers(a: i32, b: i32) -> i32 {
    // The last expression is implicitly returned (no `return` keyword needed)
    // NO SEMICOLON at the end means this is an expression, not a statement
    a + b
}

// Function with explicit return
fn subtract_numbers(a: i32, b: i32) -> i32 {
    return a - b;  // You can use `return` if you want to return early
}

// Function with multiple returns (early return pattern)
fn describe_number(n: i32) -> &'static str {
    if n < 0 {
        return "negative";  // Early return
    }
    if n == 0 {
        return "zero";
    }
    "positive"  // Final return (implicit)
}

// Function that doesn't return anything (returns unit type `()`)
fn print_number(n: i32) {
    println!("The number is: {}", n);
    // Implicitly returns () - the "unit" type
}

// ============================================================================
// GUESSING GAME
// ============================================================================
// Let's put it all together in a simple guessing game

fn guessing_game() {
    // For this demo, we'll use a fixed secret number
    // In a real game, you'd use: rand::thread_rng().gen_range(1..=100)
    let secret_number = 42;

    println!("I'm thinking of a number between 1 and 100.");
    println!("(Hint: it's 42 - but try different numbers to see the logic!)");

    // Infinite loop - we'll break when they guess correctly
    loop {
        println!("\nPlease input your guess:");

        // Create a new mutable String to store the input
        let mut guess = String::new();

        // Read a line from stdin
        // read_line returns a Result<usize, Error>
        // We use .expect() to crash if reading fails
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string to a number
        // trim() removes whitespace (including the newline)
        // parse() converts string to the type we specify
        // This returns Result<i32, ParseIntError>
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,  // Successfully parsed
            Err(_) => {
                println!("Please enter a valid number!");
                continue;  // Skip to next loop iteration
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        // cmp() returns an Ordering enum: Less, Greater, or Equal
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🎉");
                break;  // Exit the loop
            }
        }
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ZERO-COST ABSTRACTIONS
//    All these control flow structures compile to efficient assembly.
//    A `for` loop over a range compiles to the same code as a raw loop.
//
// 2. MATCH COMPILATION
//    The compiler optimizes `match` expressions into jump tables or
//    decision trees - very fast at runtime.
//
// 3. FUNCTION INLINING
//    Small functions like add_numbers() are inlined by the compiler.
//    There's NO function call overhead in the final binary.
//
// 4. EXPRESSION-BASED DESIGN
//    Because if/match are expressions, the compiler can optimize better.
//    It knows the value will be used, so it can eliminate dead code.
//
// 5. NO GARBAGE COLLECTION
//    All variables here are on the stack. When a function returns,
//    the stack is popped instantly - no GC pauses.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. if/match are EXPRESSIONS (return values)
// 2. No implicit type coercion - conditions must be exactly bool
// 3. Three loop types: loop (infinite), while (conditional), for (iteration)
// 4. match is EXHAUSTIVE - must handle all cases
// 5. Functions return the last expression (no semicolon)
// 6. Use snake_case for function names
// 7. Error handling with Result and expect/unwrap
// 8. User input requires handling potential errors

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ if number { ... }  (number is not bool)
// ❌ fn add(a, b) -> i32 { a + b; }  (semicolon makes it return (), not i32)
// ❌ match n { 1 => ..., 2 => ... }  (not exhaustive, missing _ case)
// ❌ let x = if true { 5 } else { "six" };  (type mismatch)
// ❌ stdin().read_line(&mut s)  (not handling Result)
