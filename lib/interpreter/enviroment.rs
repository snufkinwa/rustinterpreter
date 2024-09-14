use std::collections::HashMap;
use crate::token::token::Token;
use crate::interpreter::object::Object;
use crate::interpreter::runtime_error::InterpreterError;  // Assuming you have InterpreterError defined
use anyhow::Result;

// Define the Environment for variable storage
#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>, 
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    // Create an environment with an enclosing (parent) environment
    pub fn from_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    // Define a new variable
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    // Get the value of a variable, check enclosing environments if necessary
    pub fn get(&self, name: &Token) -> Result<Object, InterpreterError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(ref enclosing) = self.enclosing {
            return enclosing.get(name);
        }
        // Use InterpreterError instead of a String for error handling
        Err(InterpreterError::UndefinedVariable { 
            name: name.lexeme.clone(), 
            line: name.line 
        })
    }

    // Assign a new value to a variable, check enclosing environments if necessary
    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), InterpreterError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        if let Some(ref mut enclosing) = self.enclosing {
            return enclosing.assign(name, value);
        }
        Err(InterpreterError::UndefinedVariable { 
            name: name.lexeme.clone(), 
            line: name.line 
        })
    }

    // Get an ancestor environment by traversing a specified distance
    pub fn ancestor(&self, distance: usize) -> Option<&Environment> {
        let mut environment = self;
        for _ in 0..distance {
            if let Some(ref enclosing) = environment.enclosing {
                environment = enclosing;
            } else {
                return None;
            }
        }
        Some(environment)
    }

    // Get a value from an ancestor environment
    pub fn get_at(&self, distance: usize, name: &str) -> Option<Object> {
        self.ancestor(distance)?.values.get(name).cloned()
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) {
        if let Some(environment) = self.ancestor_mut(distance) {
            environment.values.insert(name.lexeme.clone(), value);
        }
    }
    
    fn ancestor_mut(&mut self, distance: usize) -> Option<&mut Environment> {
        let mut environment = self;
        for _ in 0..distance {
            if let Some(ref mut enclosing) = environment.enclosing {
                environment = enclosing;
            } else {
                return None;
            }
        }
        Some(environment)
    }
}
