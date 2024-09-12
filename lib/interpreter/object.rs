#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn as_number(&self) -> Result<f64, String> {
        match self {
            Object::Number(n) => Ok(*n),
            _ => Err("Operand must be a number.".to_string()),
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}
