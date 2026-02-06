# Lab Conversion Quick Reference Card

Print this page or keep it open while converting labs!

## One-Line Checklist

- [ ] `./scripts/convert_lab.sh labs/NN-lab-name` (bootstrap)
- [ ] Edit `src/lib.rs` (function stubs + todos)
- [ ] Edit `src/solution.rs` (implementations + docs)
- [ ] Edit `tests/integration_test.rs` (20+ tests)
- [ ] Edit `README.md` (pedagogy)
- [ ] `cargo test` ✅ and `cargo run` ✅

## File Structure Template

```
labs/NN-lab-name/
├── Cargo.toml                      # ✅ auto-generated
├── README.md                       # Edit me: add pedagogy
├── src/
│   ├── lib.rs                      # Create: function stubs
│   ├── solution.rs                 # Create: implementations
│   ├── main.rs                     # ✅ already exists
│   └── [optional: exercise.rs]     # For complex labs
└── tests/
    └── integration_test.rs         # Create: comprehensive tests
```

## lib.rs Template (30 lines per function)

```rust
//! # Lab NN - Topic
//! [Brief overview]

/// One-liner description
///
/// This teaches you about:
/// - [Concept 1]
/// - [Concept 2]
///
/// # Parameters
/// - `param: Type` - [What is it]
///
/// # Returns
/// [What is returned]
///
/// # Example
/// ```ignore
/// assert_eq!(function(input), expected);
/// ```
pub fn function(param: Type) -> ReturnType {
    todo!("Implement...")
}
```

## solution.rs Template (150+ lines per function)

```rust
/// Description
///
/// ## What This Function Does
/// [Plain English explanation]
///
/// ## Parameters
/// - `param: Type` - [Deep explanation]
///   - `param` = [what]
///   - `:` = [symbol meaning]
///   - `Type` = [why this type]
///
/// ## Returns
/// [Explanation]
///
/// ## Example
/// ```ignore
/// assert_eq!(function(input), expected);
/// ```
///
/// ## Ownership & Borrowing Analysis
/// - Parameters: BORROWED/OWNED and why
/// - Return value: BORROWED/OWNED and why
///
/// ## Memory Layout
/// ```ignore
/// Stack:                  Heap:
/// [ASCII diagram]         [ASCII diagram]
/// ```
///
/// ## Common Mistakes
/// 1. [Mistake]: [Fix]
///
pub fn function(param: Type) -> ReturnType {
    // Implementation from main.rs
}
```

## integration_test.rs Structure

```rust
use module::solution::*;

// ============================================================================
// function_name tests
// ============================================================================

#[test]
fn test_function_basic() {
    // Basic/happy path case
    assert_eq!(function(input), expected);
}

#[test]
fn test_function_edge_case_1() {
    // Edge case: [what edge case]
    assert_eq!(function(edge_input), expected);
}

#[test]
fn test_function_property() {
    // Property: [invariant that must hold]
    assert!(result.property());
}
```

## README.md Sections (Copy from template!)

```markdown
# Lab NN - Topic

## What You're Building (Plain English)
[Simple explanation]

## New Rust Concepts in This Lab
- **Concept 1**: [One sentence]
- **Concept 2**: [One sentence]

## Rust Syntax You'll See
[Code examples with comments]

## How to Run
[cargo commands]

## The Exercises
[List of functions to implement]

## Solution Explanation (No Code - Just Ideas)
[How would you solve this?]

## Where Rust Shines
[Comparisons to other languages]

## Common Beginner Mistakes & How to Avoid Them
[List of mistakes with fixes]

## Stretch Goals
[Challenge problems]

