// Project 01: Variables and Types
//
// This program demonstrates Rust's fundamental type system, variables,
// and mutability rules. We'll explore all primitive types and see how
// Rust's compiler enforces type safety and immutability.

fn main() {
    println!("=== Rust Variables and Types ===\n");

    // ============================================================================
    // IMMUTABILITY BY DEFAULT
    // ============================================================================
    // In Rust, variables are IMMUTABLE by default. This means once you bind
    // a value to a variable, you cannot change it. This prevents bugs caused
    // by unexpected mutations.

    let x = 5;  // Immutable variable
    println!("The value of x is: {}", x);

    // x = 6;  // ❌ This would cause a compile error!
    // ERROR: cannot assign twice to immutable variable `x`

    // To make a variable mutable, use the `mut` keyword:
    let mut y = 10;
    println!("The value of y is: {}", y);

    y = 15;  // ✅ This is OK because y is mutable
    println!("Now y is: {}", y);

    println!();

    // ============================================================================
    // TYPE INFERENCE
    // ============================================================================
    // Rust can infer types from the value and usage. You don't always need
    // to explicitly specify types (unlike in C or Go).

    let inferred = 42;  // Rust infers this as i32 (32-bit signed integer)
    println!("Inferred type (i32): {}", inferred);

    // You can also explicitly specify the type:
    let explicit: i32 = 42;
    println!("Explicit type (i32): {}", explicit);

    // Type annotations are required when the compiler can't infer the type:
    let parsed: i32 = "42".parse().expect("Not a number!");
    println!("Parsed from string: {}", parsed);

    println!();

    // ============================================================================
    // INTEGER TYPES
    // ============================================================================
    // Rust has multiple integer types with different sizes and signedness:
    // Signed: i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    // The number indicates the number of bits.

    let small_int: i8 = 127;           // 8-bit signed: -128 to 127
    let medium_int: i16 = 32_767;      // 16-bit signed: -32,768 to 32,767
    let default_int: i32 = 2_147_483_647;  // 32-bit signed (DEFAULT)
    let big_int: i64 = 9_223_372_036_854_775_807;  // 64-bit signed
    let huge_int: i128 = 1_000_000_000_000_000_000;  // 128-bit signed

    println!("i8:   {}", small_int);
    println!("i16:  {}", medium_int);
    println!("i32:  {} (default integer type)", default_int);
    println!("i64:  {}", big_int);
    println!("i128: {}", huge_int);

    // Unsigned integers (no negative values, but larger positive range):
    let byte: u8 = 255;                // 8-bit unsigned: 0 to 255
    let unsigned: u32 = 4_294_967_295; // 32-bit unsigned

    println!("u8:  {}", byte);
    println!("u32: {}", unsigned);

    // Architecture-dependent sizes (32 or 64 bits depending on the target):
    let pointer_sized: isize = 100;  // Signed, pointer-sized
    let upointer_sized: usize = 200; // Unsigned, pointer-sized (used for indexing)

    println!("isize: {}", pointer_sized);
    println!("usize: {} (used for array/vec indexing)", upointer_sized);

    println!();

    // ============================================================================
    // FLOATING-POINT TYPES
    // ============================================================================
    // Rust has two floating-point types: f32 (single precision) and f64 (double)
    // f64 is the DEFAULT because modern CPUs make it roughly the same speed as f32

    let float32: f32 = 3.14159;      // 32-bit float
    let float64: f64 = 2.71828;      // 64-bit float (DEFAULT)
    let inferred_float = 1.618;      // Inferred as f64

    println!("f32: {}", float32);
    println!("f64: {} (default)", float64);
    println!("Inferred float: {}", inferred_float);

    println!();

    // ============================================================================
    // BOOLEAN TYPE
    // ============================================================================
    // Booleans are 1 byte in size and can be true or false

    let is_rust_awesome: bool = true;
    let is_learning_hard = false;  // Type inferred

    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is learning hard? {}", is_learning_hard);

    println!();

    // ============================================================================
    // CHARACTER TYPE
    // ============================================================================
    // The char type is Rust's most primitive alphabetic type.
    // It's 4 bytes in size and represents a Unicode Scalar Value.
    // This means it can represent more than just ASCII!

    let letter: char = 'A';
    let emoji: char = '😎';
    let chinese: char = '中';

    println!("ASCII character: {}", letter);
    println!("Emoji: {}", emoji);
    println!("Chinese character: {}", chinese);

    println!();

    // ============================================================================
    // STRING TYPES: String vs &str
    // ============================================================================
    // Rust has TWO main string types:
    // 1. &str (string slice) - Immutable, fixed-size, usually borrowed
    // 2. String - Mutable, growable, heap-allocated

    // &str - String slice (fast, immutable)
    let string_slice: &str = "Hello, Rust!";
    println!("String slice (&str): {}", string_slice);

    // String literals are &str by default
    let literal = "I'm a &str";
    println!("String literal: {}", literal);

    // String - Owned string (can grow, modify, but slower to create)
    let owned_string: String = String::from("Hello, World!");
    println!("Owned String: {}", owned_string);

    // Converting &str to String:
    let converted: String = "Convert me".to_string();
    println!("Converted to String: {}", converted);

    // Why two types?
    // - &str is a VIEW into string data (doesn't own it, can't modify)
    // - String OWNS its data (can modify, but uses heap memory)
    // Think of &str as a "window" and String as the "actual house"

    println!();

    // ============================================================================
    // TYPE CASTING
    // ============================================================================
    // Rust doesn't do implicit type conversion (unlike C).
    // You must explicitly cast with the `as` keyword.

    let integer = 65;
    let float = integer as f64;       // Cast i32 to f64
    let character = integer as u8 as char;  // Cast to char (ASCII 'A')

    println!("Integer: {}", integer);
    println!("As float: {}", float);
    println!("As char: {}", character);

    // Be careful with casting - you can lose data!
    let big_number: i64 = 1000;
    let small_number = big_number as i8;  // Truncation happens!
    println!("i64 {} cast to i8 becomes: {}", big_number, small_number);

    println!();

    // ============================================================================
    // CONSTANTS vs VARIABLES
    // ============================================================================
    // Constants are ALWAYS immutable and must have a type annotation.
    // They can be declared in any scope, including global.
    // Their value must be computable at compile time.

    const MAX_SCORE: u32 = 100_000;
    println!("Constant MAX_SCORE: {}", MAX_SCORE);

    // Difference between `const` and immutable `let`:
    // - `const` must be a compile-time constant
    // - `let` can be the result of a runtime computation
    // - `const` can be global
    // - `const` never allocates memory (inlined by compiler)

    println!();

    // ============================================================================
    // SHADOWING
    // ============================================================================
    // You can declare a new variable with the same name as a previous variable.
    // This is called "shadowing" and is different from mutability.

    let shadow = 5;
    println!("First shadow: {}", shadow);

    let shadow = shadow + 1;  // New variable, shadows the old one
    println!("Second shadow: {}", shadow);

    let shadow = "Now I'm a string!";  // Can even change type!
    println!("Third shadow: {}", shadow);

    // Why is this useful?
    // 1. You can change the type of a value
    // 2. You can transform a value without making it mutable
    // 3. The old value is dropped (freed) when shadowed

    println!();

    // ============================================================================
    // FORMATTED PRINTING
    // ============================================================================
    // The println! macro supports various formatting options

    let name = "Alice";
    let age = 30;
    let height = 5.6;

    // Basic interpolation
    println!("Name: {}, Age: {}, Height: {}", name, age, height);

    // Named arguments
    println!("Hello, {name}! You are {age} years old.", name = name, age = age);

    // Debug printing (requires Debug trait)
    println!("Debug: {:?}", (name, age, height));

    // Pretty debug printing
    println!("Pretty debug: {:#?}", (name, age, height));

    // Number formatting
    println!("Decimal: {}, Hex: {:x}, Binary: {:b}", 255, 255, 255);

    // Padding and alignment
    println!("Padded: {:>10} {:>10}", name, age);

    println!();

    // ============================================================================
    // WHAT RUST DOES UNDER THE HOOD
    // ============================================================================
    // All these variables are stored on the STACK (not heap).
    // The stack is FAST - allocating is just moving a pointer.
    // The compiler knows the size of all these types at compile time.
    //
    // String (not &str) is special - it stores its data on the HEAP,
    // but the String struct itself (pointer, length, capacity) is on the stack.
    //
    // When this function ends, all stack variables are "popped" off
    // and any heap allocations (like String) are automatically freed.
    // This is Rust's ownership system at work!

    println!("=== Program Complete ===");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Variables are IMMUTABLE by default - use `mut` to make them mutable
// 2. Rust has STRONG type inference but is statically typed
// 3. Multiple integer and float types - choose based on your needs
// 4. char is 4 bytes and supports Unicode (not just ASCII)
// 5. Two string types: &str (borrowed) and String (owned)
// 6. Type casting must be EXPLICIT with `as`
// 7. Constants are compile-time values, `let` can be runtime
// 8. Shadowing lets you reuse variable names and change types
// 9. Everything happens at compile-time or on the stack (very fast!)
// 10. The compiler enforces correctness - if it compiles, it's probably correct

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ let x = 5; x = 6;  (forgot mut)
// ❌ let s: String = "hello";  (wrong type, should be &str or use .to_string())
// ❌ let x: i32 = 3.14;  (type mismatch, needs cast or different type)
// ❌ let big: i8 = 200;  (overflow, i8 max is 127)
// ❌ println!(x);  (forgot format string, should be println!("{}", x))
