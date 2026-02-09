//! Lexer (or Tokenizer) for the interpreter - Solution.

use thiserror::Error;
use std::iter::Peekable;
use std::str::Chars;

/// Represents a token in our language.
///
/// `PartialEq` is for comparing tokens in tests.
/// `Clone` is needed because the parser will sometimes need to clone tokens.
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

/// Represents an error that can occur during tokenization.
#[derive(Debug, Error, PartialEq)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
}

/// Takes a string and converts it into a sequence of `Token`s.
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    // Use a `peekable` iterator to allow looking at the next character
    // without consuming the current one. This is useful for parsing
    // multi-character tokens like numbers.
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Ignore whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next(); // Consume the whitespace character
            }
            // Numbers
            '0'..='9' | '.' => {
                tokens.push(tokenize_number(&mut chars)?);
            }
            // Operators and Parentheses
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
            // Unrecognized character
            _ => {
                return Err(LexerError::UnexpectedCharacter(c));
            }
        }
    }

    Ok(tokens)
}

/// Helper function to tokenize a number.
///
/// It consumes characters from the iterator as long as they form a valid number.
fn tokenize_number(chars: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) || c == '.' {
            num_str.push(c);
            chars.next();
        } else {
            break;
        }
    }
    // `parse::<f64>()` can fail if the number is malformed (e.g., "1.2.3").
    // For this simple interpreter, we'll treat that as an error, though a
    // more robust lexer might handle it differently.
    match num_str.parse::<f64>() {
        Ok(n) => Ok(Token::Number(n)),
        Err(_) => Err(LexerError::UnexpectedCharacter(num_str.chars().next().unwrap_or(' '))) // Corrected: Removed unnecessary escaping of ' '.
    }
}
