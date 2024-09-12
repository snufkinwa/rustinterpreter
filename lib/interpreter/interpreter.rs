use crate::interpreter::enviroment::Environment;
use crate::parser::{Expr, Stmt, Literal};  // Using parser's Expr, Stmt, and Literal
use crate::interpreter::object::Object;
use crate::token::token::{Token, TokenType};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    /// Interpret the program by executing each statement.
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                let result = self.evaluate(expr);
                // Print the result of the expression (REPL-like behavior)
                println!("{}", self.stringify(&result));
            }
            Stmt::Print(expr) => {
                self.visit_print_stmt(expr);
            }
            Stmt::Var { name, initializer } => {
                self.visit_var_stmt(name, initializer);
            }
            Stmt::Block(statements) => {
                self.visit_block_stmt(statements);
            }
        }
    }
    

    /// Evaluate an expression and return an Object.
    fn evaluate(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Binary(left, operator, right) => self.visit_binary_expr(*left, operator, *right),
            Expr::Grouping(expression) => self.visit_grouping_expr(*expression),
            Expr::Unary(operator, right) => self.visit_unary_expr(operator, *right),
            Expr::Variable(token) => self.visit_variable_expr(token),
            Expr::Assign(name, value) => self.visit_assign_expr(name, *value),
        }
    }

    // Visit a literal expression.
    fn visit_literal_expr(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::Number(n) => Object::Number(n),
            Literal::String(s) => Object::String(s),
            Literal::Bool(b) => Object::Bool(b),  // Handle booleans
            Literal::Nil => Object::Nil,          // Handle nil
        }
    }

    /// Visit a binary expression and evaluate it.
    fn visit_binary_expr(&mut self, left: Expr, operator: Token, right: Expr) -> Object {
        let left_val = self.evaluate(left);
        let right_val = self.evaluate(right);
    
        match operator.token_type {
            TokenType::Plus => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val.clone(), right_val.clone()) {
                    Object::Number(left_num + right_num)
                } else if let (Object::String(left_str), Object::String(right_str)) = (left_val, right_val) {
                    // Handle string concatenation
                    Object::String(format!("{}{}", left_str, right_str))
                } else {
                    Object::Nil
                }
            }
            TokenType::Minus => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Number(left_num - right_num)
                } else {
                    Object::Nil
                }
            }
            TokenType::Star => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Number(left_num * right_num)
                } else {
                    Object::Nil
                }
            }
            TokenType::Slash => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Number(left_num / right_num)
                } else {
                    Object::Nil
                }
            }
            TokenType::Equal_Equal => Object::Bool(left_val == right_val),  // Handle equality
            TokenType::Bang_Equal => Object::Bool(left_val != right_val),   // Handle inequality
            TokenType::Greater => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Bool(left_num > right_num)
                } else {
                    Object::Nil
                }
            }
            TokenType::Greater_Equal => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Bool(left_num >= right_num)
                } else {
                    Object::Nil
                }
            }
            TokenType::Less => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Bool(left_num < right_num)
                } else {
                    Object::Nil
                }
            }
            TokenType::Less_Equal => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Object::Bool(left_num <= right_num)
                } else {
                    Object::Nil
                }
            }
            _ => Object::Nil,
        }
    }

    /// Visit a grouping expression.
    fn visit_grouping_expr(&mut self, expr: Expr) -> Object {
        self.evaluate(expr)
    }

    /// Visit a unary expression.
    fn visit_unary_expr(&mut self, operator: Token, right: Expr) -> Object {
        let right_val = self.evaluate(right);

        match operator.token_type {
            TokenType::Minus => {
                if let Object::Number(val) = right_val {
                    Object::Number(-val)
                } else {
                    Object::Nil
                }
            }
            TokenType::Bang => Object::Bool(!right_val.is_truthy()),  // Negate truthy value
            _ => Object::Nil,
        }
    }

    /// Visit a variable expression (lookup in environment).
    fn visit_variable_expr(&mut self, token: Token) -> Object {
        self.environment.get(&token).unwrap_or(Object::Nil)
    }

    /// Visit an assignment expression.
    fn visit_assign_expr(&mut self, name: Token, value: Expr) -> Object {
        let value = self.evaluate(value);
        self.environment.assign(&name, value.clone()).unwrap();
        value
    }

    /// Visit a print statement and output the result.
    fn visit_print_stmt(&mut self, expr: Expr) {
        let value = self.evaluate(expr);
        eprintln!("Debug: Evaluated value: {:?}", value);
        println!("{}", self.stringify(&value));  
    }

    /// Visit a variable declaration statement.
    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) {
        let value = if let Some(init_expr) = initializer {
            self.evaluate(init_expr)
        } else {
            Object::Nil
        };
        self.environment.define(name.lexeme, value);  // Define the variable
    }

    /// Visit a block statement (new scope).
    fn visit_block_stmt(&mut self, statements: Vec<Stmt>) {
        let previous_env = self.environment.clone();
        self.environment = Environment::from_enclosing(previous_env);
        for stmt in statements {
            self.execute(stmt);
        }
        // Restore the previous environment after the block execution
        //self.environment = self.environment.enclosing().unwrap_or_else(|| Environment::new());
    }

    /// Convert an Object to its string representation.
    fn stringify(&self, object: &Object) -> String {
        match object {
            Object::Number(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),  // Print booleans as "true" or "false"
            Object::String(s) => s.clone(),
            Object::Nil => "nil".to_string(),  // Print "nil" for nil objects
        }
    }
}
