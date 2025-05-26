use std::collections::HashMap;

use error::InterpreterError;

use super::functions::FUNCTIONS;
use super::{
    ast::{self, BinOp},
    functions::Function,
};

pub fn interpret(
    expr: ast::MathExpression,
    environment: &HashMap<String, f64>,
) -> Result<f64, InterpreterError> {
    match expr {
        ast::MathExpression::BinOp { op, lhs, rhs } => {
            let lhs = interpret(*lhs, environment)?;
            let rhs = interpret(*rhs, environment)?;
            match op {
                BinOp::Add => Ok(lhs + rhs),
                BinOp::Subtract => Ok(lhs - rhs),
                BinOp::Multiply => Ok(lhs * rhs),
                BinOp::Divide => {
                    if rhs != 0.0 {
                        Ok(lhs / rhs)
                    } else {
                        Err(InterpreterError::DivideByZero {
                            dividend: lhs,
                            divisor: rhs,
                        })
                    }
                }
                BinOp::Exponent => {
                    if lhs == 0.0 && rhs <= 0.0 {
                        Err(InterpreterError::Undefined)
                    } else {
                        Ok(lhs.powf(rhs))
                    }
                }
                BinOp::Modulus => Ok(lhs % rhs),
            }
        }
        ast::MathExpression::Variable { name } => environment
            .get(&name)
            .ok_or_else(|| InterpreterError::UnboundVariable {
                variable_name: name,
            })
            .copied(),
        ast::MathExpression::Function { name, arguments } => {
            let arguments =
                arguments
                    .into_iter()
                    .try_fold(Vec::<f64>::new(), |mut vec, expression| {
                        let result = interpret(expression, environment)?;
                        vec.push(result);
                        Ok(vec)
                    })?;

            let &Function {
                ref name,
                ref arity,
                ref function,
            } = FUNCTIONS
                .get(name.as_str())
                .ok_or_else(|| InterpreterError::UnknownFunction {
                    function: name,
                    args: arguments.clone(),
                })?;

            if !arity.is_valid(arguments.len()) {
                Err(InterpreterError::FunctionArityMismatch {
                    function: name.clone(),
                    arity: arity.clone(),
                })
            } else {
                Ok(function(&arguments))
            }
        }
        ast::MathExpression::Number(float) => Ok(float),
    }
}

pub mod error {
    use std::error::Error;

    use crate::interp::functions::Arity;

    #[derive(Debug, PartialEq)]
    pub enum InterpreterError {
        UnboundVariable { variable_name: String },
        UnknownFunction { function: String, args: Vec<f64> },
        FunctionArityMismatch { function: String, arity: Arity },
        DivideByZero { dividend: f64, divisor: f64 },
        Undefined,
    }

    impl std::fmt::Display for InterpreterError {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Error for InterpreterError {}
}
