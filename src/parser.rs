use crate::token::Token;

pub(crate) enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
}

impl Expr {
    pub(crate) fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(e) => visitor.visit_binary_expr(&*e),
            Expr::Grouping(e) => visitor.visit_grouping_expr(&*e),
            Expr::Literal(e) => visitor.visit_literal_expr(&*e),
            Expr::Unary(e) => visitor.visit_unary_expr(&*e),
        }
    }
}

/**
 * Any operation that can be performed on Expressions will impl ExprVisitor
 */
pub(crate) trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> T;
}

pub(crate) struct BinaryExpr {
    operator: Token,
    lhs: Expr,
    rhs: Expr,
}

impl BinaryExpr {
    pub(crate) fn get_operator(&self) -> &Token {
        &self.operator
    }

    pub(crate) fn get_lhs(&self) -> &Expr {
        &self.lhs
    }

    pub(crate) fn get_rhs(&self) -> &Expr {
        &self.rhs
    }
}

pub(crate) struct GroupingExpr {
    expression: Expr,
}

impl GroupingExpr {
    pub(crate) fn get_expression(&self) -> &Expr {
        &self.expression
    }
}

pub(crate) enum LiteralExpr {
    Nil,
    String(String),
    Float(f32),
}

pub(crate) struct UnaryExpr {
    operator: Token,
    rhs: Expr,
}

impl UnaryExpr {
    pub(crate) fn get_operator(&self) -> &Token {
        &self.operator
    }
    pub(crate) fn get_rhs(&self) -> &Expr {
        &self.rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast_printer::AstPrinter,
        parser::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
        token::{Token, TokenType},
    };

    #[test]
    fn given_valid_input() {
        // -123 * (45.67)
        let expr: Expr = Expr::Binary(Box::new(BinaryExpr {
            lhs: Expr::Unary(Box::new(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1).unwrap(),
                rhs: Expr::Literal(Box::new(LiteralExpr::Float(123.0))),
            })),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1).unwrap(),
            rhs: Expr::Grouping(Box::new(GroupingExpr {
                expression: Expr::Literal(Box::new(LiteralExpr::Float(45.67))),
            })),
        }));

        let ast_printer = AstPrinter {};

        assert_eq!(ast_printer.print(expr), "(* (- 123) (group 45.67))")
    }
}
