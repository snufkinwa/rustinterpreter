use crate::interpreter::enviroment::Environment;
use crate::parser::{Expr, Stmt, Literal};
use crate::interpreter::object::Object;
use crate::token::token::{Token, TokenType};
use crate::interpreter::runtime_error::InterpreterError;
use crate::interpreter::expr::ExprVisitor;
use crate::interpreter::stmt::StmtVisitor;

pub struct Interpreter {
    environment: Environment,
    evaluate_mode: bool,
}

impl Interpreter {
    pub fn new(evaluate_mode: bool) -> Self {
        Interpreter {
            environment: Environment::new(),
            evaluate_mode
        }
    }

    /// Interpret the program by executing each statement.
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), InterpreterError> {
        for statement in statements {
            self.execute(&statement)?;  
        }
        Ok(())
    }

    /// Execute a statement using the visitor pattern.
    fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        stmt.accept(self)  
    }

    /// Evaluate an expression using the visitor pattern.
    fn evaluate(&mut self, expr: &Expr) -> Result<Object, InterpreterError> {
        expr.accept(self)  
    }

    /// Convert an Object to its string representation.
    fn stringify(&self, object: &Object) -> String {
        match object {
            Object::Number(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Nil => "nil".to_string(),
        }
    }
}

/// Implement ExprVisitor for Interpreter
impl ExprVisitor<Result<Object, InterpreterError>> for Interpreter {
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<Object, InterpreterError> {
        match literal {
            Literal::Number(n) => Ok(Object::Number(*n)),
            Literal::String(s) => Ok(Object::String(s.clone())),
            Literal::Bool(b) => Ok(Object::Bool(*b)),
            Literal::Nil => Ok(Object::Nil),
        }
    }

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, InterpreterError> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator.token_type {
            // Handle addition and string concatenation
            TokenType::Plus => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val.clone(), right_val.clone()) {
                    Ok(Object::Number(left_num + right_num))
                } else if let (Object::String(left_str), Object::String(right_str)) = (left_val, right_val) {
                    Ok(Object::String(format!("{}{}", left_str, right_str))) // Handle string concatenation
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle subtraction
            TokenType::Minus => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Ok(Object::Number(left_num - right_num))
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle multiplication
            TokenType::Star => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Ok(Object::Number(left_num * right_num))
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle division
            TokenType::Slash => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    if right_num == 0.0 {
                        Err(InterpreterError::DivisionByZero { line: operator.line })
                    } else {
                        Ok(Object::Number(left_num / right_num))
                    }
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle equality and inequality
            TokenType::Equal_Equal => Ok(Object::Bool(match (&left_val, &right_val) {
                (Object::String(l), Object::String(r)) => l == r,  
                (Object::Number(l), Object::Number(r)) => l == r,  
                (Object::Bool(l), Object::Bool(r)) => l == r,      
                (Object::Nil, Object::Nil) => true,               
                _ => false,                                        
            })),
            TokenType::Bang_Equal => Ok(Object::Bool(match (&left_val, &right_val) {
                (Object::String(l), Object::String(r)) => l != r, 
                (Object::Number(l), Object::Number(r)) => l != r,  
                (Object::Bool(l), Object::Bool(r)) => l != r,      
                (Object::Nil, Object::Nil) => false,               
                _ => true,                                         
            })),
            // Handle greater than
            TokenType::Greater => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Ok(Object::Bool(left_num > right_num))
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle greater than or equal
            TokenType::Greater_Equal => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Ok(Object::Bool(left_num >= right_num))
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle less than
            TokenType::Less => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Ok(Object::Bool(left_num < right_num))
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            // Handle less than or equal
            TokenType::Less_Equal => {
                if let (Object::Number(left_num), Object::Number(right_num)) = (left_val, right_val) {
                    Ok(Object::Bool(left_num <= right_num))
                } else {
                    Err(InterpreterError::InvalidBinaryOperands { line: operator.line })
                }
            }

            _ => Ok(Object::Nil),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, InterpreterError> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, InterpreterError> {
        let right_val = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => {
                if let Object::Number(val) = right_val {
                    Ok(Object::Number(-val))
                } else {
                    Err(InterpreterError::InvalidUnaryOperand { line: operator.line })
                }
            }
            TokenType::Bang => Ok(Object::Bool(!right_val.is_truthy())),  // Negation
            _ => Ok(Object::Nil),
        }
    }
    
    fn visit_variable_expr(&mut self, token: &Token) -> Result<Object, InterpreterError> {
        self.environment
            .get(token)
            .map_err(|_| InterpreterError::UndefinedVariable { name: token.lexeme.clone(), line: token.line })
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, InterpreterError> {
        let value = self.evaluate(value)?;
        self.environment.assign(name, value.clone())
            .map_err(|_| InterpreterError::UndefinedVariable { name: name.lexeme.clone(), line: name.line })?;
        Ok(value)
    }
}

/// Implement StmtVisitor for Interpreter
impl StmtVisitor<Result<(), InterpreterError>> for Interpreter {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), InterpreterError> {
        let value = self.evaluate(expr)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: Option<&Expr>) -> Result<(), InterpreterError> {
        let value = if let Some(expr) = initializer {
            self.evaluate(expr)?
        } else {
            Object::Nil
        };
        self.environment.define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<(), InterpreterError> {
        let value = self.evaluate(expr)?;

        if self.evaluate_mode {
            println!("{}", self.stringify(&value));
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), InterpreterError> {
        let previous_env = self.environment.clone();
        self.environment = Environment::from_enclosing(previous_env.clone());
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));
        self.environment = previous_env;
        result
    }
}
