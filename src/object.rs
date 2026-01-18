use std::fmt;
use crate::ast::{Identifier, BlockStatement}; // Import AST nodes

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>), // Wraps a value to signal "Stop!"
    Function(Function),       // The executable function
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    // Note: In a production compiler, we would store the 'Environment' here 
    // to support Closures (accessing outer variables). 
    // We are skipping that for v0.1 to keep the Rust code simple.
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(val) => write!(f, "{}", val),
            Object::Function(fun) => {
                let params: Vec<String> = fun.parameters.iter().map(|p| p.value.clone()).collect();
                write!(f, "fn({}) {{ ... }}", params.join(", "))
            },
        }
    }
}