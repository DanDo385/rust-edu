# Project 43: Declarative Macros

## Overview
Learn to write powerful declarative macros using `macro_rules!`. This project demonstrates macro patterns, token trees, repetition, code generation, and creating domain-specific languages (DSLs) in Rust.

## Concepts Taught
- **macro_rules!** for declarative macros
- **Token trees** and pattern matching
- **Repetition** with `$(...)*` and `$(...)+`
- **Fragment specifiers**: `expr`, `ident`, `ty`, `pat`, `stmt`, `block`, `item`
- **Macro hygiene** and scope
- **Code generation** at compile-time
- **DSL creation** for cleaner APIs
- **Debugging macros** with `cargo expand`

## Why Rust Macros Are Different

### Compile-Time Code Generation
Rust macros run **during compilation**, not runtime. They:
- Generate Rust code from patterns
- Have zero runtime cost
- Are type-checked after expansion
- Enable powerful abstractions without performance penalty

### Hygiene
Rust macros are **hygienic** - variables declared in macros don't accidentally clash with variables in the calling code. This prevents subtle bugs common in C macros.

**Comparison with other languages:**
- **C/C++ Macros**: Text substitution (dangerous, no hygiene)
- **Lisp Macros**: Powerful but runtime cost
- **Python Decorators**: Runtime transformation (slower)
- **Go**: No macros (intentionally simple)

## Beginner Pitfalls & Best Practices

### Pitfall 1: Fragment Specifier Confusion
```rust
// ❌ WRONG: Using wrong fragment type
macro_rules! bad {
    ($x:expr) => {
        let $x = 5;  // expr can't be used as identifier!
    };
}
```
**Fix**: Use correct specifier:
```rust
// ✅ CORRECT: Use ident for variable names
macro_rules! good {
    ($x:ident) => {
        let $x = 5;
    };
}
```

### Pitfall 2: Missing Repetition Separators
```rust
// ❌ WRONG: Repetition without separator
macro_rules! sum {
    ($($x:expr)*) => {
        $($x)+  // This concatenates, doesn't add!
    };
}
```
**Fix**: Add separator:
```rust
// ✅ CORRECT: Use + operator between repetitions
macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}
```

### Pitfall 3: Not Handling Empty Repetitions
```rust
// ❌ WRONG: sum!() would expand to "0 +" which is invalid
macro_rules! sum {
    ($($x:expr),+) => {  // + requires at least one
        0 $(+ $x)*
    };
}
```
**Fix**: Handle empty case or use `*` instead of `+`:
```rust
// ✅ CORRECT: Works with zero or more arguments
macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*  // Expands to just "0" if empty
    };
}
```

### Pitfall 4: Order of Macro Arms
```rust
// ❌ WRONG: Specific pattern after general pattern (unreachable)
macro_rules! bad {
    ($x:expr) => { ... };           // Matches everything
    ($x:expr, $y:expr) => { ... };  // Never reached!
}
```
**Fix**: Put specific patterns first:
```rust
// ✅ CORRECT: Specific before general
macro_rules! good {
    ($x:expr, $y:expr) => { ... };  // Try this first
    ($x:expr) => { ... };           // Fallback
}
```

## Code Walkthrough

See `src/main.rs` for a complete implementation that demonstrates:
1. Simple macros for code generation
2. Macros with repetition patterns
3. Multiple macro arms (pattern matching)
4. Fragment specifiers (expr, ident, ty, etc.)
5. Building a mini DSL
6. HashMap initialization macro
7. Test generation macro
8. Debugging and expanding macros

## Macro Fragment Specifiers

| Specifier | Matches | Example |
|-----------|---------|---------|
| `expr` | Expression | `1 + 2`, `foo()`, `vec![1, 2]` |
| `ident` | Identifier | `foo`, `bar`, `MyStruct` |
| `ty` | Type | `i32`, `Vec<String>`, `&str` |
| `pat` | Pattern | `Some(x)`, `42`, `_` |
| `stmt` | Statement | `let x = 5;`, `foo();` |
| `block` | Code block | `{ let x = 5; x + 1 }` |
| `item` | Item | `fn foo() {}`, `struct Bar {}` |
| `tt` | Token tree | Any single token or `{...}`, `[...]`, `(...)` |
| `meta` | Meta item | Used in attributes |
| `lifetime` | Lifetime | `'a`, `'static` |
| `vis` | Visibility | `pub`, `pub(crate)` |
| `literal` | Literal | `42`, `"hello"`, `true` |
| `path` | Path | `std::vec::Vec`, `crate::foo` |

## Macro Repetition Syntax

```rust
$(...)*  // Zero or more repetitions
$(...)+  // One or more repetitions
$(...)?  // Zero or one repetition (optional)
```

**With separators:**
```rust
$($x:expr),*     // Comma-separated: a, b, c
$($x:expr);*     // Semicolon-separated: a; b; c
$($x:expr)*      // No separator: abc
```