## What's Next?
[Link to next lab]
```

## Key Documentation Patterns

### Parameter Explanation
```rust
/// - `param: &str` - Let's break this down:
///   - `param` = parameter name (you choose, be descriptive)
///   - `:` = "has type" separator
///   - `&` = borrow operator (not taking ownership)
///   - `str` = string slice type
///
///   The `&str` means [why we use this type]
```

### Ownership Analysis
```rust
/// ## Ownership & Borrowing Analysis
///
/// - Parameters `param` are BORROWED (&T)
///   - The `&` means we're borrowing, not taking ownership
///   - The caller still owns the original data
///   - We can only READ the data (immutable borrow)
///   - WHY: [reason]
///   - After this function ends: [what happens]
///
/// - Return value is OWNED (T)
///   - No `&`, so ownership transfers
///   - We create [on stack/heap]
///   - Caller receives ownership
///   - WHY: [reason]
```

### Memory Diagram
```rust
/// ## Memory Layout
///
/// ```ignore
/// Stack:                      Heap:
/// ┌──────────────┐
/// │ var: Type    │──────▶  [data]
/// │ ptr: 0x1000  │
/// └──────────────┘
/// ```
```

### Comparison to Other Languages
```rust
/// **Compared to JavaScript:**
/// ```javascript
/// // JavaScript code example
/// ```
///
/// **Compared to Python:**
/// ```python
/// # Python code example
/// ```
///
/// **Rust advantages:**
/// - [Advantage 1]
/// - [Advantage 2]
```

## Test Patterns (Copy-Paste Ready!)

### Unit Test
```rust
#[test]
fn test_function_basic() {
    // Test basic/happy path case
    assert_eq!(function(input), expected);
}
```

### Edge Case Test
```rust
#[test]
fn test_function_empty() {
    // Test edge case: empty input
    assert_eq!(function(&[]), expected);
}
```

### Property-Based Test
```rust
#[test]
fn test_property_invariant() {
    // Property: [statement that must always be true]
    for i in 0..100 {
        let result = function(i);
        assert!(result >= 0);
    }
}
```

### Integration Test
```rust
#[test]
fn test_integration_functions_together() {
    // Test multiple functions working together
    let result1 = function1(input);
    let result2 = function2(result1);
    assert_eq!(result2, expected);
}
```

## Common Mistakes to Document

1. **Off-by-one errors in loops**
   - **Fix**: Use iterators instead of manual indexing

2. **Not handling edge cases**
   - **Fix**: Start with empty/zero/negative cases

3. **Forgetting exhaustiveness in match**
   - **Fix**: Use `_` catch-all or handle all variants

4. **Copying when borrowing would work**
   - **Fix**: Use `&` and `&mut` references

5. **Panicking instead of returning Option**
   - **Fix**: Use `Option<T>` or `Result<T, E>`

## Commands (Keyboard-Ready!)

```bash
# Bootstrap a lab
./scripts/convert_lab.sh labs/NN-lab-name

# Work on the lab
cd labs/NN-lab-name

# Test it
cargo test                        # Run all tests
cargo test test_function_name     # Run specific test
cargo test -- --nocapture        # Show println! output

# Check code
cargo check                       # Does it compile?
cargo clippy                      # Are there warnings?
cargo fmt                         # Format code
cargo fmt -- --check             # Check if formatted

# Run
cargo run                         # Run main.rs
cargo run --release              # Optimized version

# Back to repo
cd ../..
```

## Time Estimates

| Phase | Time | What you're doing |
|-------|------|-------------------|
| Analyze | 20 min | Read main.rs, understand concepts |
| lib.rs | 45 min | Extract functions, add todos |
| solution.rs | 90 min | Implement, add exhaustive docs |
| tests | 90 min | Write 20-40+ comprehensive tests |
| README | 60 min | Enhance with pedagogy |
| Testing | 30 min | Verify everything works |
| **TOTAL** | **~6 hours** | **Per lab** |

## Documentation Minimum

**Every function MUST have:**
- ✅ One-line description
- ✅ "What This Function Does" section
- ✅ Parameters section (with type breakdown)
- ✅ Returns section
- ✅ Example usage
- ✅ If relevant: Ownership & borrowing analysis
- ✅ If complex: Memory layout diagram

**Every lib.rs MUST have:**
- ✅ Module-level doc comment
- ✅ List of concepts taught
- ✅ Hints in todo!() comments
- ✅ Example usage in doc comments

**Every README MUST have:**
- ✅ Plain English explanation
- ✅ New Rust Concepts section
- ✅ How to Run commands
- ✅ The Exercises section
- ✅ Where Rust Shines comparison
- ✅ Common Mistakes & Fixes

## When Stuck

1. **Don't know how to document?**
   → Look at existing lab (01-10) with same concept

2. **Don't know what tests to write?**
   → Look at tests/integration_test.rs in existing labs

3. **Don't know what to put in README?**
   → Use `templates/README.md.template`

4. **Don't know how to explain a symbol?**
   → Check Symbol Deep Dive examples in solution.rs files

5. **Don't know the time complexity?**
   → Look at how your code iterates or recurses

## Resources at Your Fingertips

```bash
# View templates
ls templates/

# See an example
cat examples/CONVERSION_EXAMPLE.md

# Read the full guide
cat LAB_CONVERSION_GUIDE.md

# Check progress
cat CONVERSION_PROGRESS.md

# Reference another lab
ls labs/01-variables-types/
```

## When Your Tests Fail

```bash
# See detailed output
cargo test test_name -- --nocapture --test-threads=1

# Debug a specific test
cargo test test_name --lib

# See which tests exist
cargo test --lib -- --list
```

## When Code Won't Compile

```bash
# Check what's wrong
cargo check

# Get more details
cargo build

# Get detailed error info
RUST_BACKTRACE=1 cargo check

# Format might help
cargo fmt
```

---

**PRO TIP**: Keep this open in a split screen while converting! Copy-paste templates and adjust.

Last updated: 2026-02-06
