use crate::ast::{Statement, Expression};
use crate::object::{Object, Function};
use crate::environment::Environment; // <--- NEW IMPORT
use crate::ast::BlockStatement;

// Updated Signature: Now takes &mut Environment
pub fn eval_program(program: &crate::ast::Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    
    for statement in &program.statements {
        result = eval_statement(statement, env);
        
        // Unwrapping ReturnValue to stop execution
        if let Object::ReturnValue(val) = result {
            return *val;
        }
    }
    
    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Expression(val) => eval_expression(&val.expression, env),
        Statement::Let(val) => {
            let value = eval_expression(&val.value, env);
            env.set(val.name.value.clone(), value)
        },
        Statement::Return(val) => {
            let value = eval_expression(&val.return_value, env);
            Object::ReturnValue(Box::new(value))
        },
    }
}

fn eval_expression(exp: &Expression, env: &mut Environment) -> Object {
    match exp {
        Expression::IntegerLiteral(i) => Object::Integer(i.value),
        Expression::Boolean(b) => Object::Boolean(b.value), // Ensure AST has Boolean if used, else skip
        Expression::Prefix(p) => {
            let right = eval_expression(&p.right, env);
            eval_prefix_expression(&p.operator, right)
        },
        Expression::Infix(i) => {
            let left = eval_expression(&i.left, env);
            let right = eval_expression(&i.right, env);
            eval_infix_expression(&i.operator, left, right)
        },
        Expression::If(ie) => eval_if_expression(ie, env),
        Expression::Identifier(ident) => {
            match env.get(&ident.value) {
                Some(val) => val,
                None => Object::Null, 
            }
        },
        // NEW: Function Definition
        Expression::Function(fl) => {
            Object::Function(Function {
                parameters: fl.parameters.clone(),
                body: fl.body.clone(),
            })
        },
        // NEW: Function Call
        Expression::Call(c) => {
            let function = eval_expression(&c.function, env);
            
            // 1. Evaluate arguments
            let args = eval_expressions(&c.arguments, env);

            // 2. Apply function
            if let Object::Function(fn_obj) = function {
                return apply_function(fn_obj, args, env);
            } else {
                return Object::Null; // Error: calling non-function
            }
        },
        _ => Object::Null,
    }
}

// --- LOGIC HELPERS ---

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Object::Null, // Unknown operator
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true), // !null == true
        _ => Object::Boolean(false), // !5 == false
    }
}

fn eval_minus_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        _ => Object::Null,
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix_expression(operator, l, r),
        _ => Object::Null, // Type mismatch or unknown types
    }
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        // Comparisons returning Booleans
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

fn eval_if_expression(ie: &crate::ast::IfExpression, env: &mut Environment) -> Object {
    let condition = eval_expression(&ie.condition, env);

    if is_truthy(&condition) {
        return eval_block_statement(&ie.consequence, env);
    } else if let Some(alt) = &ie.alternative {
        return eval_block_statement(alt, env);
    } else {
        return Object::Null;
    }
}

fn eval_block_statement(block: &crate::ast::BlockStatement, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in &block.statements {
        result = eval_statement(statement, env);

        // If we hit a return, we DON'T unwrap it yet. 
        // We pass the "Signal" up to eval_program or the function caller.
        if let Object::ReturnValue(_) = result {
            return result;
        }
    }

    result
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true, // In Nova, integers/objects are "true" (like Ruby)
    }
}

// --- HELPER: Execute the function ---
fn apply_function(fn_obj: Function, args: Vec<Object>, _old_env: &Environment) -> Object {
    // 1. Create a NEW scope for the function execution
    let mut extended_env = Environment::new(); 
    
    // 2. Bind arguments (x=5, y=10) in this new scope
    for (i, param) in fn_obj.parameters.iter().enumerate() {
        extended_env.set(param.value.clone(), args[i].clone());
    }

    // 3. Execute the body
    let evaluated = eval_block_statement(&fn_obj.body, &mut extended_env);

    // 4. Unwrap return value if present
    if let Object::ReturnValue(val) = evaluated {
        return *val;
    }
    evaluated
}

fn eval_expressions(exps: &Vec<Expression>, env: &mut Environment) -> Vec<Object> {
    let mut result = vec![];
    for e in exps {
        result.push(eval_expression(e, env));
    }
    result
}