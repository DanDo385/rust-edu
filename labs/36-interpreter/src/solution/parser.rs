//! Parser for the interpreter - Solution.
//!
//! This parser implements a "recursive descent" strategy to handle operator
//! precedence. The grammar for our language is defined as follows, from lowest
//! to highest precedence:
//!
//! expression -> term ( ( "+" | "-" ) term )*
//! term       -> factor ( ( "*" | "/" ) factor )*
//! factor     -> NUMBER | "(" expression ")" | "-" factor

use crate::solution::lexer::Token;
use thiserror::Error;

/// An enum for the different binary operators.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// An enum for the different unary operators.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
    Negative,
}

/// The Abstract Syntax Tree (AST) for our language.
///
/// `Box<Expr>` is used for recursive variants to avoid infinite type size.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(f64),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

/// Represents errors that can occur during parsing.
#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Expected a number, found {0:?}")]
    ExpectedNumber(Token),
    #[error("Expected an operator, found {0:?}")]
    ExpectedOperator(Token),
    #[error("Expected a right parenthesis ')'")]
    ExpectedRightParen,
}

/// The parser struct holds the token stream and our position in it.
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

/// Main entry point for parsing.
pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse_expression()
}

impl Parser {
    /// expression -> term ( ( "+" | "-" ) term )*
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = if self.peek_is(Token::Plus) { BinaryOp::Add } else { BinaryOp::Subtract };
                    self.advance(); // Consume the operator
                    let right = self.parse_term()?;
                    expr = Expr::Binary {
                        op,
                        left: Box::new(expr),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// term -> factor ( ( "*" | "/" ) factor )*
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Multiply | Token::Divide => {
                    let op = if self.peek_is(Token::Multiply) { BinaryOp::Multiply } else { BinaryOp::Divide };
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::Binary {
                        op,
                        left: Box::new(expr),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// factor -> NUMBER | "(" expression ")" | "-" factor
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        if let Some(token) = self.advance() {
            match token {
                Token::Number(n) => Ok(Expr::Literal(n)),
                Token::LeftParen => {
                    let expr = self.parse_expression()?;
                    if self.peek_is(Token::RightParen) {
                        self.advance(); // Consume ')'
                        Ok(Expr::Grouping(Box::new(expr)))
                    } else {
                        Err(ParseError::ExpectedRightParen)
                    }
                }
                Token::Minus => {
                    let expr = self.parse_factor()?; // Recursive call for unary minus
                    Ok(Expr::Unary {
                        op: UnaryOp::Negative,
                        expr: Box::new(expr),
                    })
                }
                _ => Err(ParseError::ExpectedNumber(token.clone())),
            }
        } else {
            Err(ParseError::UnexpectedEndOfInput)
        }
    }

    // --- Helper Methods ---

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek_is(&self, expected: Token) -> bool {
        self.peek().map_or(false, |t| *t == expected)
    }
}
