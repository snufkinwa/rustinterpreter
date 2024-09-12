use crate::parser::parser::{Expr, Stmt, Literal};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(left, operator, right) => {
                format!("({} {} {})", operator.lexeme, self.print_expr(left), self.print_expr(right))
            }
            Expr::Grouping(expr) => format!("(group {})", self.print_expr(expr)),
            Expr::Literal(literal) => self.print_literal(literal),
            Expr::Variable(token) => token.lexeme.clone(),
            Expr::Assign(name, value) => {
                format!("(assign {} {})", name.lexeme, self.print_expr(value))
            }
            Expr::Unary(operator, expr) => {
                format!("({} {})", operator.lexeme, self.print_expr(expr))
            }
        }
    }

    fn print_literal(&self, literal: &Literal) -> String {
        match literal {
            Literal::String(s) => s.clone(),
            Literal::Number(n) => format_number_test!(n),
            Literal::Bool(b) => format!("{}", b),
            Literal::Nil => "nil".to_string(),
        }
    }

    pub fn print_stmt(&self, stmt: &Stmt) -> String {
        match stmt {
            Stmt::Expression(expr) => self.print_expr(expr),
            Stmt::Print(expr) => format!("(print {})", self.print_expr(expr)),
            Stmt::Var { name, initializer } => {
                let init = initializer
                    .as_ref()
                    .map(|init| self.print_expr(init))
                    .unwrap_or_else(|| "nil".to_string());
                format!("(var {} {})", name.lexeme, init)
            }
            Stmt::Block(statements) => {
                let mut result = String::from("(block");
                for statement in statements {
                    result.push_str(&format!(" {}", self.print_stmt(statement)));
                }
                result.push(')');
                result
            }
        }
    }
}
