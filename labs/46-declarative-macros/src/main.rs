// Project 43: Declarative Macros
//
// Demonstrates macro_rules!, token trees, code generation, and DSL creation.
// Macros are Rust's metaprogramming tool for reducing boilerplate and creating custom syntax.

// ============================================================================
// EXAMPLE 1: SIMPLE MACROS
// ============================================================================

// Basic macro that prints with a prefix
macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

// Macro that takes an argument
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// Macro with multiple arms (pattern matching)
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (multiply $a:expr, $b:expr) => {
        $a * $b
    };
    (power $base:expr, $exp:expr) => {
        ($base as f64).powf($exp as f64)
    };
}

// ============================================================================
// EXAMPLE 2: MACROS WITH REPETITION
// ============================================================================

// Sum multiple values
macro_rules! sum {
    // Base case: single value
    ($x:expr) => {
        $x
    };
    // Recursive case: multiple values
    ($x:expr, $($rest:expr),+) => {
        $x + sum!($($rest),+)
    };
}

// Create a Vec with custom syntax
macro_rules! make_vec {
    ($($element:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($element);
        )*
        v
    }};
}

// Print all arguments with labels
macro_rules! debug_vars {
    ($($var:expr),+) => {
        $(
            println!("{} = {:?}", stringify!($var), $var);
        )+
    };
}

// ============================================================================
// EXAMPLE 3: HASHMAP INITIALIZATION MACRO
// ============================================================================

// Initialize a HashMap with nice syntax: hashmap!{ "key" => value }
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

// ============================================================================
// EXAMPLE 4: DSL FOR CONFIGURATION
// ============================================================================

// Mini DSL for defining configuration
macro_rules! config {
    (
        $struct_name:ident {
            $(
                $field:ident : $field_type:ty = $default:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $struct_name {
            $(
                $field: $field_type,
            )*
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $(
                        $field: $default,
                    )*
                }
            }
        }

        impl $struct_name {
            fn new() -> Self {
                Self::default()
            }
        }
    };
}

// ============================================================================
// EXAMPLE 5: ENUM WITH STRING CONVERSION
// ============================================================================

// Generate enum with automatic to_string() and from_string()
macro_rules! string_enum {
    (
        $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum $name {
            $($variant),*
        }

        impl $name {
            fn to_string(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                }
            }

            fn from_string(s: &str) -> Option<Self> {
                match s {
                    $(
                        stringify!($variant) => Some($name::$variant),
                    )*
                    _ => None,
                }
            }

            fn all_variants() -> &'static [&'static str] {
                &[
                    $(
                        stringify!($variant),
                    )*
                ]
            }
        }
    };
}

// ============================================================================
// EXAMPLE 6: TIMING MACRO
// ============================================================================

// Time the execution of a block of code
macro_rules! time_it {
    ($label:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("{}: {:?}", $label, duration);
        result
    }};
}

// ============================================================================
// EXAMPLE 7: CUSTOM ASSERT MACRO
// ============================================================================

// Custom assert with detailed error message
macro_rules! assert_between {
    ($value:expr, $min:expr, $max:expr) => {
        if !($min <= $value && $value <= $max) {
            panic!(
                "Assertion failed: {} is not between {} and {} (actual value: {})",
                stringify!($value),
                $min,
                $max,
                $value
            );
        }
    };
}

// ============================================================================
// EXAMPLE 8: STRUCT BUILDER PATTERN
// ============================================================================

// Generate a builder pattern for a struct
macro_rules! builder {
    (
        $struct_name:ident {
            $(
                $field:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Default)]
        struct $struct_name {
            $(
                $field: $field_type,
            )*
        }

        paste::paste! {
            impl $struct_name {
                fn new() -> Self {
                    Self::default()
                }

                $(
                    fn [<with_ $field>](mut self, value: $field_type) -> Self {
                        self.$field = value;
                        self
                    }
                )*

                fn build(self) -> Self {
                    self
                }
            }
        }
    };
}

// ============================================================================
// EXAMPLE 9: TT MUNCHER (COUNTING)
// ============================================================================

// Count the number of tokens using TT munching
macro_rules! count_tokens {
    () => { 0 };
    ($head:tt $($tail:tt)*) => {
        1 + count_tokens!($($tail)*)
    };
}

// ============================================================================
// EXAMPLE 10: INTERNAL RULES
// ============================================================================

