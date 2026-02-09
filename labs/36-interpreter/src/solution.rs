//! # Solution: A Tree-Walking Interpreter
//!
//! This reference implementation shows a full expression interpreter pipeline:
//! 1. Lexing (turn source text into tokens)
//! 2. Parsing (turn tokens into an AST)
//! 3. Evaluating (walk the AST to produce a numeric result)
//!
//! ## Classroom Narrative
//! 1. **Memory flow**: Source text lives on the stack as `&str`. The lexer borrows it, producing `Vec<Token>` (heap). The parser owns those tokens and produces `Expr` nodes (heap via `Box`). Evaluator walks the AST using stack references (`&Expr`).
//! 2. **Ownership beats GC**: Every AST node is boxed once and owned by its parent. When an `Expr` drops, Rust recursively drops its children, freeing both stack and heap without a GC.
//! 3. **Borrow checker proofs**: Lexer and parser borrow slices (`&Token`), but no mutable aliasing occurs—they own their data or borrow immutably. Evaluation borrows `&Expr` because we never mutate the AST.
//!
//! ### Symbol Drill
//! - `&str` inputs are shared borrows; we pass an address to the lexer without copying the string.
//! - `Box<Expr>` allocates the AST node on the heap; `*` is used implicitly when pattern matching on `Box`, but it just dereferences to the inner value for matching.
//! - `*` also appears in parser expressions (multiplication) and is value arithmetic, not pointer dereference.
//!
//! ## Step-by-step Teaching Breakdown
//! 1. **Lexing**: `tokenize` iterates `chars()` and builds tokens. Numbers are parsed by collecting digits into a `String` (heap) before converting to `f64`. The result `Vec<Token>` owns its contents on the heap.
//! 2. **Parsing**: `Parser` owns the token vector. Each recursive call consumes tokens (`advance()`), moving ownership while those tokens live on the heap.
//! 3. **Evaluation**: AST nodes are borrowed (`&Expr`) to compute results. Binary operations evaluate both sides recursively and combine them, demonstrating how `Box` stores heap nodes and the stack tracks recursive evaluation state.

use thiserror::Error;

pub mod lexer {
    //! Lexer: converts source text into tokens.

    use thiserror::Error;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        Number(f64),
        Plus,
        Minus,
        Multiply,
        Divide,
        LeftParen,
        RightParen,
    }

    #[derive(Debug, Error, PartialEq)]
    pub enum LexerError {
        #[error("Unexpected character: {0}")]
        UnexpectedCharacter(char),
        #[error("Invalid number literal: {0}")]
        InvalidNumber(String),
    }

    pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                '0'..='9' | '.' => {
                    let mut number = String::new();
                    let mut dot_count = 0_usize;

                    while let Some(&next) = chars.peek() {
                        if next.is_ascii_digit() {
                            number.push(next);
                            chars.next();
                        } else if next == '.' {
                            dot_count += 1;
                            if dot_count > 1 {
                                return Err(LexerError::InvalidNumber(number + "."));
                            }
                            number.push(next);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if number == "." {
                        return Err(LexerError::InvalidNumber(number));
                    }

                    let parsed = number
                        .parse::<f64>()
                        .map_err(|_| LexerError::InvalidNumber(number.clone()))?;
                    tokens.push(Token::Number(parsed));
                }
                other => {
                    return Err(LexerError::UnexpectedCharacter(other));
                }
            }
        }

        Ok(tokens)
    }
}

pub mod parser {
    //! Recursive-descent parser with precedence handling.

