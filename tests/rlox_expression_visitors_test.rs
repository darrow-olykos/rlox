use std::rc::Rc;

use rlox::ast_printer::AstPrinter;
use rlox::ast_printer_rpn::AstPrinterRpn;
use rlox::parser::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use rlox::token::{Token, TokenType};

#[test]
fn ast_printer_prints_in_lisp_like_format() {
    // -123 * (45.67)
    let expr: Expr = Expr::Binary(BinaryExpr::new(
        Token::new(TokenType::Star, "*".to_string(), None, 1).unwrap(),
        Expr::Unary(UnaryExpr::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1).unwrap(),
            Expr::Literal(LiteralExpr::new(LiteralExpr::Float(123.0))),
        )),
        Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
            LiteralExpr::Float(45.67),
        )))),
    ));

    let printer = AstPrinter::default();

    assert_eq!(printer.print(expr), "(* (- 123) (group 45.67))")
}

#[test]
fn ast_printer_rpn_prints_in_reverse_polish_notation() {
    //  "-123 * (45.67)" which is "123 - 45.67 *" in reverse polish notation
    let expr: Expr = Expr::Binary(BinaryExpr::new(
        Token::new(TokenType::Star, "*".to_string(), None, 1).unwrap(),
        Expr::Unary(UnaryExpr::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1).unwrap(),
            Expr::Literal(LiteralExpr::new(LiteralExpr::Float(123.0))),
        )),
        Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
            LiteralExpr::Float(45.67),
        )))),
    ));

    let printer = AstPrinterRpn::default();

    assert_eq!(printer.print(expr), "123 - 45.67 *");
}

#[ignore]
#[test]
fn ast_printer_rpn_prints_in_reverse_polish_notation_2() {
    //  "(1 + 2) * (4 - 3)" which is "1 2 + 4 3 - *" in reverse polish notation
    todo!()
}
