//! # A Tree-Walking Interpreter - Your Implementation
//!
//! This project is about building a full interpreter for a simple arithmetic
//! language. You will implement it in three main parts: the lexer, the parser,
//! and the evaluator.
//!
//! ## Your Task
//!
//! You need to fill in the logic in the three sub-modules: `lexer.rs`,
//! `parser.rs`, and `evaluator.rs`.
//!
//! 1.  **`lexer.rs`**: Implement the `tokenize` function to turn a string into
//!     a `Vec<Token>`.
//! 2.  **`parser.rs`**: Implement the `parse` function to turn a `Vec<Token>`
//!     into an Abstract Syntax Tree (`Expr`). This is the hardest part!
//! 3.  **`evaluator.rs`**: Implement the `evaluate` function to walk the `Expr`
//!     tree and compute the final `f64` result.
//!
//! This main `lib.rs` file ties them all together.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p interpreter
//! cargo run -p interpreter
//! ```
//!
//! ## Stuck?
//!
//! Check out the corresponding files in `src/solution/` for a complete,
//! heavily-commented solution for each part.

// Declare the modules. The content of these modules will be in
// `src/lexer.rs`, `src/parser.rs`, and `src/evaluator.rs`.
pub mod evaluator;
pub mod lexer;
pub mod parser;

use thiserror::Error;
use lexer::{LexerError, tokenize};
use parser::{ParseError, parse};
use evaluator::{EvalError, evaluate};

/// A top-level error type that encapsulates all possible failures.
#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    #[error("Lexer Error: {0}")]
    Lexer(#[from] LexerError),
    #[error("Parser Error: {0}")]
    Parser(#[from] ParseError),
    #[error("Evaluation Error: {0}")]
    Evaluator(#[from] EvalError),
}

/// Interprets a mathematical expression from a string.
///
/// This function orchestrates the three main phases:
/// 1. Tokenizing (Lexing)
/// 2. Parsing
/// 3. Evaluating
pub fn interpret(input: &str) -> Result<f64, InterpreterError> {
    // TODO: Implement the interpretation pipeline.
    // 1. Call `tokenize()` from the `lexer` module. Use `?` to propagate errors.
    // 2. Call `parse()` from the `parser` module. Use `?` to propagate errors.
    // 3. Call `evaluate()` from the `evaluator` module. Use `?` to propagate errors.
    // 4. Return the final result.
    todo!("Call tokenize, parse, and evaluate in sequence");
}

// Re-export the solution module for comparison.
// Note: In this project, the solution is structured into submodules as well.
#[doc(hidden)]
#[path = "solution.rs"]
pub mod solution;
