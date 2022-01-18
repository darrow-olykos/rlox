use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // expression --> equality ;
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality --> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.advance_if_match(&[&TokenType::BangEqual, &TokenType::Equal]) {
            let operator = self.previous().clone();
            let lhs = expr.clone();
            let rhs = self.comparison().clone();
            expr = BinaryExpr::new(operator, lhs, rhs)
        }
        expr
    }

    // comparison --> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.advance_if_match(&[
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let lhs = expr.clone();
            let rhs = self.term().clone();
            expr = BinaryExpr::new(operator, lhs, rhs)
        }
        expr
    }

    fn advance_if_match(&mut self, token_types: &[&TokenType]) -> bool {
        for token in &self.tokens {
            if self.check(&token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type() == token.token_type()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == &TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.advance_if_match(&[&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous().clone();
            let lhs = expr.clone();
            let rhs = self.factor().clone();
            expr = BinaryExpr::new(operator, lhs, rhs);
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.advance_if_match(&[&TokenType::Slash, &TokenType::Star]) {
            let operator = self.previous().clone();
            let lhs = expr.clone();
            let rhs = self.unary().clone();
            expr = BinaryExpr::new(operator, lhs, rhs);
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.advance_if_match(&[&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();
            let rhs = self.unary().clone();
            return UnaryExpr::new(operator, rhs);
        }
        self.primary()
    }

    // primary --> NUMBER | STRING | "true" | "false" | "nil"
    //             | "(" expression ")" ;
    fn primary(&mut self) -> Expr {
        if self.advance_if_match(&[&TokenType::False]) {
            LiteralExpr::new(LiteralExpr::Bool(false))
        } else if self.advance_if_match(&[&TokenType::True]) {
            LiteralExpr::new(LiteralExpr::Bool(true))
        } else if self.advance_if_match(&[&TokenType::Nil]) {
            LiteralExpr::new(LiteralExpr::Nil)
        } else if self.advance_if_match(&[&TokenType::Number, &TokenType::String]) {
            let prev = self.previous();
            match prev.literal() {
                Some(v) => match v {
                    Literal::String(s) => LiteralExpr::new(LiteralExpr::String(s.to_string())),
                    Literal::Float(f) => LiteralExpr::new(LiteralExpr::Float(*f)),
                },
                None => panic!("Something went wrong. Literal should exist for any Number or String token"), // TODO: design this potential bug away
            }
        } else if self.advance_if_match(&[&TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            GroupingExpr::new(expr)
        }
        else {
            panic!("Something went wrong. Unable to parse Primary expression.")
        }
    }

    fn consume(&self, token_type: TokenType, msg: &str) {
        todo!()
    }
}
