//! # A Tree-Walking Interpreter - Complete Solution
//!
//! This module contains the complete solution for the interpreter, broken down
//! into three main parts: a lexer, a parser, and an evaluator.

pub mod evaluator;
pub mod lexer;
pub mod parser;

use thiserror::Error;
use lexer::{LexerError, tokenize};
use parser::{ParseError, parse};
use evaluator::{EvalError, evaluate};

/// A top-level error type that encapsulates all possible failures.
///
/// Using `thiserror`, we can easily create a single error type that can
/// represent errors from any of the three stages of interpretation.
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
/// This function orchestrates the three main phases of interpretation.
pub fn interpret(input: &str) -> Result<f64, InterpreterError> {
    // 1. Tokenize the input string. The `?` operator will convert a
    //    `LexerError` into an `InterpreterError` and return early if it fails.
    let tokens = tokenize(input)?;

    // 2. Parse the token stream into an AST. The `?` operator handles
    //    `ParseError` conversion.
    let ast = parse(tokens)?;

    // 3. Evaluate the AST. The `?` operator handles `EvalError` conversion.
    let result = evaluate(&ast)?;

    Ok(result)
}