    use crate::solution::lexer::Token;
    use thiserror::Error;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum BinaryOp {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

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

    #[derive(Debug, Error, PartialEq)]
    pub enum ParseError {
        #[error("Unexpected end of input")]
        UnexpectedEndOfInput,
        #[error("Unexpected token")]
        UnexpectedToken,
        #[error("Expected right parenthesis")]
        ExpectedRightParen,
    }

    pub struct Parser {
        tokens: Vec<Token>,
        pos: usize,
    }

    impl Parser {
        fn new(tokens: Vec<Token>) -> Self {
            Self { tokens, pos: 0 }
        }

        fn is_at_end(&self) -> bool {
            self.pos >= self.tokens.len()
        }

        fn peek(&self) -> Option<&Token> {
            self.tokens.get(self.pos)
        }

        fn advance(&mut self) -> Option<Token> {
            let tok = self.tokens.get(self.pos).cloned();
            if tok.is_some() {
                self.pos += 1;
            }
            tok
        }

        fn parse_expression(&mut self) -> Result<Expr, ParseError> {
            let mut expr = self.parse_term()?;

            loop {
                let op = match self.peek() {
                    Some(Token::Plus) => BinaryOp::Add,
                    Some(Token::Minus) => BinaryOp::Subtract,
                    _ => break,
                };
                self.advance();
                let right = self.parse_term()?;
                expr = Expr::Binary {
                    op,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }

            Ok(expr)
        }

        fn parse_term(&mut self) -> Result<Expr, ParseError> {
            let mut expr = self.parse_factor()?;

            loop {
                let op = match self.peek() {
                    Some(Token::Multiply) => BinaryOp::Multiply,
                    Some(Token::Divide) => BinaryOp::Divide,
                    _ => break,
                };
                self.advance();
                let right = self.parse_factor()?;
                expr = Expr::Binary {
                    op,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }

            Ok(expr)
        }

        fn parse_factor(&mut self) -> Result<Expr, ParseError> {
            match self.advance() {
                Some(Token::Number(n)) => Ok(Expr::Literal(n)),
                Some(Token::Minus) => {
                    let inner = self.parse_factor()?;
                    Ok(Expr::UnaryMinus(Box::new(inner)))
                }
                Some(Token::LeftParen) => {
                    let expr = self.parse_expression()?;
                    match self.advance() {
                        Some(Token::RightParen) => Ok(Expr::Grouping(Box::new(expr))),
                        _ => Err(ParseError::ExpectedRightParen),
                    }
                }
                Some(_) => Err(ParseError::UnexpectedToken),
                None => Err(ParseError::UnexpectedEndOfInput),
            }
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
        let mut parser = Parser::new(tokens);

        if parser.is_at_end() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        let expr = parser.parse_expression()?;

        if parser.is_at_end() {
            Ok(expr)
        } else {
            Err(ParseError::UnexpectedToken)
        }
    }
}

pub mod evaluator {
    //! Evaluator: recursively computes expression values from the AST.

    use crate::solution::parser::{BinaryOp, Expr};
    use thiserror::Error;

    #[derive(Debug, Error, PartialEq)]
    pub enum EvalError {
        #[error("Division by zero")]
        DivisionByZero,
    }

    pub fn evaluate(expr: &Expr) -> Result<f64, EvalError> {
        match expr {
            Expr::Literal(n) => Ok(*n),
            Expr::Grouping(inner) => evaluate(inner),
            Expr::UnaryMinus(inner) => Ok(-evaluate(inner)?),
            Expr::Binary { op, left, right } => {
                let l = evaluate(left)?;
                let r = evaluate(right)?;
                match op {
                    BinaryOp::Add => Ok(l + r),
                    BinaryOp::Subtract => Ok(l - r),
                    BinaryOp::Multiply => Ok(l * r),
                    BinaryOp::Divide => {
                        if r == 0.0 {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(l / r)
                        }
                    }
                }
            }
        }
    }
}

use evaluator::{EvalError, evaluate};
use lexer::{LexerError, tokenize};
use parser::{ParseError, parse};

#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    #[error("Lexer Error: {0}")]
    Lexer(#[from] LexerError),
    #[error("Parser Error: {0}")]
    Parser(#[from] ParseError),
    #[error("Evaluation Error: {0}")]
    Evaluator(#[from] EvalError),
}

pub fn interpret(input: &str) -> Result<f64, InterpreterError> {
    let tokens = tokenize(input)?;
    let ast = parse(tokens)?;
    let result = evaluate(&ast)?;
    Ok(result)
}
