use crate::parser::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr};

struct AstPrinter {}

impl AstPrinter {
    fn print(&self, expr: Expr) -> String {
        expr.accept::<String>(self).to_string()
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        parenthesize(
            &expr.get_operator().get_lexeme(),
            &[expr.get_lhs(), expr.get_rhs()],
        )
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        parenthesize("group", &[&expr.get_expression()])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match expr {
            LiteralExpr::Nil => "nil".to_string(),
            LiteralExpr::String(s) => s.to_string(),
            LiteralExpr::Float(f) => f.to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        parenthesize(expr.get_operator().get_lexeme(), &[&expr.get_rhs()])
    }
}

fn parenthesize(token: &str, expressions: &[&Expr]) -> String {
    todo!()
}
