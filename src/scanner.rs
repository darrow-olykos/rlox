use crate::error::{RloxError, RloxSyntaxError};
use crate::token::{Token};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    /*
     * keep trying to scan tokens until end of file is reached,
     * when eof is reached push eof token to the end of tokens collection to make parser a little cleaner,
     * return collected tokens
     */
    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new_eof(self.line));
        &self.tokens
    }

    fn scan_token(&mut self) -> Result<(), RloxError> {
        let c: char = self.advance();
        let text = &self.source[self.start..self.current];
        let token = Token::new(c, text.to_string(), self.line)?;
        Ok(())
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let chars = self.source.chars().collect::<Vec<_>>();
        *chars.get(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }
}
