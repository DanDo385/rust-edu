//! Lexer (or Tokenizer) for the interpreter.
//!
//! The lexer's job is to take a raw string of source code and turn it into
//! a sequence of "tokens". Each token represents a meaningful unit of the
//! language, like a number, an operator, or a parenthesis.

use thiserror::Error;

// TODO: Define the Token enum.
// It should represent all the possible tokens in our language.
//
// #[derive(Debug, PartialEq, Clone)]
// pub enum Token {
//     Number(f64),
//     Plus,
//     Minus,
//     Multiply,
//     Divide,
//     LeftParen,
//     RightParen,
// }
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

// TODO: Define LexerError enum
// It should represent possible errors during tokenization,
// like encountering an unexpected character.
//
// #[derive(Debug, Error, PartialEq)]
// pub enum LexerError { ... }
#[derive(Debug, Error, PartialEq)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Invalid number literal: {0}")]
    InvalidNumber(String),
}


/// Takes a string and converts it into a sequence of tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    // TODO: Implement the tokenizer.
    // 1. Create an empty `Vec<Token>`.
    // 2. Use a `peekable()` iterator over the characters of the input string.
    // 3. Loop through the characters:
    //    - If it's whitespace, ignore it.
    //    - If it's a digit (`'0'`-`'9'`), parse the full number (including decimals)
    //      and push a `Token::Number`.
    //    - If it's an operator (`+`, `-`, `*`, `/`), push the corresponding token.
    //    - If it's a parenthesis, push the corresponding token.
    //    - If it's any other character, return a `LexerError::UnexpectedCharacter`.
    // 4. Return the `Vec<Token>`.
    todo!("Implement the tokenizer");
}
