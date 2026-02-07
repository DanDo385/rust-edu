// Lab 46: Declarative Macros
//
// This module defines several declarative macros using `macro_rules!` and
// helper functions that demonstrate metaprogramming in Rust.
//
// Key concepts:
// - macro_rules! syntax and pattern matching
// - Fragment specifiers (expr, ident, ty, tt)
// - Repetition patterns ($(...),*)
// - Macro hygiene
// - Internal rules with @ prefix
// - Zero-cost compile-time code generation

// ============================================================================
// SIMPLE MACROS
// ============================================================================

/// A macro that takes an expression and returns a greeting string.
#[macro_export]
macro_rules! greet {
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}

/// A macro with multiple arms for arithmetic operations.
///
/// Supports: `calculate!(add a, b)`, `calculate!(multiply a, b)`, `calculate!(power base, exp)`
#[macro_export]
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
// MACROS WITH REPETITION
// ============================================================================

/// Sum multiple values using recursive macro expansion.
///
/// Usage: `sum!(1, 2, 3, 4, 5)` returns `15`
#[macro_export]
macro_rules! sum {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        $x + sum!($($rest),+)
    };
}

/// Create a Vec with custom syntax, similar to the std `vec!` macro.
///
/// Usage: `make_vec![1, 2, 3]` returns `vec![1, 2, 3]`
#[macro_export]
macro_rules! make_vec {
    ($($element:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($element);
        )*
        v
    }};
}

// ============================================================================
// HASHMAP INITIALIZATION MACRO
// ============================================================================

/// Initialize a HashMap with `=>` syntax.
///
/// Usage: `hashmap!{ "key1" => value1, "key2" => value2 }`
#[macro_export]
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
// CUSTOM ASSERT MACRO
// ============================================================================

/// Assert that a value falls between a minimum and maximum (inclusive).
///
/// Panics with a descriptive message if the value is out of range.
///
/// Usage: `assert_between!(value, 0, 100)`
#[macro_export]
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
// TOKEN COUNTING MACRO (TT MUNCHER)
// ============================================================================

/// Count the number of token trees passed to the macro.
///
/// Usage: `count_tokens!(a b c d e)` returns `5`
#[macro_export]
macro_rules! count_tokens {
    () => { 0usize };
    ($head:tt $($tail:tt)*) => {
        1usize + count_tokens!($($tail)*)
    };
}

// ============================================================================
// INTERNAL RULES MACRO
// ============================================================================

/// Sum values using internal accumulator rules.
///
/// Uses the `@internal` convention for helper rules that should not
/// be called directly by users.
///
/// Usage: `fancy_sum!(1, 2, 3, 4, 5)` returns `15`
#[macro_export]
macro_rules! fancy_sum {
    ($first:expr $(, $rest:expr)*) => {
        fancy_sum!(@internal 0, $first $(, $rest)*)
    };

    (@internal $acc:expr, $head:expr $(, $tail:expr)*) => {
        fancy_sum!(@internal $acc + $head $(, $tail)*)
    };

    (@internal $acc:expr) => {
        $acc
    };
}

// ============================================================================
// STRING ENUM MACRO
// ============================================================================

/// Generate an enum with automatic `to_str()`, `from_str()`, and `all_variants()` methods.
///
/// Usage:
/// ```
/// use declarative_macros::string_enum;
///
/// string_enum! {
///     Color {
///         Red,
///         Green,
///         Blue,
///     }
/// }
///
/// let c = Color::Red;
/// assert_eq!(c.to_str(), "Red");
/// assert_eq!(Color::from_str("Green"), Some(Color::Green));
/// ```
#[macro_export]
macro_rules! string_enum {
    (
        $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        stringify!($variant) => Some($name::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn all_variants() -> &'static [&'static str] {
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
// TIMING MACRO
// ============================================================================

/// Time the execution of a block and return its result.
///
/// Returns a tuple of `(result, duration)`.
///
/// Usage:
/// ```ignore
/// let (result, elapsed) = time_it!({ expensive_computation() });
/// ```
#[macro_export]
macro_rules! time_it {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        (result, duration)
    }};
}

// ============================================================================
// CONFIG DSL MACRO
// ============================================================================

/// A DSL for defining configuration structs with default values.
///
/// Generates a struct with a `Default` impl and a `new()` constructor.
///
/// Usage:
/// ```
/// use declarative_macros::config;
///
/// config! {
///     AppConfig {
///         host: String = "localhost".to_string(),
///         port: u16 = 8080,
///     }
/// }
///
/// let cfg = AppConfig::new();
/// assert_eq!(cfg.port, 8080);
/// ```
#[macro_export]
macro_rules! config {
    (
        $struct_name:ident {
            $(
                $field:ident : $field_type:ty = $default:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq)]
        pub struct $struct_name {
            $(
                pub $field: $field_type,
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
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}

// ============================================================================
// HELPER FUNCTIONS (for testing macros from integration tests)
// ============================================================================

/// Demonstrate the `calculate!` macro with addition.
pub fn calculate_add(a: i32, b: i32) -> i32 {
    calculate!(add a, b)
}

/// Demonstrate the `calculate!` macro with multiplication.
pub fn calculate_multiply(a: i32, b: i32) -> i32 {
    calculate!(multiply a, b)
}

/// Demonstrate the `calculate!` macro with power.
pub fn calculate_power(base: f64, exp: f64) -> f64 {
    calculate!(power base, exp)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. MACRO EXPANSION
//    - Macros expand during compilation, before type checking
//    - The compiler matches tokens against macro patterns
//    - Result is regular Rust code that gets type-checked
//
// 2. HYGIENE
//    - Variables defined in macros don't conflict with outer scope
//    - Each expansion gets its own syntax context
//    - This prevents accidental variable capture
//
// 3. FRAGMENT SPECIFIERS
//    - expr: full expression, ident: identifier, ty: type, tt: token tree
//    - Each fragment is parsed according to Rust's grammar
//
// 4. ZERO RUNTIME COST
//    - All macro processing happens at compile-time
//    - The final binary contains expanded code, not macros
