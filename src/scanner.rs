use crate::error::{RloxError, RloxSyntaxError};
use crate::token::{get_keyword_token_type, Literal, Token, TokenType};

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        let mut s = Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        match s.scan_tokens() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        s
    }

    pub(crate) fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn scan_tokens(&mut self) -> Result<(), RloxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        let token = Token::new(TokenType::Eof, "".to_string(), None, self.line)?;
        self.tokens.push(token);
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /*
     * TODO: continue scanning even if error
     */
    fn scan_token(&mut self) -> Result<(), RloxError> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.advance_if_match('=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '=' => match self.advance_if_match('=') {
                true => self.add_token(TokenType::EqualEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '<' => match self.advance_if_match('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.advance_if_match('=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => match self.advance_if_match('/') {
                true => {
                    self.advance_through_end_of_line();
                    Ok(())
                }
                false => self.add_token(TokenType::Slash, None),
            },
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '"' => self.consume_string_literal(),
            _ => match c.is_ascii_digit() {
                true => self.consume_number_literal(),
                false => match c == '_' || c.is_alphabetic() {
                    true => self.consume_identifier(),
                    false => Err(RloxError::SyntaxError(RloxSyntaxError {
                        line_number: self.line,
                        description: "Unexpected character.".to_string(),
                    })),
                },
            },
        }?;
        Ok(())
    }

    fn advance(&mut self) -> char {
        let current_char = *(self
            .source
            .chars()
            .collect::<Vec<_>>()
            .get(self.current)
            .unwrap());
        self.current += 1;
        return current_char;
    }

    fn advance_if_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.chars().nth(self.current).unwrap();
        if c != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        literal: Option<Literal>,
    ) -> Result<(), RloxError> {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            literal,
            self.line,
        )?);
        Ok(())
    }

    fn advance_through_end_of_line(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn peek(&mut self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.source.chars().nth(self.current).unwrap(),
        }
    }

    fn consume_string_literal(&mut self) -> Result<(), RloxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(RloxError::SyntaxError(RloxSyntaxError {
                line_number: self.line,
                description: "Unterminated string.".to_string(),
            }));
        }
        self.advance();
        let value = &self.source[&self.start + 1..&self.current - 1];
        let literal = Some(Literal::String(value.to_string()));
        self.add_token(TokenType::String, literal)?;
        Ok(())
    }

    fn consume_number_literal(&mut self) -> Result<(), RloxError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current]
            .parse::<f32>()
            .unwrap();
        self.add_token(TokenType::Number, Some(Literal::Float(*value)))
    }

    fn peek_next(&self) -> char {
        match self.current + 1 >= self.source.len() {
            true => '\0',
            false => self.source.chars().nth(self.current + 1).unwrap(),
        }
    }

    fn consume_identifier(&mut self) -> Result<(), RloxError> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match get_keyword_token_type(text) {
            Some(t) => self.add_token(t, None),
            None => self.add_token(TokenType::Identifier, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Literal, Token, TokenType};

    use super::Scanner;

    #[test]
    fn given_valid_input() {
        let source = String::from("if(example_var){ print \"hi!\"; }");
        let scanner = Scanner::new(source);
        let received_tokens = scanner.tokens();
        let expected_tokens = &vec![
            Token::new(TokenType::If, "if".to_string(), None, 1).unwrap(),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1).unwrap(),
            Token::new(TokenType::Identifier, "example_var".to_string(), None, 1).unwrap(),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1).unwrap(),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1).unwrap(),
            Token::new(TokenType::Print, "print".to_string(), None, 1).unwrap(),
            Token::new(
                TokenType::String,
                "\"hi!\"".to_string(),
                Some(Literal::String("hi!".to_string())),
                1,
            )
            .unwrap(),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1).unwrap(),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1).unwrap(),
            Token::new(TokenType::Eof, "".to_string(), None, 1).unwrap(),
        ];
        assert_eq!(expected_tokens, received_tokens);
    }
}
