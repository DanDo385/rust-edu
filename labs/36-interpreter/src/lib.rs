// Lab 36: Expression Interpreter
//
// Builds a simple interpreter for arithmetic expressions. Demonstrates
// tokenization (lexing), parsing to an AST, and recursive evaluation.
// This is the foundation for building programming languages!
//
// Three phases:
//   1. Tokenization: Convert input string into tokens (lexical analysis)
//   2. Parsing: Convert tokens into an AST (syntax analysis)
//   3. Evaluation: Recursively compute the result from the AST

// ============================================================================
// TOKEN TYPES
// ============================================================================
// Tokens are the smallest meaningful units in the language.
// The tokenizer converts raw text into a stream of these.

/// Represents a single token produced by the lexer.
///
/// # Ownership Model
/// Token::Number holds an f64 (Copy type, lives on the stack).
/// The other variants are unit-like -- zero-size at runtime.
/// Vec<Token> owns the tokens; cloning is cheap because all data is Copy-sized.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

// ============================================================================
// ABSTRACT SYNTAX TREE (AST)
// ============================================================================
// The AST represents the hierarchical structure of an expression.
// Box<Expr> is required because Expr is recursive -- without indirection
// the compiler cannot compute a finite size for the type.

/// A node in the expression tree.
///
/// # Why Box?
/// `Expr::BinOp` contains two child `Expr` nodes. Without `Box`, Rust would
/// need infinite stack space (Expr contains Expr contains Expr...).
/// `Box<Expr>` allocates the children on the heap (8 bytes per pointer on
/// 64-bit), breaking the infinite recursion.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A literal numeric value.
    Number(f64),
    /// A binary operation: left op right.
    BinOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

/// Arithmetic operators supported by the interpreter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// ============================================================================
// TOKENIZER (LEXICAL ANALYSIS)
// ============================================================================

/// Converts an input string into a vector of tokens.
///
/// Whitespace is skipped. Numbers (including decimals) are parsed greedily.
/// Unknown characters are silently ignored.
///
/// # Examples
/// ```
/// use interpreter::tokenize;
/// let tokens = tokenize("2 + 3");
/// assert_eq!(tokens.len(), 3);
/// ```
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }

            // Operators and parentheses
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
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

            // Numbers (integer or decimal)
            '0'..='9' | '.' => {
                let mut number = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() || ch == '.' {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(value) = number.parse::<f64>() {
                    tokens.push(Token::Number(value));
                }
            }

            // Unknown character -- skip silently
            _ => {
                chars.next();
            }
        }
    }

    tokens
}

// ============================================================================
// PARSER (SYNTAX ANALYSIS)
// ============================================================================
// Uses recursive descent parsing to convert tokens into an AST.
//
// Grammar (encodes operator precedence):
//   expression  = term ((PLUS | MINUS) term)*
//   term        = factor ((STAR | SLASH) factor)*
//   factor      = NUMBER | LEFT_PAREN expression RIGHT_PAREN
//
// Lower grammar rules = lower precedence.
// `expression` handles +/- (low precedence).
// `term` handles *// (high precedence).
// `factor` handles atoms and grouping (highest precedence).

