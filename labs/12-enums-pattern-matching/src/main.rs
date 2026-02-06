// Project 05: Enums and Pattern Matching
//
// Enums define types with multiple possible variants.
// Pattern matching ensures you handle all cases safely.

fn main() {
    println!("=== Enums and Pattern Matching ===\n");

    // ============================================================================
    // BASIC ENUMS
    // ============================================================================

    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    println!("Direction: {:?}", dir);

    // Pattern matching with match
    match dir {
        Direction::North => println!("Heading north!"),
        Direction::South => println!("Heading south!"),
        Direction::East => println!("Heading east!"),
        Direction::West => println!("Heading west!"),
    }

    println!();

    // ============================================================================
    // ENUMS WITH DATA
    // ============================================================================

    // Each variant can hold different types of data!
    #[derive(Debug)]
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },   // Named fields (like a struct)
        Write(String),              // Single value
        ChangeColor(u8, u8, u8),   // Tuple
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        process_message(msg);
    }

    println!();

    // ============================================================================
    // OPTION<T> - RUST'S NULL REPLACEMENT
    // ============================================================================

    // Rust doesn't have null! Instead, it has Option<T>
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // You MUST handle the None case
    match some_number {
        Some(n) => println!("The number is: {}", n),
        None => println!("No number!"),
    }

    // if let for simple cases
    if let Some(n) = some_number {
        println!("Got a number: {}", n);
    } else {
        println!("Got nothing!");
    }

    // Common Option methods
    let doubled = some_number.map(|n| n * 2);  // Some(10)
    println!("Doubled: {:?}", doubled);

    let unwrapped = some_number.unwrap();  // Gets value or panics if None
    println!("Unwrapped: {}", unwrapped);

    let default = no_number.unwrap_or(0);  // Use default if None
    println!("With default: {}", default);

    println!();

    // ============================================================================
    // RESULT<T, E> - ERROR HANDLING
    // ============================================================================

    // Result is used for operations that can fail
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero!"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // ============================================================================
    // EXHAUSTIVE MATCHING
    // ============================================================================

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
        // If you forget a variant, the compiler will ERROR!
        // This prevents bugs
    }

    let coin = Coin::Quarter;
    println!("Value: {} cents", value_in_cents(coin));

    println!();

    // ============================================================================
    // PATTERNS AND DESTRUCTURING
    // ============================================================================

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Destructure in match
    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6: {}", addr);
        }
    }

    // Match with guards (additional conditions)
    let number = Some(7);
    match number {
        Some(n) if n < 5 => println!("Less than 5: {}", n),
        Some(n) if n >= 5 => println!("Greater or equal to 5: {}", n),
        None => println!("No number"),
        _ => println!("Something else"),
    }

    println!();

    // ============================================================================
    // PRACTICAL EXAMPLE: TRAFFIC LIGHT SYSTEM
    // ============================================================================

    #[derive(Debug, PartialEq)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn time_to_wait(&self) -> u32 {
            match self {
                TrafficLight::Red => 60,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 45,
            }
        }

        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Green => TrafficLight::Yellow,
                TrafficLight::Yellow => TrafficLight::Red,
            }
        }

        fn can_go(&self) -> bool {
            match self {
                TrafficLight::Green => true,
                _ => false,  // Red or Yellow
            }
        }
    }

    let mut light = TrafficLight::Red;
    println!("Current light: {:?}", light);
    println!("Wait time: {} seconds", light.time_to_wait());
    println!("Can go? {}", light.can_go());

    light = light.next();
    println!("After change: {:?}", light);
    println!("Can go? {}", light.can_go());

    println!();

    // ============================================================================
    // PRACTICAL EXAMPLE: CALCULATOR
    // ============================================================================

    #[derive(Debug)]
    enum Operation {
        Add(f64, f64),
        Subtract(f64, f64),
        Multiply(f64, f64),
        Divide(f64, f64),
    }

    impl Operation {
        fn execute(&self) -> Result<f64, String> {
            match self {
                Operation::Add(a, b) => Ok(a + b),
                Operation::Subtract(a, b) => Ok(a - b),
                Operation::Multiply(a, b) => Ok(a * b),
                Operation::Divide(a, b) => {
                    if *b == 0.0 {
                        Err(String::from("Cannot divide by zero"))
                    } else {
                        Ok(a / b)
                    }
                }
            }
        }
    }

    let ops = vec![
        Operation::Add(10.0, 5.0),
        Operation::Subtract(10.0, 5.0),
        Operation::Multiply(10.0, 5.0),
        Operation::Divide(10.0, 5.0),
        Operation::Divide(10.0, 0.0),
    ];

    for op in ops {
        match op.execute() {
            Ok(result) => println!("{:?} = {}", op, result),
            Err(e) => println!("{:?} -> Error: {}", op, e),
        }
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Enums define types with multiple variants
// 2. Variants can hold different types of data
// 3. match is exhaustive - must handle all cases
// 4. Option<T> replaces null (Some/None)
// 5. Result<T, E> is for error handling (Ok/Err)
// 6. if let simplifies single-case matches
// 7. Pattern matching can destructure data
// 8. Compiler ensures you handle all cases
// 9. No null pointer errors in Rust!
// 10. Enums + match = type-safe, expressive code
