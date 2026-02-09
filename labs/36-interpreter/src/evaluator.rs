//! Evaluator for the interpreter.
//!
//! The evaluator's job is to "walk" the Abstract Syntax Tree (AST) generated
//! by the parser and compute the final value of the expression.

use crate::parser::{BinaryOp, Expr};
use thiserror::Error;

// TODO: Define EvalError enum
// For runtime errors like division by zero.
//
// #[derive(Debug, Error, PartialEq)]
// pub enum EvalError { ... }
#[derive(Debug, Error, PartialEq)]
pub enum EvalError {
    #[error("Division by zero")]
    DivisionByZero,
}


/// Evaluates an AST `Expr` and returns the result.
pub fn evaluate(expr: &Expr) -> Result<f64, EvalError> {
    // TODO: Implement the evaluator.
    // This function will be recursive.
    //
    // Use a `match` statement on the `expr` parameter.
    //
    // - If it's a `Expr::Literal(n)`, just return the number `n`.
    //
    // - If it's a `Expr::Binary { op, left, right }`:
    //   1. Recursively call `evaluate()` on the `left` child.
    //   2. Recursively call `evaluate()` on the `right` child.
    //   3. Use another `match` on the `op` to perform the correct
    //      arithmetic operation.
    //   4. Handle division by zero here! If the denominator is zero,
    //      return an `Err(EvalError::DivisionByZero)`.
    //
    // - If it's a `Expr::Grouping(inner_expr)`, just recursively call
    //   `evaluate()` on the `inner_expr`.
    //
    // - If you implemented unary minus (`Expr::Unary`):
    //   1. Recursively call `evaluate()` on the child expression.
    //   2. Negate the result.
    todo!("Implement the recursive evaluator");
}
