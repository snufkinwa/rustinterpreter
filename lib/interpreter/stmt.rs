use crate::parser::{Expr, Stmt};
use crate::token::token::Token;

// Statement Visitor Trait
pub trait StmtVisitor<T> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: Option<&Expr>) -> T;
    fn visit_expression_stmt(&mut self, expr: &Expr) -> T;
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;  
}


// Implementing the `accept` method for parser's `Stmt`
impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer.as_ref()),
            Stmt::Expression(expr) => visitor.visit_expression_stmt(expr),
            Stmt::Block(statements) => {

                visitor.visit_block_stmt(statements)
            }
        }
    }
}

