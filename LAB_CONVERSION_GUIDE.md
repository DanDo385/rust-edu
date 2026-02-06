# Lab Conversion Framework: Converting main.rs → Full Teaching Labs

## Overview

This guide helps you convert incomplete labs (11-60, currently just `main.rs`) into complete teaching labs matching the quality and structure of labs 01-10.

**Conversion Path**: `main.rs` (raw code) → Full project structure with lib.rs, solution.rs, tests, comprehensive README

## What Makes a Complete Lab?

### Structure
```
labs/NN-lab-name/
├── Cargo.toml              (✅ auto-generated)
├── README.md               (⚠️ needs enhancement)
├── src/
│   ├── lib.rs             (🚫 missing - create!)
│   ├── exercise.rs        (optional - for complex labs)
│   ├── solution.rs        (🚫 missing - create!)
│   └── main.rs            (✅ exists)
└── tests/
    └── integration_test.rs (🚫 missing - create!)
```

### Key Files & Their Purpose

**lib.rs** (Exercise Skeleton)
- Function signatures with doc comments
- `todo!()` placeholders for students
- Hints in comments
- Example usage in doc comments
- Module-level documentation

**solution.rs** (Exhaustive Teaching)
- Complete implementations
- Deep explanations of every symbol (& * :: -> ?)
- Ownership & borrowing analysis
- Memory layout diagrams
- Complexity analysis
- Comparative explanations (vs JavaScript, Python, Go)
- 400-1500 lines per lab

**integration_test.rs** (Comprehensive Tests)
- Unit tests (happy path + edge cases)
- Integration tests (multiple functions together)
- Property-based tests (invariant verification)
- Each test has explanatory comments
- 50-100+ tests per lab

**README.md** (Pedagogical Guide)
- Plain English explanation
- New Rust concepts introduced
- Syntax examples with explanations
- How to run commands
- Exercise descriptions
- Solution explanation (no code, just ideas)
- Comparisons to other languages
- Common mistakes & fixes
- Stretch goals
- What's next

**main.rs** (Demo)
- Orchestration code
- Calls functions with example inputs
- Shows usage patterns
- Should compile and demonstrate functionality

## Step-by-Step Conversion Process

### Phase 1: Extract & Analyze (30 min)

1. **Read the existing main.rs**
   ```bash
   cat labs/11-control-flow/src/main.rs | wc -l  # Get size
   ```

2. **Understand what it does**
   - What functions/concepts does it teach?
   - What are the key learning points?
   - What Rust concepts are involved?

3. **Identify extractable functions**
   - Can you extract 3-5 core functions from the main.rs?
   - Each should teach a distinct concept
   - Should be progressively more complex

### Phase 2: Create lib.rs (1-2 hours)

1. **Extract function signatures** from main.rs
   ```rust
   // Template structure:
   pub fn function_name(param: Type) -> ReturnType {
       // TODO: [description of what to do]
       // Hint: [helpful hint]
       todo!("Complete this function")
   }
   ```

2. **Add documentation to each function**
   ```rust
   /// Brief one-liner description.
   ///
   /// Longer description explaining what this teaches.
   ///
   /// # Parameters
   /// - `param`: Description with type explanation
   ///
   /// # Returns
   /// Description of return type
   ///
   /// # Example
   /// ```ignore
   /// use module::function_name;
   /// assert_eq!(function_name(input), expected);
   /// ```
   pub fn function_name(param: Type) -> ReturnType {
       todo!("...")
   }
   ```

3. **Add module-level documentation**
   ```rust
   //! # Lab NN - Topic Name
   //!
   //! This lab teaches you about:
   //! - Concept 1
   //! - Concept 2
   //! - Concept 3
   //!
   //! ## Your Task
   //! Implement the functions below...
   ```

### Phase 3: Create solution.rs (2-4 hours)

1. **Copy implementations** from main.rs
   ```rust
   pub fn function_name(param: Type) -> ReturnType {
       // ... actual implementation
   }
   ```

2. **Add exhaustive documentation** to each function
   ```rust
   /// Function description
   ///
   /// ## What This Function Does
   /// Explanation in plain English
   ///
   /// ## Parameters
   /// - `param: Type` - Detailed explanation
   ///   - `param` = what it is
   ///   - `:` = "has type" separator
   ///   - `Type` = detailed explanation of this type
   ///
   /// ## Returns
   /// Detailed explanation
   ///
   /// ## Example
   /// ```ignore
   /// let result = function_name(input);
   /// assert_eq!(result, expected);
   /// ```
   ///
   /// ## Ownership & Borrowing Analysis
   /// [If relevant: explain borrowing/ownership implications]
   ///
   /// ## Memory Layout
   /// [For complex types, draw ASCII memory diagram]
   ///
   /// ## Time/Space Complexity
   /// O(n) or O(1), explain why
   pub fn function_name(param: Type) -> ReturnType {
       // Actual code
   }
   ```

3. **Add module-level documentation**
   ```rust
   //! # Lab NN - Complete Solution
   //!
   //! ## What We're Building
   //! [Explanation of concepts taught]
   //!
   //! ## Key Rust Concepts You'll Learn
   //! - Concept 1
   //! - Concept 2
   //!
   //! ## Time Complexity: [analysis]
   //! ## Space Complexity: [analysis]
   ```

### Phase 4: Create integration_test.rs (2-3 hours)

1. **Create test file structure**
   ```rust
   //! Integration tests for lab-NN
   use module_name::solution::*;

   // ========================================
   // TESTS FOR: function_name
   // ========================================
   ```

2. **Write unit tests** (happy path + edge cases)
   ```rust
   #[test]
   fn test_function_name_basic() {
       // Test basic case
       assert_eq!(function_name(input), expected);
   }

   #[test]
   fn test_function_name_edge_case() {
       // Test edge case (empty, zero, negative, etc)
       assert_eq!(function_name(edge_input), expected);
   }
   ```

3. **Write integration tests** (multiple functions together)
   ```rust
   #[test]
   fn test_integration_functions_together() {
       // Test multiple functions working together
       let result1 = function1(input);
       let result2 = function2(result1);
       assert_eq!(result2, expected);
   }
   ```

4. **Write property-based tests** (invariant verification)
   ```rust
   #[test]
   fn test_property_invariant() {
       // Test that a mathematical property holds
       for x in 0..100 {
           let result = function(x);
           assert!(result >= 0, "Result must be positive");
       }
   }
   ```

### Phase 5: Enhance README.md (1-2 hours)

See the template below for complete README structure.

### Phase 6: Test & Verify (30 min)

```bash
# From repo root:
cd labs/NN-lab-name