## Performance Considerations

**Compile-time cost**: Complex macros can slow down compilation. Each macro expansion is re-type-checked.

**Runtime cost**: **ZERO**! Macros are expanded during compilation. The final binary has no macro overhead.

**Code bloat**: Macros that generate lots of code can increase binary size. Use generic functions when possible.

**Benchmarks**:
- Simple macro expansion: <1ms
- Complex macro with lots of repetition: 1-10ms
- Procedural macros (derive): 10-100ms

## Comparison: Macros vs Functions vs Generics

| Feature | Macros | Functions | Generics |
|---------|--------|-----------|----------|
| When runs | Compile-time | Runtime | Compile-time (monomorphization) |
| Overhead | Zero | Function call | Zero |
| Flexibility | Very high | Medium | High |
| Type safety | After expansion | Full | Full |
| Code size | Can increase | Small | Can increase (monomorphization) |
| Debugging | Harder | Easy | Medium |

**When to use:**
- **Macros**: Different syntax, variable arguments, code generation
- **Functions**: Most code (easier to read, debug, test)
- **Generics**: Type-agnostic code with type safety

## Additional Challenges

1. **assert_eq! reimplementation**: Create your own version of `assert_eq!` with custom error messages.

2. **SQL-like DSL**: Build a macro for type-safe SQL-like queries.

3. **JSON literal**: Create a `json!` macro similar to `serde_json::json!`.

4. **Timing macro**: Build a macro that times code execution.

5. **Enum variants to strings**: Generate a function that converts enum variants to their string names.

6. **Bitflags macro**: Create a macro for defining bitflag types.

7. **Builder pattern**: Generate builder pattern code from a struct definition.

## Future Directions

- **Procedural macros**: More powerful, custom derive, attribute macros
- **syn and quote**: Libraries for writing proc macros
- **Build custom derives**: Like #[derive(Debug)] but custom
- **Compiler plugins**: Experimental, cutting-edge metaprogramming

## Running This Project

```bash
cd 43-declarative-macros
cargo run
```

To see macro expansions:
```bash
cargo install cargo-expand
cargo expand
```

**Note**: No external dependencies needed! Macros are part of the language.

## Expected Output

The program will:
1. Demonstrate simple macro expansions
2. Show macros with repetition
3. Build a HashMap using a custom macro
4. Create a mini DSL for configuration
5. Generate code for different types
6. Show debugging techniques for macros

## Macro Hygiene Example

```rust
macro_rules! declare_var {
    ($name:ident) => {
        let temp = 42;  // This 'temp' doesn't conflict with outer scope
        let $name = temp;
    };
}

fn main() {
    let temp = 100;
    declare_var!(x);
    println!("{}", temp);  // Prints 100, not 42 (hygiene!)
}
```

## Common Macro Patterns

### 1. Variable Arguments
```rust
macro_rules! print_all {
    ($($arg:expr),*) => {
        $(println!("{:?}", $arg);)*
    };
}
```

### 2. Internal Rules (Helper Patterns)
```rust
macro_rules! complex {
    // Public interface
    ($x:expr) => {
        complex!(@internal $x, 0)
    };

    // Internal helper (@ prefix is convention)
    (@internal $x:expr, $y:expr) => {
        $x + $y
    };
}
```

### 3. TT Munching (Incremental Processing)
```rust
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => {
        1 + count!($($tail)*)
    };
}
```

### 4. Callback Pattern
```rust
macro_rules! call_with {
    ($callback:ident, $value:expr) => {
        $callback!($value)
    };
}
```

## Debugging Macros

### 1. Use `cargo expand`
Shows the expanded code after macro processing.

### 2. Use `log_syntax!` (nightly)
Prints tokens at compile-time.

### 3. Use `trace_macros!` (nightly)
Shows macro expansion steps.

### 4. Simplify and test incrementally
Build complex macros piece by piece.

## Advanced Features

### 1. Import/Export
```rust
#[macro_export]  // Make macro available to other crates
macro_rules! my_macro { ... }
```

### 2. Macro Scope
```rust
// Macros are scoped - define before use
macro_rules! foo { ... }
foo!();  // OK

bar!();  // Error: macro not yet defined
macro_rules! bar { ... }
```

### 3. Following Set
After an `expr`, you can only match: `,`, `;`, or `=>`
After an `ident`, you can match almost anything.

This prevents ambiguity in macro parsing.

## Real-World Examples

Popular macros in the Rust ecosystem:
- `println!`, `format!`: String formatting
- `vec!`: Vector initialization
- `assert!`, `assert_eq!`: Testing
- `derive`: Procedural macro for trait implementation
- `serde_json::json!`: JSON literal syntax
- `lazy_static!`: Lazy static initialization
- `matches!`: Pattern matching as boolean
