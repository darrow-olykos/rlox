use crate::parser::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr};

pub struct AstPrinterRpn {}

impl AstPrinterRpn {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept::<String>(self).to_string()
    }

    pub fn default() -> Self {
        AstPrinterRpn {}
    }

    // reverse polish notation
    fn format_in_rpn(&self, name: &str, expressions: &[&Expr]) -> String {
        let mut s = String::from("");
        for expr in expressions {
            s.push_str(&expr.accept::<String>(self));
            s.push_str(" ");
        }
        s.push_str(name);
        return s.to_string();
    }
}

impl ExprVisitor<String> for AstPrinterRpn {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        self.format_in_rpn(&expr.operator().lexeme(), &[expr.lhs(), expr.rhs()])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        expr.expression().accept::<String>(self) // Don't format GroupingExpr, just visit the contained expr
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match expr {
            LiteralExpr::Nil => "nil".to_string(),
            LiteralExpr::String(s) => s.to_string(),
            LiteralExpr::Float(f) => f.to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        self.format_in_rpn(expr.operator().lexeme(), &[&expr.rhs()])
    }
}