// Macro with internal helper rules (using @ prefix convention)
macro_rules! fancy_sum {
    // Public interface: start with first element
    ($first:expr $(, $rest:expr)*) => {
        fancy_sum!(@internal 0, $first $(, $rest)*)
    };

    // Internal rule: accumulate sum
    (@internal $acc:expr, $head:expr $(, $tail:expr)*) => {
        fancy_sum!(@internal $acc + $head $(, $tail)*)
    };

    // Base case: return accumulated sum
    (@internal $acc:expr) => {
        $acc
    };
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

fn main() {
    println!("=== Declarative Macros ===\n");

    // Example 1: Simple macros
    println!("1. Simple Macros");
    say_hello!();
    greet!("World");
    greet!("Rust");
    println!("  5 + 3 = {}", calculate!(add 5, 3));
    println!("  5 * 3 = {}", calculate!(multiply 5, 3));
    println!("  2 ^ 10 = {}", calculate!(power 2, 10));

    println!("
{}
", "=".repeat(60));

    // Example 2: Macros with repetition
    println!("2. Macros with Repetition");
    println!("  sum!(1, 2, 3, 4, 5) = {}", sum!(1, 2, 3, 4, 5));

    let my_vec = make_vec![1, 2, 3, 4, 5];
    println!("  make_vec![1, 2, 3, 4, 5] = {:?}", my_vec);

    let x = 10;
    let y = "hello";
    let z = vec![1, 2, 3];
    println!("\n  Debug variables:");
    debug_vars!(x, y, z);

    println!("
{}
", "=".repeat(60));

    // Example 3: HashMap initialization
    println!("3. HashMap Initialization Macro");
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    println!("  Scores: {:?}", scores);

    let config_map = hashmap! {
        "host" => "localhost",
        "port" => "8080",
        "protocol" => "http",
    };
    println!("  Config: {:?}", config_map);

    println!("
{}
", "=".repeat(60));

    // Example 4: Configuration DSL
    println!("4. Configuration DSL");

    config! {
        ServerConfig {
            host: String = "127.0.0.1".to_string(),
            port: u16 = 8080,
            max_connections: usize = 100,
        }
    }

    let server = ServerConfig::new();
    println!("  Default server config: {:?}", server);

    println!("
{}
", "=".repeat(60));

    // Example 5: String enum
    println!("5. Enum with String Conversion");

    string_enum! {
        Color {
            Red,
            Green,
            Blue,
            Yellow,
        }
    }

    let color = Color::Red;
    println!("  Color: {} (as string: {})", color.to_string(), color.to_string());
    println!("  All colors: {:?}", Color::all_variants());

    if let Some(parsed) = Color::from_string("Green") {
        println!("  Parsed 'Green': {} = {}", parsed.to_string(), parsed == Color::Green);
    }

    println!("
{}
", "=".repeat(60));

    // Example 6: Timing macro
    println!("6. Timing Macro");

    let result = time_it!("Sleep 100ms", {
        std::thread::sleep(std::time::Duration::from_millis(100));
        "Done!"
    });
    println!("  Result: {}", result);

    time_it!("Calculate sum", {
        let sum: i32 = (1..=1000).sum();
        println!("  Sum of 1..=1000 = {}", sum);
    });

    println!("
{}
", "=".repeat(60));

    // Example 7: Custom assert
    println!("7. Custom Assert Macro");

    let value = 50;
    assert_between!(value, 0, 100);
    println!("  ✓ assert_between!({}, 0, 100) passed", value);

    let score = 85;
    assert_between!(score, 0, 100);
    println!("  ✓ assert_between!({}, 0, 100) passed", score);

    // This would panic:
    // assert_between!(150, 0, 100);

    println!("
{}
", "=".repeat(60));

    // Example 8: TT Muncher (counting)
    println!("8. Token Counting (TT Muncher)");

    let count1 = count_tokens!(a b c d e);
    println!("  count_tokens!(a b c d e) = {}", count1);

    let count2 = count_tokens!(1 + 2 + 3);
    println!("  count_tokens!(1 + 2 + 3) = {}", count2);

    println!("
{}
", "=".repeat(60));

    // Example 9: Internal rules
    println!("9. Macro with Internal Rules");

    let sum1 = fancy_sum!(1, 2, 3, 4, 5);
    println!("  fancy_sum!(1, 2, 3, 4, 5) = {}", sum1);

    println!("
{}
", "=".repeat(60));

    // Example 10: Macro expansion demonstration
    println!("10. Understanding Macro Expansion");
    println!("  Macros expand to regular Rust code at compile-time.");
    println!("  Use 'cargo expand' to see the expanded code!");
    println!("\n  Example expansion of vec![1, 2, 3]:");
    println!("    {{");
    println!("        let mut temp_vec = Vec::new();");
    println!("        temp_vec.push(1);");
    println!("        temp_vec.push(2);");
    println!("        temp_vec.push(3);");
    println!("        temp_vec");
    println!("    }}");

    println!("\n=== Macros Complete ===");
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. MACRO EXPANSION
//    - Macros expand during compilation, before type checking
//    - The compiler matches tokens against macro patterns
//    - Matched tokens are substituted into the macro body
//    - Result is regular Rust code that gets type-checked
//
// 2. HYGIENE
//    - Variables defined in macros don't conflict with outer scope
//    - Each macro expansion gets its own "syntax context"
//    - This prevents accidental variable capture
//    - C-style macros don't have this safety!
//
// 3. FRAGMENT SPECIFIERS
//    - Each $x:TYPE is parsed according to Rust's grammar
//    - expr: full expression (respects precedence)
//    - ident: identifier (variable/function/type name)
//    - ty: type (i32, Vec<String>, etc.)
//    - This ensures valid Rust syntax
//
// 4. REPETITION
//    - $(...)*  matches zero or more times
//    - $(...)+  matches one or more times
//    - Separators (like ,) are part of the pattern
//    - Repetition is expanded at compile-time
//
// 5. ZERO RUNTIME COST
//    - All macro processing happens at compile-time
//    - The final binary contains expanded code, not macros
//    - No performance penalty compared to writing code manually

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Macros generate code at compile-time (zero runtime cost)
// 2. macro_rules! uses pattern matching on token trees
// 3. Fragment specifiers ensure valid Rust syntax
// 4. Repetition ($(...)*) enables variable arguments
// 5. Hygiene prevents variable name conflicts
// 6. Multiple arms allow pattern matching (like match)
// 7. Internal rules (@ prefix) help organize complex macros
// 8. stringify! converts code to strings (for debugging)
// 9. Macros can't do everything - use functions when possible
// 10. cargo expand shows macro expansions (very useful for debugging!)

// ============================================================================
// MACRO DESIGN PRINCIPLES
// ============================================================================
// 1. Keep macros simple - complex logic belongs in functions
// 2. Use descriptive names (action_noun pattern)
// 3. Document with examples (macros can be hard to understand)
// 4. Test macro output (use #[test] functions)
// 5. Consider whether a function or generic would work instead
// 6. Use @internal rules for complex multi-step macros
// 7. Make macros composable (macros can call other macros)
// 8. Handle edge cases (empty input, single item, etc.)

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Using wrong fragment specifier (expr vs ident vs ty)
// ❌ Forgetting separators in repetition ($(...),*)
// ❌ Not handling empty repetition cases
// ❌ Putting general patterns before specific ones
// ❌ Forgetting trailing comma support ($(,)?)
// ❌ Using macros when functions would work (harder to debug)
// ❌ Not using stringify! for debugging
// ❌ Expecting macros to have access to runtime values

// ============================================================================
// WHEN TO USE MACROS VS FUNCTIONS
// ============================================================================
// USE MACROS FOR:
// ✓ Variable number of arguments (println!)
// ✓ Custom syntax / DSLs (vec![1, 2, 3])
// ✓ Code generation (reducing boilerplate)
// ✓ Compile-time operations
// ✓ Different types (can't use generics)
//
// USE FUNCTIONS FOR:
// ✓ Regular logic (easier to read, debug, test)
// ✓ Runtime calculations
// ✓ Anything that doesn't need special syntax
// ✓ Most code (default choice)

// ============================================================================
// ADVANCED MACRO TECHNIQUES
// ============================================================================
// 1. TT Munching: Process tokens incrementally
// 2. Internal rules: Helper patterns with @ prefix
// 3. Callback pattern: Pass macro names as arguments
// 4. Push-down accumulation: Build result incrementally
// 5. Incremental parsing: Parse complex syntax step-by-step
// 6. Macro recursion: Macros calling themselves
// 7. #[macro_export]: Make macros public
// 8. #[macro_use]: Import macros from other crates

// ============================================================================
// DEBUGGING TIPS
// ============================================================================
// 1. Use `cargo expand` to see expanded code
// 2. Use `stringify!` to print macro arguments
// 3. Test each arm separately
// 4. Start simple, add complexity gradually
// 5. Use `dbg!` macro to inspect values
// 6. Check fragment specifier types carefully
// 7. Test with edge cases (empty, single item, many items)
// 8. Read compiler errors carefully (they show expansion context)

// ============================================================================
// REAL-WORLD MACRO EXAMPLES
// ============================================================================
// - println!: Formatted printing with variable arguments
// - vec!: Vector initialization [1, 2, 3]
// - assert!: Testing with custom messages
// - format!: String formatting (returns String)
// - dbg!: Debug printing with file/line info
// - matches!: Pattern matching as boolean
// - lazy_static!: Lazy initialization of statics
// - derive!: Procedural macro for trait implementation (different kind)
// - serde_json::json!: JSON literal syntax
// - tokio::select!: Async operation selection

// ============================================================================
// LIMITATIONS OF DECLARATIVE MACROS
// ============================================================================
// - Can't introspect types (use procedural macros)
// - Can't access external data at compile-time
// - Limited error messages (improving)
// - Can make code harder to understand
// - IDE support is limited (improving)
// - Can't generate new identifiers easily (use procedural macros)
// - Compile times can increase with heavy macro use
