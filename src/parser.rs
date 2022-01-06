use crate::token::Token;

pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
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
pub trait ExprVisitor<T> {
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
    pub fn get_operator(&self) -> &Token {
        &self.operator
    }

    pub fn get_lhs(&self) -> &Expr {
        &self.lhs
    }

    pub fn get_rhs(&self) -> &Expr {
        &self.rhs
    }
}

pub struct GroupingExpr {
    expression: Expr,
}

impl GroupingExpr {
    pub fn get_expression(&self) -> &Expr {
        &self.expression
    }
}

pub enum LiteralExpr {
    Nil,
    String(String),
    Float(u32),
}

pub struct UnaryExpr {
    operator: Token,
    rhs: Expr,
}

impl UnaryExpr {
    pub fn get_operator(&self) -> &Token {
        &self.operator
    }
    pub fn get_rhs(&self) -> &Expr {
        &self.rhs
    }
}
