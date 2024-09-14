use crate::parser::{Expr, Literal};  // Import the Expr from parser
use crate::token::token::Token;

// Expression Visitor Trait
pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable_expr(&mut self, token: &Token) -> T;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> T;
}

// Implement the `accept` method for `Expr` using the visitor pattern
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Literal(lit) => visitor.visit_literal_expr(lit),
            Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            Expr::Variable(token) => visitor.visit_variable_expr(token),
            Expr::Assign(name, value) => visitor.visit_assign_expr(name, value),
        }
    }
}


