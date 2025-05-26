mod data_access;
mod interp;

use std::{collections::HashMap, env};

use interp::{ast::MathExpression, interpreter::interpret, parse::expression_p};

fn main() {
    let math_expression: String = env::args().collect::<Vec<_>>()[1..].join(" ");

    if math_expression.is_empty() {
        panic!("Requires a math expression!")
    }

    let math_ast: MathExpression = expression_p(&math_expression)
        .unwrap_or_else(|err| panic!("Failed to parse expression with following error: {err}"));

    let environment = env::vars().fold(HashMap::<String, f64>::new(), |mut acc, (key, value)| {
        if let Ok(value) = value.parse::<f64>() {
            acc.insert(key, value);
        };
        acc
    });

    match interpret(math_ast, &environment) {
        Ok(result) => println!("{}", result),
        Err(err) => match err {
            interp::interpreter::error::InterpreterError::UnboundVariable { variable_name } => {
                panic!("Missing environment variable {variable_name}");
            }
            interp::interpreter::error::InterpreterError::UnknownFunction { function, args } => {
                let args = args.as_slice();
                panic!("Unknown function {function} with args {args:?}");
            }
            interp::interpreter::error::InterpreterError::FunctionArityMismatch {
                function,
                arity: _,
            } => panic!("Function {function} given wrong number of of arguments"),
            interp::interpreter::error::InterpreterError::DivideByZero {
                dividend: _,
                divisor: _,
            } => {
                panic!("Cannot divide by zero!");
            }
            interp::interpreter::error::InterpreterError::Undefined => panic!("undefined"),
        },
    }
}
