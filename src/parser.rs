use std::rc::Rc;

use crate::token::{Literal, Token};

pub enum Expr {
    Binary(Rc<BinaryExpr>),
    Grouping(Rc<GroupingExpr>),
    Literal(Rc<LiteralExpr>),
    Unary(Rc<UnaryExpr>),
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

pub struct BinaryExpr {
    operator: Token,
    lhs: Expr,
    rhs: Expr,
}

impl BinaryExpr {
    pub fn new(operator: Token, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary(Rc::new(BinaryExpr { operator, lhs, rhs }))
    }

    pub(crate) fn operator(&self) -> &Token {
        &self.operator
    }

    pub(crate) fn lhs(&self) -> &Expr {
        &self.lhs
    }

    pub(crate) fn rhs(&self) -> &Expr {
        &self.rhs
    }
}

pub struct GroupingExpr {
    expression: Expr,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Expr {
        Expr::Grouping(Rc::new(GroupingExpr { expression }))
    }

    pub(crate) fn expression(&self) -> &Expr {
        &self.expression
    }
}

pub enum LiteralExpr {
    Nil,
    String(String),
    Float(f32),
}

impl LiteralExpr {
    pub fn new(e: LiteralExpr) -> Expr {
        Expr::Literal(Rc::new(e))
    }
}

pub struct UnaryExpr {
    operator: Token,
    rhs: Expr,
}

impl UnaryExpr {
    pub fn new(operator: Token, rhs: Expr) -> Expr {
        Expr::Unary(Rc::new(UnaryExpr { operator, rhs }))
    }

    pub(crate) fn operator(&self) -> &Token {
        &self.operator
    }
    pub(crate) fn rhs(&self) -> &Expr {
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
    fn given_valid_input() {}
}
