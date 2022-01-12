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
        visitor.visit(self)
    }
}

/**
 * Any operation that can be performed on Expressions will impl ExprVisitor
 *   Matching on Expr will force implementer to implement a match arm for every Expr variant that exists.
 *   Adding new Expr variances will convenient raise syntax errors in existing implementations that do not provide match arms for those Expr variants.
 */
pub(crate) trait ExprVisitor<T> {
    fn visit(&self, expr: &Expr) -> T;
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