/// A recursive-descent parser that converts tokens into an AST.
///
/// # Ownership
/// The parser takes ownership of the token vector. `position` is an index
/// into the vector -- no borrowing issues because we own the data.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Creates a new parser from a token stream.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Parses the entire token stream into a single expression AST.
    ///
    /// Returns an error if the input is empty or if tokens remain after
    /// parsing (e.g., an extra closing parenthesis).
    pub fn parse(&mut self) -> Result<Expr, String> {
        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        let expr = self.expression()?;

        // Make sure we consumed all tokens
        if self.position < self.tokens.len() {
            return Err(format!(
                "Unexpected token: {:?}",
                self.tokens[self.position]
            ));
        }

        Ok(expr)
    }

    /// Parse expression: term ((+ | -) term)*
    /// Lower precedence (evaluated last).
    fn expression(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        while self.position < self.tokens.len() {
            let op = match &self.tokens[self.position] {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                _ => break,
            };

            self.position += 1;
            let right = self.term()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse term: factor ((* | /) factor)*
    /// Higher precedence (evaluated first).
    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.factor()?;

        while self.position < self.tokens.len() {
            let op = match &self.tokens[self.position] {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                _ => break,
            };

            self.position += 1;
            let right = self.factor()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse factor: NUMBER | ( expression )
    /// Highest precedence.
    fn factor(&mut self) -> Result<Expr, String> {
        if self.position >= self.tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        match &self.tokens[self.position] {
            Token::Number(n) => {
                let value = *n;
                self.position += 1;
                Ok(Expr::Number(value))
            }

            Token::LeftParen => {
                self.position += 1; // Consume (

                let expr = self.expression()?;

                if self.position >= self.tokens.len() {
                    return Err("Unclosed parenthesis".to_string());
                }

                match &self.tokens[self.position] {
                    Token::RightParen => {
                        self.position += 1; // Consume )
                        Ok(expr)
                    }
                    _ => Err("Expected closing parenthesis".to_string()),
                }
            }

            _ => Err(format!(
                "Unexpected token: {:?}",
                self.tokens[self.position]
            )),
        }
    }
}

// ============================================================================
// EVALUATOR (INTERPRETER)
// ============================================================================
// Recursively walks the AST and computes the numeric result.

/// Evaluates an AST node to produce a numeric result.
///
/// # Errors
/// Returns an error on division by zero.
///
/// # Recursion
/// Each `BinOp` node recurses into its children. Stack depth equals the
/// depth of the AST. Rust's default 2 MB stack handles ~10,000 levels.
pub fn evaluate(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),

        Expr::BinOp { left, op, right } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;

            match op {
                Operator::Add => Ok(left_val + right_val),
                Operator::Subtract => Ok(left_val - right_val),
                Operator::Multiply => Ok(left_val * right_val),
                Operator::Divide => {
                    if right_val == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
            }
        }
    }
}

// ============================================================================
// HIGH-LEVEL INTERPRET FUNCTION
// ============================================================================

/// Interprets an arithmetic expression string and returns the result.
///
/// This is the main entry point that chains tokenize -> parse -> evaluate.
///
/// # Examples
/// ```
/// use interpreter::interpret;
/// assert_eq!(interpret("2 + 3").unwrap(), 5.0);
/// assert_eq!(interpret("2 + 3 * 4").unwrap(), 14.0);
/// assert_eq!(interpret("(2 + 3) * 4").unwrap(), 20.0);
/// ```
///
/// # Errors
/// Returns an error for:
/// - Empty input
/// - Malformed expressions (missing operands, unclosed parens)
/// - Division by zero
pub fn interpret(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    evaluate(&ast)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("2 + 3");
        assert_eq!(tokens, vec![Token::Number(2.0), Token::Plus, Token::Number(3.0)]);
    }

    #[test]
    fn test_tokenize_all_operators() {
        let tokens = tokenize("1 + 2 - 3 * 4 / 5");
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[1], Token::Plus);
        assert_eq!(tokens[3], Token::Minus);
        assert_eq!(tokens[5], Token::Star);
        assert_eq!(tokens[7], Token::Slash);
    }

    #[test]
    fn test_tokenize_parens() {
        let tokens = tokenize("(1 + 2)");
        assert_eq!(tokens[0], Token::LeftParen);
        assert_eq!(tokens[4], Token::RightParen);
    }

    #[test]
    fn test_tokenize_decimal() {
        let tokens = tokenize("3.14");
        assert_eq!(tokens, vec![Token::Number(3.14)]);
    }

    #[test]
    fn test_evaluate_number() {
        let expr = Expr::Number(42.0);
        assert_eq!(evaluate(&expr).unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let expr = Expr::BinOp {
            left: Box::new(Expr::Number(1.0)),
            op: Operator::Divide,
            right: Box::new(Expr::Number(0.0)),
        };
        assert!(evaluate(&expr).is_err());
    }
}
