//! Evaluator for the interpreter - Solution.
//!
//! The evaluator takes an Abstract Syntax Tree (AST) and computes the final
//! value. It does this by "walking" the tree recursively.

use crate::solution::parser::{BinaryOp, Expr, UnaryOp};
use thiserror::Error;

/// Represents errors that can occur during evaluation.
#[derive(Debug, Error, PartialEq)]
pub enum EvalError {
    #[error("Division by zero")]
    DivisionByZero,
}

/// Evaluates an AST `Expr` and returns the `f64` result.
///
/// This function is recursive. It computes the value of the children of a node
/// first, then combines them based on the node's operator.
pub fn evaluate(expr: &Expr) -> Result<f64, EvalError> {
    match expr {
        // Base case: If the expression is just a number, that's the result.
        Expr::Literal(n) => Ok(*n),

        // Recursive case: A binary operation.
        Expr::Binary { op, left, right } => {
            // 1. Evaluate the left-hand side of the operation.
            let left_val = evaluate(left)?;
            // 2. Evaluate the right-hand side.
            let right_val = evaluate(right)?;

            // 3. Perform the operation.
            match op {
                BinaryOp::Add => Ok(left_val + right_val),
                BinaryOp::Subtract => Ok(left_val - right_val),
                BinaryOp::Multiply => Ok(left_val * right_val),
                BinaryOp::Divide => {
                    if right_val == 0.0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(left_val / right_val)
                    }
                }
            }
        }

        // Recursive case: A unary operation (like negation).
        Expr::Unary { op, expr } => {
            // 1. Evaluate the inner expression.
            let val = evaluate(expr)?;
            // 2. Apply the unary operator.
            match op {
                UnaryOp::Negative => Ok(-val),
            }
        }

        // Recursive case: A grouping. Just evaluate the inner expression.
        Expr::Grouping(expr) => evaluate(expr),
    }
}
