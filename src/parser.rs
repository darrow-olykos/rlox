use crate::token::Token;

enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
}

struct BinaryExpr {
    operator: Token,
    lhs: Expr,
    rhs: Expr,
}

struct GroupingExpr {
    operator: Token,
}

struct LiteralExpr {
    value: String,
}

struct UnaryExpr {
    operator: Token,
}

trait ExprVisitor {
    fn visit(&self, expr: Expr) {
        match expr {
            Expr::Binary(e) => self.visitBinaryExpr(e),
            Expr::Grouping(e) => self.visitGroupingExpr(e),
            Expr::Literal(e) => self.visitLiteralExpr(e),
            Expr::Unary(e) => self.visitUnaryExpr(e),
        }
    }

    fn visitBinaryExpr(&self, expr: Box<BinaryExpr>) {
        todo!()
    }

    fn visitGroupingExpr(&self, expr: Box<GroupingExpr>) {
        todo!()
    }

    fn visitLiteralExpr(&self, expr: Box<LiteralExpr>) {
        todo!()
    }

    fn visitUnaryExpr(&self, expr: Box<UnaryExpr>) {
        todo!()
    }
}
