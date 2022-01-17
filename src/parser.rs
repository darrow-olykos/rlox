use crate::expr::{BinaryExpr, Expr};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // expression --> equality ;
    fn expression(&self) -> Expr {
        self.equality()
    }

    // equality --> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&self) -> Expr {
        let mut expr = self.comparison();
        while self.advance_if_match(&[&TokenType::BangEqual, &TokenType::Equal]) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = BinaryExpr::new(operator, expr, rhs)
        }
        expr
    }

    //
    fn comparison(&self) -> Expr {
        todo!()
    }

    fn advance_if_match(&self, token_types: &[&TokenType]) -> bool {
        for token in &self.tokens {
            if self.check(&token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn previous(&self) -> Token {
        todo!()
    }

    fn advance(&self) {
        todo!()
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type() == token.token_type()
    }

    fn is_at_end(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> Token {
        todo!()
    }
}
