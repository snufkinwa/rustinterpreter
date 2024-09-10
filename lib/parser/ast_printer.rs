use crate::parser::{Expr, Stmt};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(left, operator, right) => {
                format!("({} {} {})", operator.lexeme, self.print_expr(left), self.print_expr(right))
            }
            Expr::Grouping(expr) => format!("(group {})", self.print_expr(expr)),
            Expr::Literal(value) => {
                match value {
                    Some(v) => {
                        if v.starts_with('"') && v.ends_with('"') {
                            v[1..v.len()-1].to_string()
                        } else if let Ok(num) = v.parse::<f64>() {
                            if v.contains('.') {
                                v.to_string()
                            } else {
                                format!("{:.1}", num)
                            }
                        } else {
                            v.to_string()
                        }
                    },
                    None => "nil".to_string(),
                }
            },
            Expr::Variable(token) => token.lexeme.clone(),
            Expr::Assign(name, value) => format!("(assign {} {})", self.print_expr(name), self.print_expr(value)),
            Expr::Unary(operator, expr) => {
                format!("({} {})", operator.lexeme, self.print_expr(expr))
            }
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