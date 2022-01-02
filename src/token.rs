use std::fmt::{self, Display};

use crate::error::{RloxError, RloxSyntaxError};

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    //literal: there is no Object type in Rust <--- TODO: handle this
    pub line_number: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}

impl Token {
    pub fn new(c: char, text: String, line_number: usize) -> Result<Self, RloxError> {
        let maybe_token_type = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            _ => None,
        };

        match maybe_token_type {
            Some(t) => Ok(Token {
                token_type: t,
                lexeme: text.to_string(),
                line_number,
            }),
            None => {
                return Err(RloxError::SyntaxError(RloxSyntaxError {
                    line_number,
                    location: "".to_string(),
                    description: "Unexpected character.".to_string(),
                }));
            }
        }
    }
    pub fn new_eof(line_number: usize) -> Self {
        Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line_number: line_number,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
