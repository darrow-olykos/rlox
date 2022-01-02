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
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line_number: usize,
    ) -> Result<Self, RloxError> {
        Ok(Token {
            token_type,
            lexeme,
            line_number,
        })
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
