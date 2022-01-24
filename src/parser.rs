use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&self) -> Result<Expr, String> {
        self.expression()
    }

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
            if self.is_current_token_type(&token.token_type()) {
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

    fn is_current_token_type(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type() == token_type
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
            self.consume(&TokenType::RightParen, "Expect ')' after expression.");
            GroupingExpr::new(expr)
        }
        else {
            self.error(self.peek(), "Expect expression.");
            panic!("Something went wrong. Unable to parse Primary expression.")
        }
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> &Token {
        if self.is_current_token_type(token_type) {
            return self.advance()
        }
        else {
            self.error(self.peek(), msg);
            panic!("{} {}", self.peek(), msg);
        }
    }

    fn error(&self, token: &Token, msg: &str) {
        if token.token_type() == &TokenType::Eof {
            println!("{} at end. {}", token.line_number(), msg)
        }
        else {
            println!("{} at '{}' {}", token.line_number(), token.lexeme(), msg)
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == &TokenType::Semicolon {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
                        => return,
                _ => ()
            }

            self.advance();
        }
    }
}
