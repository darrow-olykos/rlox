use crate::parser::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept::<String>(self).to_string()
    }

    pub fn default() -> Self {
        AstPrinter {}
    }

    fn parenthesize(&self, name: &str, expressions: &[&Expr]) -> String {
        let mut s = format!("({}", name);
        for expr in expressions {
            s.push_str(" ");
            s.push_str(&expr.accept::<String>(self));
        }
        s.push_str(")");

        return s.to_string();
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator().lexeme(), &[expr.lhs(), expr.rhs()])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        self.parenthesize("group", &[&expr.expression()])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match expr {
            LiteralExpr::Nil => "nil".to_string(),
            LiteralExpr::String(s) => s.to_string(),
            LiteralExpr::Float(f) => f.to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        self.parenthesize(expr.operator().lexeme(), &[&expr.rhs()])
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(expr) => self.visit_binary_expr(&expr),
            Expr::Grouping(expr) => self.visit_grouping_expr(&expr),
            Expr::Literal(expr) => self.visit_literal_expr(&expr),
            Expr::Unary(expr) => self.visit_unary_expr(&expr),
        }
    }
}
