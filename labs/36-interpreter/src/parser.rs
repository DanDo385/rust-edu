//! Parser for the interpreter.
//!
//! The parser takes a sequence of tokens from the lexer and builds an
//! Abstract Syntax Tree (AST). The AST represents the structure and meaning
//! of the program. Our parser will handle operator precedence (multiplication
//! before addition) and parentheses.

use crate::lexer::Token;
use thiserror::Error;

// TODO: Define the AST nodes.
// We need an enum for binary operators and an enum for expressions.
// The `Expr` enum will be recursive.

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum BinaryOp {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
// }
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum Expr {
//     Literal(f64),
//     Binary {
//         op: BinaryOp,
//         left: Box<Expr>,
//         right: Box<Expr>,
//     },
//     Grouping(Box<Expr>),
//     Unary {
//         op: UnaryOp, // You would need a UnaryOp enum
//         expr: Box<Expr>,
//     },
// }
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(f64),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    UnaryMinus(Box<Expr>),
}

// TODO: Define ParseError enum
// For errors like "Unexpected token" or "Unexpected end of input".
//
// #[derive(Debug, Error, PartialEq)]
// pub enum ParseError { ... }
#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Unexpected token")]
    UnexpectedToken,
}


/// A parser that builds an AST from a vector of tokens.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

/// Parses a sequence of tokens into an AST.
pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    // TODO: Implement the parser.
    // This is a classic recursive-descent parser. The general idea is to
    // have a function for each level of precedence in your grammar.
    //
    // Grammar:
    // expression -> term ( ( "+" | "-" ) term )*
    // term       -> factor ( ( "*" | "/" ) factor )*
    // factor     -> NUMBER | "(" expression ")" | "-" factor
    //
    // 1. Create a `Parser` struct to hold the token stream and current position.
    // 2. Implement a method for each grammar rule (e.g., `parse_expression`,
    //    `parse_term`, `parse_factor`).
    // 3. `parse_expression` will be the entry point.
    // 4. These methods will call each other according to the grammar rules.
    //    For example, `parse_expression` will call `parse_term`.
    // 5. Use helper methods like `peek()`, `advance()`, `is_at_end()` to
    //    navigate the token stream.
    todo!("Implement the recursive-descent parser");
}
