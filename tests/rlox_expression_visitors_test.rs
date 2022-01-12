use std::rc::Rc;

use rlox::ast_printer::AstPrinter;
use rlox::ast_printer_rpn::AstPrinterRpn;
use rlox::parser::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use rlox::token::{Token, TokenType};

#[test]
fn ast_printer_prints_in_lisp_like_format() {
    // -123 * (45.67)
    let expr = BinaryExpr::new(
        Token::new(TokenType::Star, "*".to_string(), None, 1).unwrap(),
        UnaryExpr::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1).unwrap(),
            LiteralExpr::new(LiteralExpr::Float(123.0)),
        ),
        GroupingExpr::new(LiteralExpr::new(LiteralExpr::Float(45.67))),
    );

    let printer = AstPrinter::default();

    assert_eq!(printer.print(expr), "(* (- 123) (group 45.67))")
}

#[test]
fn ast_printer_rpn_prints_in_reverse_polish_notation() {
    //  "-123 * (45.67)" which is "123 - 45.67 *" in reverse polish notation
    let expr = BinaryExpr::new(
        Token::new(TokenType::Star, "*".to_string(), None, 1).unwrap(),
        UnaryExpr::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1).unwrap(),
            LiteralExpr::new(LiteralExpr::Float(123.0)),
        ),
        GroupingExpr::new(LiteralExpr::new(LiteralExpr::Float(45.67))),
    );

    let printer = AstPrinterRpn::default();

    assert_eq!(printer.print(expr), "123 - 45.67 *");
}

#[test]
fn ast_printer_rpn_prints_in_reverse_polish_notation_2() {
    //  "(1 + 2) * (4 - 3)" is "1 2 + 4 3 - *" in reverse polish notation
    let expr = BinaryExpr::new(
        Token::new(TokenType::Star, "*".to_string(), None, 1).unwrap(),
        GroupingExpr::new(
        BinaryExpr::new(
            Token::new(TokenType::Plus, "+".to_string(), None, 1).unwrap(),
            LiteralExpr::new(LiteralExpr::Float(1.0)),
            LiteralExpr::new(LiteralExpr::Float(2.0))
        )),
        GroupingExpr::new(
            BinaryExpr::new(
                Token::new(TokenType::Minus, "-".to_string(), None, 1).unwrap(),
                LiteralExpr::new(LiteralExpr::Float(4.0)),
                LiteralExpr::new(LiteralExpr::Float(3.0))
            ))
        );

        let printer = AstPrinterRpn::default();

        assert_eq!(printer.print(expr), "1 2 + 4 3 - *")
}