# Check lib.rs compiles
cargo check --lib

# Check solution.rs compiles
cargo check --lib --features solution

# Run tests
cargo test

# Run main
cargo run

# Format code
cargo fmt
```

## Templates

### lib.rs Template
See `templates/lib.rs.template`

### solution.rs Template
See `templates/solution.rs.template`

### integration_test.rs Template
See `templates/integration_test.rs.template`

### README.md Template
See `templates/README.md.template`

## Conversion Checklist

For each lab (11-60):

- [ ] Analyzed main.rs and extracted key functions
- [ ] Created lib.rs with function signatures and todos
- [ ] Created solution.rs with exhaustive documentation
- [ ] Created integration_test.rs with 20+ tests
- [ ] Enhanced README.md with full pedagogical structure
- [ ] Tests compile and pass (`cargo test`)
- [ ] main.rs still works (`cargo run`)
- [ ] lib.rs has proper module documentation
- [ ] solution.rs has function-level docs with ownership analysis
- [ ] README has Memory Model section (if applicable)
- [ ] README has Symbol Deep Dive section (if applicable)
- [ ] Verified with `cargo fmt` and `cargo check`

## Time Estimates

| Phase | Time | Notes |
|-------|------|-------|
| Analyze | 30 min | Quick read of main.rs |
| lib.rs | 1-2 hr | Extract functions + document |
| solution.rs | 2-4 hr | Implement + exhaust docs |
| tests | 2-3 hr | Comprehensive test coverage |
| README | 1-2 hr | Full pedagogical structure |
| Verify | 30 min | Test everything works |
| **TOTAL** | **7-13 hr** | Per lab |

## Priority Labs (Do First)

1. **Lab 11** - control-flow (fundamental)
2. **Lab 15** - lifetimes (critical concept)
3. **Lab 20** - multithreading-basics (important systems concept)
4. **Lab 21** - async-basics (increasingly important)
5. **Lab 25** - transaction-pool (domain-specific)

Then work through systematically: 12-14, 16-19, 22-24, 26-60

## Tools & Scripts

### Converting a Lab Automatically

```bash
./scripts/convert_lab.sh labs/11-control-flow
```

(See `scripts/convert_lab.sh` for script that bootstraps structure)

### Generating lib.rs from main.rs

```bash
./scripts/extract_functions.py labs/11-control-flow/src/main.rs > /tmp/lib.rs.draft
# Then manually edit and refine
```

## Common Patterns

### Pattern 1: Algorithm Teaching
```rust
/// Finds the [something] in [collection]
///
/// ## What This Function Does
/// [Plain English explanation]
///
/// ## How It Works
/// 1. [Step 1]
/// 2. [Step 2]
/// 3. [Step 3]
```

### Pattern 2: Type System Teaching
```rust
/// Demonstrates [Type] usage
///
/// ## Why This Type
/// - [Reason 1]
/// - [Reason 2]
///
/// ## Compared to [Other Language]
/// [Code comparison]
```

### Pattern 3: Ownership Teaching
```rust
/// ## Ownership & Borrowing Analysis
///
/// - Parameters are BORROWED (&T)
///   - The & means we're borrowing, not taking ownership
///   - The caller retains ownership
///   - We can only READ the data
///   - WHY: [reason]
///   - After function: parameters disappear (they were refs)
///
/// - Return value is OWNED (T)
///   - No &, so ownership transfers
///   - We create data on the [stack/heap]
///   - Caller receives ownership
///   - WHY: [reason]
///
/// ## Memory Layout
/// ```ignore
/// Stack:                          Heap:
/// ┌─────────────┐
/// │ param: &T   │──────────▶  [actual data]
/// └─────────────┘
/// ```
```

## Support

- **Questions?** Check existing labs 01-10 as examples
- **Stuck?** Look at the corresponding main.rs for inspiration
- **Issues?** See CLAUDE.md for refactoring context

## Example: Converting Lab 11 (control-flow)

See `examples/lab-11-conversion.md` for a complete step-by-step example.
